# slice_struct

A Rust procedural macro for structs with **multiple inline, variable-length slices** packed into a single heap allocation — without any pointer indirection.

## Motivation

Rust's type system allows a struct to have exactly **one** trailing [DST](https://doc.rust-lang.org/reference/dynamically-sized-types.html) field (e.g. `[T]`). If you need two variable-length arrays alongside each other, the usual answer is multiple `Vec` fields — but that scatters data across three separate allocations per `Vec`.

`slice_struct` solves this by generating a custom `repr(C)` layout where all slices are packed inline, back-to-back, inside the **same** allocation as the struct's sized fields, with only alignment padding between them.

It provides a 100% sound, zero-cost API with **safe disjoint mutable borrowing**.

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
slice_struct = "0.1.1"
```

Apply `#[slice_struct]` to any struct with named fields. Mark every variable-length field with `#[slice]`:

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

Because `#[slice_struct]` creates a self-referential pinned struct, all constructors return a `Pin<Box<Self>>`.

### `new_box_iter` — iterator-based, zero-clone

Each `#[slice]` field receives an `impl ExactSizeIterator<Item = FieldTy>`. Elements are written directly into the single allocation — no intermediate buffer, no `Clone` bound.

```rust
let p = Packet::new_box_iter(
    42,
    [10_u8, 20, 30].into_iter(),   // payload
    100_u32..104,                  // tags — any ExactSizeIterator
);

assert_eq!(p.id, 42);
assert_eq!(&*p.payload, &[10, 20, 30]);
assert_eq!(&*p.tags,    &[100, 101, 102, 103]);
```

Works with `Vec::into_iter()`, arrays, `Range`, mapped iterators, and anything else that implements `ExactSizeIterator`.

### `new_box_def` — `(value, length)` pair

Like `vec![val; len]`, but written directly into the target allocation without creating a temporary `Vec`. Pass a `(v, n)` tuple per `#[slice]` field.

```rust
// [0_u8; 8] payload and [0_u32; 4] tags, in one allocation.
let p = Packet::new_box_def(1, (0_u8, 8), (0_u32, 4));

assert_eq!(&*p.payload, &[0_u8; 8]);
assert_eq!(&*p.tags,    &[0_u32; 4]);
```

Requires `T: Clone` on each slice element type (same as `vec![v; n]`).

## Field Access & Disjoint Borrowing

For each `#[slice] foo: T` field, the macro creates an actual field:
`foo: SliceHandle<T>`

### Immutable Access

`SliceHandle` implements `Deref<Target = [T]>`. You can access elements exactly like a standard slice:

```rust
let first_tag = p.tags[0];
let payload_slice = &*p.payload;
```

### Mutable Access (Disjoint Borrowing)

Because the struct is pinned (`!Unpin`), you cannot mutably access fields directly. Instead, a `.project()` method is automatically generated for safe, zero-cost disjoint borrowing:

```rust
// 1. Get a mutable projection of the pinned struct
let mut proj = p.as_mut().project();

// 2. Both mutable slices can be used simultaneously!
proj.payload.as_mut_slice()[0] = 99;
proj.tags.as_mut_slice()[0] = 100;

// Sized fields are accessed directly
*proj.id = 43;
```

## Memory layout

For a struct with sized field `id: u32` and slice fields `payload: u8`, `tags: u32`:

```text
┌────────────────────────────────────────────────────────────────────────┐
│  Packet                                                                │
│  ┌─────────┬──────────────┬──────────────┬─────────────┬─────────────┐ │
│  │ id: u32 │   padding    │payload:      │ tags:       │  __pin:     │ │
│  │         │   (4 bytes)  │SliceHandle<u8│SliceHandle< │PhantomPinned│ │
│  │         │              │  (16 bytes)  │u32> (16b)   │   (0b)      │ │
│  └─────────┴──────────────┴──────────────┴─────────────┴─────────────┘ │
├────────────────────────────────────────────────────────────────────────┤
│  payload[0]  payload[1]  …  payload[payload_len-1]      (u8, inline)   │
├────────────────────────────────────────────────────────────────────────┤
│  padding to align u32                                                  │
├────────────────────────────────────────────────────────────────────────┤
│  tags[0]  tags[1]  …  tags[tags_len-1]                  (u32, inline)  │
└────────────────────────────────────────────────────────────────────────┘
```

Everything lives in **one** allocation. The `SliceHandle` fields contain the raw pointer and length for immediate `O(1)` access without recomputing layout offsets.

## Drop behaviour

`SliceHandle` automatically implements `Drop` to run element destructors (`ptr::drop_in_place`) for every slice before the `Box` dealloc fires. This is correct for all element types, including `String`, `Vec`, `Arc`, and custom types with non-trivial destructors.

## Limitations

| Limitation | Reason |
|---|---|
| Named fields only | Tuple and unit structs are not supported |
| `#[slice]` fields must follow plain fields in source order | Matches the physical allocation layout |
| `new_box_def` requires `T: Clone` | Needed to fill each slot |
| Cannot construct on the stack | The struct is `?Sized` and self-referential |
| Moving is forbidden | The struct is `!Unpin` and lives exclusively in `Pin<Box<Self>>` |
| `#[derive(Clone)]` is unsafe | Bitwise cloning the handles creates dangling pointers. Implement `Clone` manually by allocating a new box. |

## License

MIT OR Apache-2.0
