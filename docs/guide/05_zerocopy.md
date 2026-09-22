# Zerocopy Support

If you need to parse a dynamically sized binary structure directly from disk or network, `slice_struct` provides seamless integration with the `zerocopy` crate.

*(Requires the `zerocopy` feature)*

## 1. Opting In

Add the `#[slice_struct(zerocopy)]` attribute to your struct. This will automatically implement `FromBytes` and `Immutable` for your struct's internal layout.

**Requirement:** All types inside your struct (plain fields and slice elements) must themselves implement `FromBytes`.

```rust
# use slice_struct::slice_struct;
#[slice_struct(zerocopy)]
pub struct Packet {
    pub id: u32,
    #[slice] pub payload: [u8],
}
```

## 2. Casting from Bytes

Once you have a buffer of bytes (e.g., from a network socket or file), you can cast it safely into a view of your struct using `.ref_from_bytes()` or `.mut_from_bytes()`.

The lengths of the slices are automatically validated against the buffer size! You do not need to provide the lengths explicitly, because the internal layout metadata is encoded directly in the struct's byte stream.

```rust
# use slice_struct::slice_struct;
# #[slice_struct(zerocopy)]
# pub struct Packet { pub id: u32, #[slice] pub payload: [u8] }
// Imagine this buffer came from a network socket
// Contains: internal length/offset metadata, id (42), payload ([1,2,3,4])
// (For this example, we generate valid bytes via initialization)
let original = Packet::init_def(42, (0, 4)).in_box();
let original_bytes: &[u8] = unsafe { 
    std::slice::from_raw_parts(&*original as *const _ as *const u8, std::mem::size_of_val(&*original)) 
};

// Cast safely! We just provide the raw byte buffer.
let packet = Packet::ref_from_bytes(original_bytes).expect("Buffer size mismatch!");

assert_eq!(*packet.view().id, 42);
assert_eq!(packet.view().payload, &[0, 0, 0, 0]);
```

## Nested Zerocopy Arenas

You can even zerocopy cast an entire hierarchical data structure by combining `zerocopy` with `arena`. Because the layout math is serialized within the parent, casting is incredibly simple!

```rust
# use slice_struct::{slice_struct, ArenaSlice, ArenaDescriptor};
#[slice_struct(arena, zerocopy)]
pub struct Child {
    pub id: u32,
    #[slice] pub data: [u8],
}

#[slice_struct(zerocopy)]
pub struct Parent {
    #[slice] pub children: ArenaSlice<Child>,
}
```

When casting the parent from bytes, you don't even need the `ArenaDescriptor`. The structure fully decodes itself!

```rust
# use slice_struct::{slice_struct, ArenaSlice, ArenaDescriptor};
# #[slice_struct(arena, zerocopy)]
# pub struct Child { pub id: u32, #[slice] pub data: [u8] }
# #[slice_struct(zerocopy)]
# pub struct Parent { #[slice] pub children: ArenaSlice<Child> }
let arena = Child::init_arena(5);
let original = Parent::init_def(arena, ((1, 0xFF), 2)).in_box();
let bytes: &[u8] = unsafe { 
    std::slice::from_raw_parts(&*original as *const _ as *const u8, std::mem::size_of_val(&*original)) 
};

// Instantly decode the hierarchy!
let decoded = Parent::ref_from_bytes(bytes).unwrap();
assert_eq!(*decoded.view().children.at(0).id, 1);
```