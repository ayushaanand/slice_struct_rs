# Internal Design & Memory Layout

This document details the low-level architecture, memory layout, and soundness guarantees of `slice_struct`. It serves as a reference for contributors and those curious about advanced Rust unsafe memory patterns.

## 1. The Core Architecture

The goal of `slice_struct` is to pack multiple dynamically sized arrays (`[T]`) into a single heap allocation alongside fixed-size struct fields. 

Rust natively supports Dynamically Sized Types (DSTs), but **strictly limits them to a single dynamic array at the very end of a struct**. To bypass this limitation, `slice_struct` creates a custom DST where the single native dynamic array is just a raw byte buffer (`[MaybeUninit<u8>]`), and we manually partition that buffer into multiple typed slices.

### The Generated Struct

When a user defines:
```rust
#[slice_struct]
struct Packet {
    id: u32,
    #[slice] payload: u8,
    #[slice] tags: u16,
}
```

The macro generates a struct that looks roughly like this under the hood:
```rust
#[repr(C)]
struct Packet {
    // 1. User's sized fields
    __id: u32,
    
    // 2. Alignment enforcers (0-byte arrays)
    __align_0: [u8; 0],
    __align_1: [u16; 0],
    
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

## 2. Dynamic Layout Calculation

Because the lengths of the slices are not known until runtime, the compiler cannot automatically calculate the size or padding of the struct. We must compute this manually at allocation time.

We use `std::alloc::Layout` to compute the exact memory footprint safely:
```rust
let layout = Layout::new::<SizedPrefix>();
let (layout, payload_offset) = layout.extend(Layout::array::<u8>(payload_len)).unwrap();
let (layout, tags_offset) = layout.extend(Layout::array::<u16>(tags_len)).unwrap();
let layout = layout.pad_to_align();
```
This guarantees that each slice is placed at an offset that perfectly satisfies its alignment requirements, even if it follows a slice with a completely different alignment.

---

## 3. Edge Cases, Soundness, and Bug Post-Mortems

Writing safe wrappers around custom heap allocations involves navigating extreme edge cases. Below are specific soundness holes that were discovered and patched during the development of this crate.

### Case 1: The "Heap Corruption" Alignment Mismatch

**The Bug:** 
When a user placed a highly aligned type inside a slice (e.g., `#[repr(align(64))] struct Aligned(u8)`), the test suite crashed instantly with `STATUS_HEAP_CORRUPTION`.

**The Cause:**
When we allocate the memory, our custom `Layout` math correctly calculates that the pointer needs 64-byte alignment. We request this from the OS and receive a 64-byte aligned pointer.
However, when the `Box<Packet>` is eventually dropped, Rust's `Box` automatically calculates the layout for deallocation based purely on the `Packet` struct definition. Because we stripped the slice fields and replaced them with `SliceHandle` (which only requires 8-byte pointer alignment), Rust instructed the allocator to deallocate the memory with an 8-byte alignment layout.
Deallocating memory with a layout different from the one used to allocate it causes instant heap corruption.

**The Fix:**
We must force the compiler to statically recognize the maximum alignment of all possible slice elements. We do this by injecting zero-length arrays of the slice types directly into the sized portion of the struct:
```rust
__align_0: [Aligned; 0],
```
This occupies 0 bytes of space, but successfully propagates the 64-byte alignment requirement to the struct's intrinsic layout. When `Box` drops, it now perfectly matches the custom allocation layout.

### Case 2: The "Lying Iterator" Buffer Overflow

**The Bug:**
When constructing a struct using `new_box_iter`, the macro takes an `ExactSizeIterator` for each slice. Initially, the code read `iter.len()` to calculate the layout size, but then used a standard `.enumerate()` loop to write elements until the iterator returned `None`.

**The Cause:**
`ExactSizeIterator` is a *safe* trait. Any user can implement it and intentionally return `100` for `.len()`, but yield `0` items, or yield `10_000` items.
* If it yielded fewer items: The loop stopped early, leaving uninitialized memory in the slice. Dropping that memory later caused Undefined Behavior.
* If it yielded more items: The loop kept writing past the end of the heap allocation, causing a buffer overflow and heap corruption.

