# `slice_struct` 0.3.0 Architecture Plan

This document details the major architectural features planned for the next major release of `slice_struct`. Both features share a common philosophy: **Move complexity out of the macro (codegen) and into the Rust type and trait system.**

---

## Feature 1: Universal Inline Slice Projection

### The Problem
Currently, users can only define slices of raw elements (e.g., `[u8]`). If a user wants to wrap their slice in a synchronization primitive or a validation wrapper (like `str` or `Mutex<[T]>`), they are blocked. 
* Writing `#[slice] payload: Mutex<u8>` creates `&[Mutex<u8>]`, requiring the user to lock every individual byte separately—terrible for performance.
* Writing `#[slice] payload: Mutex<[u8]>` is normally impossible because `Mutex<[u8]>` is a Dynamically Sized Type (DST), and packing complex DSTs with opaque layouts into a struct tail is fundamentally broken.

### The Solution: The `InlineSlice` Trait
We will decouple the "State" of a wrapper from its "Data". The macro will stop assuming slice fields are raw arrays, and instead treat every slice type as a generic `T` that implements an `InlineSlice` trait.

```rust
pub trait InlineSlice {
    type Element;
    type State;
    type View<'a> where Self: 'a;

    fn init_state() -> Self::State;
    fn project<'a>(state: &'a Self::State, data: &'a [Self::Element]) -> Self::View<'a>;
}
```

When a user writes:
```rust
#[slice_struct]
struct Packet {
    #[slice] payload: Mutex<[u8]>,
}
```

The macro generates a sized field for the state in the prefix, and calculates the layout using the element type for the tail:
```rust
struct Packet {
    __payload_state: <Mutex<[u8]> as InlineSlice>::State,
    __align: [<Mutex<[u8]> as InlineSlice>::Element; 0],
    __data_tail: [MaybeUninit<u8>],
}
```

### The Built-in Implementations
The `slice_struct` crate will ship with implementations for standard types out of the box:
1. **`[T]`**: `State = ()`, `Element = T`, returns `&[T]`.
2. **`str`**: `State = ()`, `Element = u8`, returns `&str` (validates UTF-8 once on init, zero-cost access).
3. **`Mutex<[T]>`**: `State = Mutex<()>`, `Element = T`, returns a custom `SliceMutexGuard`.
4. **`RefCell<[T]>`**: `State = RefCell<()>`, `Element = T`, returns a custom `SliceRefGuard`.

**Why this is elegant:** The macro requires zero custom parsing. The user writes standard Rust types, and the compiler's trait system handles the physical memory layout split automatically.

---

## Feature 2: The `SliceBuilder` (In-Place Initialization)

### The Problem
If a user wants to allocate their `slice_struct` into an `Arc`, or if they want to wrap the *entire* struct in a `Mutex` (e.g., `Arc<DstMutex<Packet>>`), the macro currently has to generate a specific constructor for it. 
This leads to an explosive combinatorial API: `new_box_iter`, `new_arc_iter`, `new_arc_mutex_iter`, `new_rc_refcell_iter`, etc. This balloons compile times, makes the generated docs unreadable, and limits users to only the smart pointers we explicitly hardcode.

### The Solution: Separate "What" from "Where"
We will remove allocation logic from the macro entirely. The macro will generate a single `.init()` method that returns a `SliceBuilder`. All allocations and wrapping will be handled by composable trait methods in the library.

```rust
// 1. Generate the Initializer
let init = Packet::init(42, iter1, iter2);

// 2. Compose Wrappers (Type System)
let init_mutexed = init.with_mutex();

// 3. Terminal Allocation
let p: Pin<Arc<DstMutex<Packet>>> = init_mutexed.in_arc();
```

### The Architecture
We define a single trait that represents a pending allocation:
```rust
pub unsafe trait SliceInit<T: ?Sized> {
    fn layout(&self) -> Layout;
    unsafe fn init(self, ptr: *mut T);
}
```

