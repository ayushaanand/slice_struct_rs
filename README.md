# slice_struct

A Rust procedural macro for structs with **multiple inline, variable-length slices**(length is not mut, but also not const) packed into a single heap allocation — without any pointer indirection.

## Motivation

Rust's type system allows a struct to have exactly **one** trailing [DST](https://doc.rust-lang.org/reference/dynamically-sized-types.html) field (e.g. `[T]`). If you need two variable-length arrays alongside each other, the usual answer is multiple `Vec` fields — but that scatters data across three separate allocations per `Vec`.

`slice_struct` solves this by generating a custom `repr(C)` layout where all slices are packed inline, back-to-back, inside the **same** allocation as the struct's sized fields, with only alignment padding between them.

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
slice_struct = "0.1.0"
```

Apply `#[slice_struct]` to any struct with named fields.  Mark every variable-length field with `#[slice]`:

```rust
use slice_struct::slice_struct;

#[slice_struct]
pub struct Packet {
    pub id:      u32,
    #[slice] pub payload: u8,
    #[slice] pub tags:    u32,
}
```

## Constructors

### `new_box_iter` — iterator-based, zero-clone

Each `#[slice]` field receives an `impl ExactSizeIterator<Item = FieldTy>`.  Elements are written directly into the single allocation — no intermediate buffer, no `Clone` bound.

```rust
let p = Packet::new_box_iter(
    42,
    [10_u8, 20, 30].into_iter(),   // payload
    100_u32..104,                  // tags — any ExactSizeIterator
);

assert_eq!(p.id,        42);
assert_eq!(p.payload(), &[10, 20, 30]);
assert_eq!(p.tags(),    &[100, 101, 102, 103]);
```

Works with `Vec::into_iter()`, arrays, `Range`, mapped iterators, and anything else that implements `ExactSizeIterator`.

### `new_box_def` — `(value, length)` pair

Like `vec![val; len]`, but written directly into the target allocation without creating a temporary `Vec`.  Pass a `(v, n)` tuple per `#[slice]` field.

```rust
// [0_u8; 8] payload and [0_u32; 4] tags, in one allocation.
let p = Packet::new_box_def(1, (0_u8, 8), (0_u32, 4));

assert_eq!(p.payload(), &[0_u8; 8]);
assert_eq!(p.tags(),    &[0_u32; 4]);
```

Requires `T: Clone` on each slice element type (same as `vec![v; n]`).

## Getters and setters

For each `#[slice] foo: T` field the macro generates:

```rust
fn foo(&self)         -> &[T]
fn foo_mut(&mut self) -> &mut [T]
```

```rust
let mut p = Packet::new_box_iter(0, [1_u8, 2, 3].into_iter(), [].into_iter());

p.payload_mut()[0] = 99;
assert_eq!(p.payload(), &[99, 2, 3]);
```

## Memory layout

For a struct with sized field `id: u32` and slice fields `payload: u8`, `tags: u32`:

```
┌──────────────────────────────────────────────────────────────────────┐
│  Packet_SizedPrefix                                                  │
│  ┌─────────┬──────────────┬─────────────┬──────────────┬──────────┐ │
│  │ id: u32 │ __payload_len│ __payload_  │ __tags_len   │ __tags_  │ │
│  │         │    usize     │ align [u8;0]│    usize     │align[u32;│ │
│  └─────────┴──────────────┴─────────────┴──────────────┴──────────┘ │
├──────────────────────────────────────────────────────────────────────┤
│  payload[0]  payload[1]  …  payload[payload_len-1]    (u8, inline)  │
├──────────────────────────────────────────────────────────────────────┤
│  padding to align u32                                                │
├──────────────────────────────────────────────────────────────────────┤
│  tags[0]  tags[1]  …  tags[tags_len-1]                (u32, inline) │
└──────────────────────────────────────────────────────────────────────┘
```

Everything lives in **one** allocation.  Slice byte-offsets are computed from the stored length fields at getter call time via `std::alloc::Layout` arithmetic — no extra storage needed.

## Drop behaviour

A `Drop` impl is automatically generated that runs element destructors (`ptr::drop_in_place`) for every slice before the `Box` dealloc fires.  This is correct for all element types, including `String`, `Vec`, `Arc`, and custom types with non-trivial destructors.

## Generics

The macro fully supports generic structs:

```rust
#[slice_struct]
struct Pair<A: Clone, B: Clone> {
    count: u32,
    #[slice] first:  A,
    #[slice] second: B,
}

let p = Pair::<i32, f64>::new_box_iter(2, [1, 2].into_iter(), [3.0, 4.0].into_iter());
```

## Limitations

| Limitation | Reason |
|---|---|
| Named fields only | Tuple and unit structs are not supported |
| `#[slice]` fields must follow plain fields in source order | Matches the physical allocation layout |
| `new_box_def` requires `T: Clone` | Needed to fill each slot |
| Cannot construct on the stack | The struct is `?Sized`; use the generated `Box`-returning constructors |

## Crate structure

```
slice_struct/
├── Cargo.toml               # workspace root + main crate
├── src/
│   └── lib.rs               # re-exports the macro; documentation lives here
└── slice_struct_macro/
    ├── Cargo.toml           # proc-macro crate (proc-macro = true)
    └── src/
        └── lib.rs           # macro implementation using syn + quote
```

## License

MIT OR Apache-2.0