**The Fix:**
We completely decoupled the loop boundary from the iterator's internal state. The loop runs exactly `0..iter.len()` times. 
* If the iterator yields fewer items, it hits an `.expect()` and panics immediately, aborting before any UB can occur. 
* If the iterator yields more items, the loop finishes gracefully, and the extra elements are simply ignored/dropped safely.

### Case 3: Panic Unwinding Memory Leaks

**The Bug:**
If the aforementioned panic occurs during a write loop (or if the iterator itself panics mid-write), the stack begins unwinding. The raw `ptr` allocation was leaked, and worse, any items that *had* been written before the panic were abandoned without having their `Drop` implementations called.

**The Fix:**
We introduced `__DropGuard<T>`, a RAII guard similar to `Vec`'s internal `SetLenOnDrop`. 
Before entering the write loop, we initialize the guard with the raw pointer and a tracking counter. As each element is written, the counter increments. 
If a panic occurs, the guard's `Drop` implementation runs and safely executes `ptr::drop_in_place` on exactly the slice of elements that were successfully initialized. If the loop completes successfully, `core::mem::forget(guard)` is called, transferring ownership to the final `Box`.

---

## 4. Disjoint Borrowing (The View API)

One of the most complex challenges of `slice_struct` is making the resulting struct ergonomic to use, specifically when it comes to mutating the data. 

### The Problem with `Pin<&mut Self>`

Because the struct is a custom DST with a raw byte tail, it is strictly `!Unpin` (enforced via a hidden `PhantomPinned` field). This means users can never safely obtain a `&mut Packet` reference; they must interact with it via `Pin<&mut Packet>`.

If we simply generated getter and setter methods on the struct, like this:
```rust
impl Packet {
    pub fn payload(self: Pin<&mut Self>) -> &mut [u8] { ... }
    pub fn tags(self: Pin<&mut Self>) -> &mut [u16] { ... }
}
```
We run into a massive usability wall: **Rust's borrow checker considers a method call on `self` to borrow the *entire* struct**. 
If a user calls `p.payload()`, the entire `Pin<&mut Packet>` is locked. They would be completely forbidden from calling `p.tags()` or accessing `p.id` at the same time. This makes standard data manipulation (like reading a tag to decide how to modify the payload) impossible.

### The `ViewMut` Solution (Destructuring the Borrow)

To solve this, `slice_struct` completely hides the actual fields behind `__` prefixes and forces all interaction through a "View API".

When the user calls `.as_mut().view_mut()`, the macro generated method does something very specific:
1. It takes the `Pin<&'a mut Self>`.
2. It uses `unsafe { self.get_unchecked_mut() }` to bypass the `Pin` restriction internally.
3. It creates a temporary, short-lived struct called `PacketViewMut<'a>`.
4. It populates `PacketViewMut` with distinct, independent references to every single field inside the struct.

```rust
// Macro-generated View struct
pub struct PacketViewMut<'a> {
    pub id: &'a mut u32,
    pub payload: SliceBorrow<'a, u8>,
    pub tags: SliceBorrow<'a, u16>,
}
```

Because `PacketViewMut` is just a standard struct with public fields, **the Rust compiler's borrow checker can see inside it.** 
When the user accesses `v.payload` and `v.tags`, the compiler understands they are distinct memory addresses. This perfectly re-enables simultaneous, disjoint borrowing across the entire struct, completely bypassing the opacity of the original `Pin<&mut Self>` method call.

### The Role of `SliceBorrow`

While standard fields get mapped to standard `&'a mut T` references in the View, the slice fields are mapped to a custom `SliceBorrow<'a, T>` guard. 

Internally, the original struct holds a `SliceHandle<T>` (which is just a `NonNull<T>` pointer and a `usize` length). `SliceBorrow` wraps this handle and implements `DerefMut<Target = [T]>`. When the user accesses `v.payload[0] = 99`, the `SliceBorrow` dereferences the raw pointer and reconstructs the `&mut [u8]` slice dynamically. 

Because `SliceBorrow` is tied to the `'a` lifetime of the `ViewMut` struct (which in turn is tied to the mutable borrow of the `Box`), memory safety and exclusivity are perfectly maintained.