The macro generates a private `PacketInit` struct holding the user's arguments (iterators, sized fields) and implements `SliceInit<Packet>`.

The `slice_struct` crate provides the builder methods:
1. **Wrappers (`with_mutex`)**: Returns a new `SliceBuilder` wrapping the old one. Its `layout()` method asks the inner builder for its layout, and extends it with `Layout::new::<Mutex<()>>()`. Its `init()` method writes the `Mutex::new(())` to the prefix, and delegates to the inner builder for the tail.
2. **Terminals (`in_arc`, `in_box`)**: Asks the builder for the final layout, calls `std::alloc::alloc`, runs the `.init()` method, and wraps the raw pointer in the requested smart pointer.

### `Arc` and `Rc` Safety (The Move-and-Fixup Pattern)
Because `Arc::from(Box)` physically moves memory, destroying our self-referential `NonNull` pointers, the `in_arc()` terminal will safely intercept the `Arc` before it is pinned. It will call `Arc::get_mut()`, recalculate the absolute memory addresses based on the new `ArcInner` heap location, update the pointers, and finally call `Pin::new_unchecked()`. 

**Why this is elegant:** The macro goes from generating a massive wall of allocation code to generating a single simple struct. Infinite composition (`Arc`, `Rc`, `Box`, `Mutex`, `RwLock`) is unlocked purely through standard Rust trait bounds.

---

## Feature 3: Inline Dynamic Traits (`#[dyn]`)

### The Problem
Dynamically dispatched traits (`dyn Trait`) are DSTs. To store them, users typically have to use `Box<dyn Trait>`, which introduces an extra heap allocation and a pointer hop (pointer chasing) every time the trait method is called. For high-performance networking or parsing, this extra cache miss is unacceptable. 

### The Solution: The Fat Pointer Trick
We will introduce a `#[dyn]` attribute that allows users to store `dyn Trait` instances inline directly inside the `slice_struct` allocation, right alongside their inline slices.

```rust
#[slice_struct]
struct Packet {
    #[slice] data: [u8],
    #[dyn] handler: dyn std::fmt::Debug, 
}
```

### The Architecture
Unlike a slice (which only needs to store a `len` in the sized prefix), a `dyn Trait` requires a **vtable**. A Rust fat pointer (`*mut dyn Trait`) contains exactly what we need: a pointer to the data, and a pointer to the vtable.

**1. The Prefix State:**
The macro generates a field in the sized prefix to store the full fat pointer:
```rust
struct Packet {
    __handler_ptr: *mut (dyn std::fmt::Debug + 'static),
}
```

**2. The Constructor:**
The `.init()` method generated by Feature 2 will accept a generic concrete type for the `#[dyn]` field:
```rust
pub fn init<H: std::fmt::Debug + 'static>(
    data: impl ExactSizeIterator<Item = u8>,
    handler: H, // User passes a concrete struct here
) -> SliceBuilder<Packet, PacketInit<...>>
```

**3. In-Place Initialization:**
Inside `SliceInit::init(ptr)`, the macro writes the concrete data to the tail, and uses Rust's coercion to capture the vtable:
```rust
// 1. Write the concrete struct `H` into the tail
std::ptr::write(tail_ptr as *mut H, self.handler);

// 2. Coerce the thin pointer to a fat pointer (attaches the vtable)
let fat_ptr = tail_ptr as *mut H as *mut dyn std::fmt::Debug;

// 3. Save the fat pointer in the prefix
(*ptr).__handler_ptr = fat_ptr;
```

**4. Zero-Cost Access:**
When the user calls `.view().handler`, the struct simply dereferences the fat pointer saved in the prefix, granting immediate zero-cost access to the dynamically dispatched trait object.

**Integration:** This integrates perfectly with Feature 2. The `SliceBuilder` doesn't care whether it's initializing slices or dynamic traits—it only cares about the final layout sizes. This allows `#[dyn]` fields to automatically benefit from `Arc`, `Box`, and `DstMutex` wrapping without any additional macro code!
