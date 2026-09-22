# Nested Arenas

A powerful feature of `slice_struct` is the ability to perfectly embed dynamically sized data structures *inside* other dynamically sized data structures natively on the heap.

This is accomplished using `ArenaSlice<T>`.

*(Requires the `arena` feature)*

## Defining Arenas

To embed a struct inside an `ArenaSlice`, that struct must itself be marked with `#[slice_struct(arena)]`.

```rust
# use slice_struct::{slice_struct, ArenaSlice, ArenaDescriptor};
// The Child is dynamically sized because it contains a slice
#[slice_struct(arena)]
pub struct Child {
    pub id: u32,
    #[slice] pub data: [u8],
}

// The Parent contains an ArenaSlice of Children
#[slice_struct]
pub struct Parent {
    pub name_hash: u64,
    #[slice] pub children: ArenaSlice<Child>,
}
```

## 1. Initializing the Arena Descriptor (`init_arena`)

Because the children inside the `ArenaSlice` are dynamically sized, you must first define their layout by initializing an `ArenaDescriptor`. 

You do this by calling `.init_arena()` on the child type. The arguments you pass to `init_arena` represent the lengths of the slices *inside* the child.

```rust
# use slice_struct::{slice_struct, ArenaSlice, ArenaDescriptor};
# #[slice_struct(arena)]
# pub struct Child { pub id: u32, #[slice] pub data: [u8] }
// Create an arena where every `Child` will have exactly 5 elements in its `data` slice
let arena = Child::init_arena(5);
```

## 2. Instantiating the Parent

When you initialize the parent, you must provide the arena descriptor in place of the slice. Additionally, the default value you provide for the arena must be a tuple representing the default values for the child's fields!

```rust
# use slice_struct::{slice_struct, ArenaSlice, ArenaDescriptor};
# #[slice_struct(arena)]
# pub struct Child { pub id: u32, #[slice] pub data: [u8] }
# #[slice_struct]
# pub struct Parent { pub name_hash: u64, #[slice] pub children: ArenaSlice<Child> }
# let arena = Child::init_arena(5);
// Parent::init_def arguments:
// 1. name_hash
// 2. arena_descriptor
// 3. children defaults: ((child_id, child_data_default), length_of_arena)
let parent = Parent::init_def(
    12345, 
    arena, 
    ((1, 0xFF), 3) 
    // Create 3 children each with id 1 and data.len() == 5, each initialized to 0xFF
).in_box();
```

## 3. Navigating Nested Arenas

To access the child items, use `.at(index)` on the `ArenaSlice` view. This perfectly resolves the internal offsets and gives you an inline view into the child struct!

```rust
# use slice_struct::{slice_struct, ArenaSlice, ArenaDescriptor};
# #[slice_struct(arena)]
# pub struct Child { pub id: u32, #[slice] pub data: [u8] }
# #[slice_struct]
# pub struct Parent { pub name_hash: u64, #[slice] pub children: ArenaSlice<Child> }
# let arena = Child::init_arena(5);
# let parent = Parent::init_def(12345, arena, ((1, 0xFF), 3)).in_box();
let v = parent.view();

// We have 3 children in the arena
assert_eq!(v.children.len(), 3);

// Navigate cleanly into the dynamically sized nested structures
let child_0 = v.children.at(0);
assert_eq!(*child_0.id, 1);
assert_eq!(child_0.data.len(), 5);
assert_eq!(child_0.data[0], 0xFF);
```
