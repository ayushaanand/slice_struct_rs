# Shared Layouts (ECS & Caching)

Every dynamic slice in a `slice_struct` requires internal metadata (offsets and lengths) to safely navigate the contiguous heap allocation. 

If you are allocating thousands of independent structs that all happen to share the exact same slice lengths (for example, in an Entity Component System or a highly structured Node Graph), storing this metadata inside every single allocation wastes memory.

You can fix this by using `#[slice_struct(shared_layout)]`.

## How it works

When you use `shared_layout`, the macro moves all the slice metadata out of the individual instances and into a `LayoutTable`. Your structs will instead store a single `Arc<LayoutTable>` pointing to the shared math.

```rust
# use slice_struct::slice_struct;
#[slice_struct(shared_layout)]
pub struct Entity {
    pub id: u32,
    #[slice] pub positions: [f32],
    #[slice] pub velocities: [f32],
}
```

## 1. Creating the Table (`make_table`)

Before you allocate your structs, you must initialize the shared layout table. The arguments to `make_table` correspond exactly to the lengths of the slices defined in your struct.

```rust
# use slice_struct::slice_struct;
# #[slice_struct(shared_layout)]
# pub struct Entity { pub id: u32, #[slice] pub positions: [f32], #[slice] pub velocities: [f32] }
// Create a table for entities that all have exactly 3 positions and 3 velocities.
// Arguments: (positions_len, velocities_len)
let table = Entity::make_table(3, 3);
```

## 2. Instantiating (`init_with_table`)

Once you have the table, you use `init_with_table` to allocate your structs. You pass in a clone of the table, followed by the default values for your plain fields and slices.

Because the lengths are already strictly defined by the table, you **do not** pass lengths for the slices—only the default values!

```rust
# use slice_struct::slice_struct;
# #[slice_struct(shared_layout)]
# pub struct Entity { pub id: u32, #[slice] pub positions: [f32], #[slice] pub velocities: [f32] }
# let table = Entity::make_table(3, 3);
// 1. Pass the table clone
// 2. Plain field: id = 1
// 3. positions slice default: 0.0
// 4. velocities slice default: 1.0
let entity1 = Entity::init_with_table(table.clone(), 1, 0.0, 1.0).in_box();
let entity2 = Entity::init_with_table(table.clone(), 2, 0.0, 1.0).in_box();
```

*(Note: There is also an `init_with_table_iter` method if you want to initialize the slices from flat iterators).*

## 3. Free `Unpin`!

Because the layout table fully abstracts the internal math, structs defined with `shared_layout` are inherently relative and safe to move in memory! 

This means they automatically implement `Unpin`, and you can write standard `&mut self` methods using `.view_mut_unpin()` without ever dealing with `Pin`.

```rust
# use slice_struct::slice_struct;
# #[slice_struct(shared_layout)]
# pub struct Entity { pub id: u32, #[slice] pub positions: [f32], #[slice] pub velocities: [f32] }
impl Entity {
    pub fn update(&mut self) {
        let mut v = self.view_mut_unpin();
        v.positions[0] += v.velocities[0];
    }
}
```
