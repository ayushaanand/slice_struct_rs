<!-- cargo-rdme start -->

# slice_struct

A Rust procedural macro for packing **multiple variable-length slices** and **complex stateful wrappers** into a single contiguous heap allocation.

Normally, putting multiple dynamic arrays in a struct requires multiple `Vec`s, which spreads your data across multiple separate heap allocations. `slice_struct` packs everything inline into one contiguous block of memory to guarantee data locality and reduce memory overhead.

## Quick Start

Add the dependency to your `Cargo.toml`. To use advanced features like nested arenas or binary parsing, enable the corresponding features:

```toml
[dependencies]
slice_struct = { version = "0.4.0", features = ["arena", "zerocopy"] }
```

Mark your dynamic fields with `#[slice]`.

```rust
use slice_struct::slice_struct;

#[slice_struct]
pub struct Packet {
    pub id: u32,
    #[slice] pub payload: [u8],
}

// 1. The macro generates `init_def`.
// Arguments strictly follow the struct fields: (id_value, (payload_default_val, payload_len))
let mut packet = Packet::init_def(42, (0, 10)).in_box();

// 2. The macro generates `.view()` and `.view_mut()` for safe data access
let mut v = packet.as_mut().view_mut();
*v.id = 1;
v.payload[0] = 255;
```

## Guides

Because `slice_struct` changes how your structs are allocated, the documentation split into dedicated guides focused on how to use the generated APIs.

- **[Initialization & Allocation](https://docs.rs/slice_struct/latest/slice_struct/guide/initialization/)**: Deep dive into `init_def`, `init_iter`, and composable allocations via `SliceBuilder` (`.in_box()`, `.in_arc()`, `.with_mutex()`).
- **[Implementing Methods](https://docs.rs/slice_struct/latest/slice_struct/guide/methods/)**: Learn how to encapsulate your logic by writing `impl` blocks on your generated structs, and how to use disjoint mutable borrowing.
- **[Shared Layouts & Unpin](https://docs.rs/slice_struct/latest/slice_struct/guide/shared_layouts/)**: Optimize memory for thousands of identical structs (ECS / Node graphs) using `#[slice_struct(shared_layout)]`, which natively unlocks `Unpin` standard methods.
- **[Nested Arenas](https://docs.rs/slice_struct/latest/slice_struct/guide/nested_arenas/)**: Embed dynamically sized arrays *inside* other dynamic arrays seamlessly using `ArenaSlice<T>`. *(Requires `arena` feature)*
- **[Zerocopy Parsing](https://docs.rs/slice_struct/latest/slice_struct/guide/zerocopy/)**: Safely cast binary byte buffers directly into viewable structs. *(Requires `zerocopy` feature)*

---

## Limitations

* **Named fields only:** Tuple and unit structs are not supported.
* **Ordering:** `#[slice]` fields must follow plain fields in the struct definition.

## License

MIT OR Apache-2.0

<!-- cargo-rdme end -->
