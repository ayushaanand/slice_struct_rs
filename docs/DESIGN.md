# Internal Design & Memory Layout

This document details the low-level architecture, memory layout, and soundness guarantees of `slice_struct`. It serves as a reference for contributors and those curious about advanced Rust unsafe memory patterns. The magic behind `slice_struct` boils down to bypassing strict Rust compiler limitations using a sequence of highly advanced unsafe memory tricks.

## 1. The Core Trick: The "Fake" DST Tail

Rust fundamentally enforces that a struct can only have **one** dynamically sized field (DST), and it **must** be at the very end. If a user asks for `payload: [u8]` and `tags: [u16]`, the compiler natively rejects it. 

We bypass this by lying to the compiler. We generate a struct where the tail is just a massive, opaque byte buffer: `__data_tail: [core::mem::MaybeUninit<u8>]`. To the compiler, this perfectly satisfies the "one DST at the end" rule. 

Behind the scenes, we take that single byte buffer and manually partition it at runtime into multiple typed slices using `std::alloc::Layout` offset math.

### The Generated Struct

When a user defines:
```rust
#[slice_struct]
struct Packet {
    id: u32,
    #[slice] payload: [u8],
    #[slice] tags: [u16],
}
```

The macro generates a struct that looks roughly like this under the hood:
```rust
#[repr(C)]
struct Packet {
    // 1. User's sized fields
    __id: u32,
    
    // 2. State & Alignment enforcers
    __payload_state: <[u8] as InlineSlice>::State,
    __align_0: [<[u8] as InlineSlice>::Element; 0],
    __tags_state: <[u16] as InlineSlice>::State,
    __align_1: [<[u16] as InlineSlice>::Element; 0],
    
    // 3. Slice Handles (pointer + length)
    __payload: SliceHandle<u8>,
    __tags: SliceHandle<u16>,
    
    // 4. Pinning guarantee
    __pin: PhantomPinned,
    
    // 5. The native DST tail (raw memory for the slices)
    __data_tail: [core::mem::MaybeUninit<u8>],
}
```

### Memory Layout Diagram

```text
[----------------- SINGLE HEAP ALLOCATION -----------------]
| Sized Prefix                  | Dynamically Sized Tail   |
|-------------------------------|--------------------------|
| id | __payload | __tags | ... | payload [u8] | tags [u16]|
[-------------------------------]--------------------------]
                                ^              ^
         __payload.ptr ---------/              |
            __tags.ptr ------------------------/
```

Because the lengths of the slices are not known until runtime, we compute the footprint safely:
```rust
let layout = Layout::new::<SizedPrefix>();
let (layout, payload_offset) = layout.extend(Layout::array::<u8>(payload_len)).unwrap();
let (layout, tags_offset) = layout.extend(Layout::array::<u16>(tags_len)).unwrap();
let layout = layout.pad_to_align();
```
This guarantees that each slice is placed at an offset that perfectly satisfies its alignment requirements, even if it follows a slice with a completely different alignment.

---

## 2. The Alignment Trick (Heap Corruption Mismatch)

Because we replace the user's slices with a raw `u8` byte buffer, Rust forgets the alignment requirements of the original slices. If a user asked for a `[u64]` (8-byte aligned), but our tail is just `u8` (1-byte aligned), `Box` will eventually deallocate the heap memory with a 1-byte alignment layout. 

Deallocating memory with a layout different from the one used to allocate it causes instant **OS heap corruption**.

To fix this, we generate **zero-length arrays** of the user's types in the sized prefix:
```rust
__align_0: [<[u64] as InlineSlice>::Element; 0],
```
This takes up 0 bytes, but forces the Rust compiler to intrinsically elevate the alignment of the entire struct to match the strictest slice. When `Box` drops, it now perfectly matches the custom allocation layout.

---

## 3. The `InlineSlice` Split (Trait-Based Parsing)

In early versions, the macro had to manually parse and calculate layouts for different types of slices. In `0.3.0`, we shifted that complexity to the trait system. 

