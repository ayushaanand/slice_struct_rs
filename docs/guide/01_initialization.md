# Initialization & Allocation

The `#[slice_struct]` macro fundamentally changes how your struct is constructed because the size of the allocation is only known at runtime. 

To handle this, the macro generates builder methods that determine the final size of the struct, allocate the memory, and initialize it safely.

## 1. Initializing with Default Values (`init_def`)

If you want to initialize the dynamic slices by cloning a default value, use the `init_def` method. 

The arguments to `init_def` must strictly follow the order of the fields in your struct:
1. **Plain Fields:** Pass the value directly.
2. **Slice Fields:** Pass a tuple `(default_value, length)`.

```rust
# use slice_struct::slice_struct;
#[slice_struct]
pub struct Packet {
    pub id: u32,
    #[slice] pub flags: [u16],
    #[slice] pub payload: [u8],
}

// 1. Plain field: 42
// 2. flags slice: 5 elements, all initialized to `0`
// 3. payload slice: 10 elements, all initialized to `0xFF`
let packet = Packet::init_def(42, (0, 5), (0xFF, 10)).in_box();
```

## 2. Initializing from Iterators (`init_iter`)

To initialize your slices directly from existing iterators (avoiding unnecessary clones or allocations), use the `init_iter` method.

The arguments follow the same field order:
1. **Plain Fields:** Pass the value directly.
2. **Slice Fields:** Pass an `ExactSizeIterator`.

```rust
# use slice_struct::slice_struct;
# #[slice_struct]
# pub struct Packet { pub id: u32, #[slice] pub flags: [u16], #[slice] pub payload: [u8] }
let flag_data = vec![1, 2, 3];
let string_data = "hello world";

let packet = Packet::init_iter(
    42,
    flag_data.into_iter(),    // ExactSizeIterator
    string_data.bytes(),      // ExactSizeIterator
).in_box();
```

## 3. Composable Allocation (`SliceBuilder`)

Both `init_def` and `init_iter` return a `SliceBuilder`, not the final struct. This builder allows you to specify *where* and *how* the memory should be allocated.

### Smart Pointers
You can finalize the allocation directly into standard Rust smart pointers:
- `.in_box()` -> `Pin<Box<Packet>>`
- `.in_arc()` -> `Pin<Arc<Packet>>`
- `.in_rc()`  -> `Pin<Rc<Packet>>`

### Dynamic Struct Locking (DST Locks)
Because the sizes of the slices are dynamic, wrapping the *entire struct* in a `Mutex` normally requires double-indirection (e.g., `Arc<Mutex<Box<Packet>>>`). 

`SliceBuilder` allows you to inject the lock directly into the single contiguous allocation natively:

```rust
# use slice_struct::slice_struct;
# #[slice_struct]
# pub struct Packet { pub id: u32, #[slice] pub payload: [u8] }
// Allocates [ Mutex State | Packet ID | Payload Array ] in ONE heap block!
let locked_packet = Packet::init_def(1, (0, 10))
    .with_mutex()
    .in_arc();

// You can lock the entire struct dynamically
let mut guard = locked_packet.lock();
guard.as_mut().view_mut().payload[0] = 99;
```

Available wrappers:
- `.with_mutex()` -> `DstMutex<Packet>`
- `.with_refcell()` -> `DstRefCell<Packet>`
