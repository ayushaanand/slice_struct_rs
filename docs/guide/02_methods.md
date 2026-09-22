# Implementing Methods & Mutation

While `slice_struct` handles the complex memory layout under the hood, the goal is for you to be able to treat the generated types exactly like regular Rust structs.

You can and should implement standard Domain-Driven Design methods on your structs to encapsulate your logic!

## 1. Immutable Methods (`&self`)

Immutable methods work exactly as you would expect. Simply call `.view()` to access the fields safely.

```rust
# use slice_struct::slice_struct;
#[slice_struct]
pub struct Packet {
    pub id: u32,
    #[slice] pub payload: [u8],
}

impl Packet {
    pub fn get_id(&self) -> u32 {
        *self.view().id
    }

    pub fn payload_len(&self) -> usize {
        self.view().payload.len()
    }
}
```

## 2. Mutable Methods (`Pin<&mut Self>`)

Because standard `slice_struct`s use absolute internal memory pointers to guarantee zero-cost reads, the struct **cannot be safely moved in memory**. Therefore, the generated allocation methods return pinned pointers (e.g., `Pin<Box<Packet>>`), and your mutable methods must require `Pin<&mut Self>`.

To mutate the struct, call `.view_mut()` on the pinned reference:

```rust
# use core::pin::Pin;
# use slice_struct::slice_struct;
# #[slice_struct]
# pub struct Packet { pub id: u32, #[slice] pub payload: [u8] }
impl Packet {
    pub fn process(self: Pin<&mut Self>) {
        // Obtain a mutable view
        let mut v = self.view_mut();
        
        // Mutate plain fields
        *v.id += 1;
        
        // Mutate slices
        v.payload[0] = 0xFF;
    }
}
```

## 3. Disjoint Mutable Borrowing

A major feature of `slice_struct` is that the `.view_mut()` method returns a transparent struct containing explicit native references to the fields. 

This means you benefit entirely from Rust's native borrow checker. You can safely hold mutable references to different slices simultaneously!

```rust
# use core::pin::Pin;
# use slice_struct::slice_struct;
# #[slice_struct]
# pub struct SplitData { #[slice] pub left: [u8], #[slice] pub right: [u8] }
impl SplitData {
    pub fn compute(self: Pin<&mut Self>) {
        let mut v = self.view_mut();
        
        // The borrow checker fully understands these are disjoint memory regions
        let l: &mut [u8] = v.left;
        let r: &mut [u8] = v.right;
        
        l[0] = r[0];
    }
}
```

## 4. Unpin Structs (`&mut self`)

If you want to avoid `Pin` entirely and write purely standard `&mut self` methods, you can opt into `Unpin` behavior by using the `#[slice_struct(unpin)]` attribute.

When you do this, the macro swaps the internal memory layout to use relative offsets rather than absolute pointers. This makes the struct safe to move in memory.

```rust
# use slice_struct::slice_struct;
#[slice_struct(unpin)]
pub struct Entity {
    pub x: f32,
    #[slice] pub flags: [u32],
}

impl Entity {
    // Pure standard Rust method! No `Pin` required!
    pub fn reset(&mut self) {
        // Use `view_mut_unpin()` to mutate cleanly
        let mut v = self.view_mut_unpin();
        *v.x = 0.0;
        v.flags[0] = 0;
    }
}
```
*(Note: Using `shared_layout` also natively unlocks `Unpin` behavior).*