If a user writes `#[slice] flags: Mutex<[u32]>`, the compiler automatically queries the `InlineSlice` trait. 
It sees that `State = Mutex<()>` and `Element = u32`. 
The macro simply injects `__flags_state: Mutex<()>` into the sized prefix, and calculates the layout of the raw memory tail based purely on `u32`. The heavy lifting is completely offloaded to standard Rust type resolution!

---

## 4. Safe Disjoint Borrowing (The View API)

Because our struct uses raw pointer offsets and a custom tail, it is inherently `!Unpin` (it cannot be safely moved). Users can only interact with it behind `Pin<&mut Self>`. 

However, Rust considers a method call taking `&mut self` to lock the *entire* struct, making it impossible to mutate `payload` and `tags` at the same time! We solve this by generating a transparent ephemeral struct (e.g. `PacketViewMut`). 

When you call `.view_mut()`, we unsafely unpack the `Pin`, pull out the raw pointers for each slice, construct native references via the `<Type as InlineSlice>::ViewMut<'a>` projection, and hand them to you inside the `PacketViewMut` struct:

```rust
// Macro-generated View struct (inherits the struct's visibility)
pub struct PacketViewMut<'a> {
    pub id: &'a mut u32,
    pub payload: &'a mut [u8],  // Projected from <[u8] as InlineSlice>
    pub tags: &'a mut [u16],    // Projected from <[u16] as InlineSlice>
}
```

Because the Rust compiler natively understands that fields inside a struct don't overlap, it mathematically re-enables simultaneous, disjoint borrowing across all your slices!

---

## 5. `NonNull` Retagging (The Miri Fix)

Miri tracks "Stacked Borrows" to rigorously prevent aliasing. If we accidentally derive a `&mut [T]` from a `&[T]`, Miri flags it as Undefined Behavior (UB), even if we hold a Mutex lock for safe interior mutability! 

We avoided this by storing `NonNull<T>` pointers inside hidden `SliceHandle` fields. When `<Mutex<[T]> as InlineSlice>::project_mut` is called to yield a `SliceMutexGuard`, we pass the raw pointer directly out of the handle without ever constructing an intermediate `&[T]`. This preserves the raw pointer's unique provenance, allowing perfectly safe interior mutability over slices directly inside the struct.

---

## 6. The "Lying Iterator" Buffer Overflow

When constructing a struct using `init_iter`, we take an `ExactSizeIterator` for each slice. Initially, the code read `iter.len()` to calculate the layout size, but then used a standard `.enumerate()` loop to write elements until the iterator returned `None`.

`ExactSizeIterator` is a *safe* trait. Any user can implement it and intentionally return `100` for `.len()`, but actually yield `10_000` items. If the loop kept writing past the end of the heap allocation, it would cause a buffer overflow.

**The Fix:** We completely decoupled the loop boundary from the iterator's internal state. The loop runs exactly `0..iter.len()` times. 
* If the iterator yields fewer items, it hits an `.expect()` and panics immediately, aborting before any UB can occur. 
* If the iterator yields more items, the loop finishes gracefully, and the extra elements are simply ignored.

---

## 7. Panic Unwinding Memory Leaks

If a panic occurs during a write loop (or if an iterator itself panics mid-write), the stack begins unwinding. In early builds, the raw `ptr` allocation was leaked, and worse, any items that *had* been written before the panic were abandoned without having their `Drop` implementations called.

**The Fix:** We introduced `__DropGuard<T>`, a RAII guard similar to `Vec`'s internal `SetLenOnDrop`. 
Before entering the write loop, we initialize the guard with the raw pointer and a tracking counter. As each element is written, the counter increments. 
If a panic occurs, the guard's `Drop` implementation runs and safely executes `ptr::drop_in_place` on exactly the slice of elements that were successfully initialized. If the loop completes successfully, `core::mem::forget(guard)` is called, transferring ownership to the final `Box`.
