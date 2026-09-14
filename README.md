# slice_struct

A Rust procedural macro for packing **multiple variable-length slices** into a single heap allocation. 

Normally, putting multiple dynamic arrays in a struct requires multiple `Vec`s, which spreads your data across multiple separate heap allocations. `slice_struct` packs everything inline into one contiguous block of memory, offering zero-cost access and perfect memory safety.

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
slice_struct = "0.2.0"
```

### 1. Defining and Reading

Mark your dynamic fields with `#[slice]`. The macro securely encapsulates the internal layout to guarantee memory safety, so you interact with your data safely through `.view()` and `.view_mut()`.

```rust
use slice_struct::slice_struct;

#[slice_struct]
pub struct Packet {
    pub id: u32,
    #[slice] pub payload: u8,
    #[slice] pub tags: u32,
}

// Create a new packet, populating the slices from iterators (zero-clone)
let p = Packet::new_box_iter(
    42,                            // id
    [10_u8, 20, 30].into_iter(),   // payload
    100_u32..104,                  // tags
);

// Read fields via `.view()`
let v = p.view();
assert_eq!(*v.id, 42);
assert_eq!(v.payload, &[10, 20, 30]);
assert_eq!(v.tags, &[100, 101, 102, 103]);
```

### 2. Mutating (Disjoint Borrowing)

To modify the data, use `.as_mut().view_mut()`. This provides safe, disjoint mutable borrowing, meaning you can mutate one slice while safely reading from another at the exact same time.

```rust
// Create a packet using default values and lengths (like vec![val; len])
let mut p = Packet::new_box_def(1, (0_u8, 8), (100_u32, 4));

let mut v = p.as_mut().view_mut();

// We can safely read `tags` while mutating `payload`!
let tags: &[u32] = &v.tags;
v.payload[0] = tags[0] as u8;

// Standard fields are mutated via dereferencing
*v.id = 43;
```

*(Note: `.as_mut()` is required because the macro returns a pinned pointer `Pin<Box<Self>>`, and standard Rust rules require you to explicitly borrow pinned boxes).*

## Constructors

The macro generates two constructors:
* `new_box_iter`: Takes an `ExactSizeIterator` for each slice field. Elements are written directly into the allocation with no intermediate buffers.
* `new_box_def`: Takes a `(value, length)` pair for each slice field and clones the value into each slot (requires `T: Clone`).

## Limitations

* **Named fields only:** Tuple and unit structs are not supported.
* **Ordering:** `#[slice]` fields must follow plain fields in the struct definition.
* **Pinning:** The resulting struct is `!Sized` and `!Unpin`, meaning it must live exclusively behind a pointer (like `Pin<Box<Self>>`) and cannot be moved in memory.
* **Derives:** Standard `#[derive(Clone)]` is not safe. If cloning is needed, implement it manually by allocating a new box.

## License

MIT OR Apache-2.0
