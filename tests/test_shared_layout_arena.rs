#![cfg(feature = "arena")]

use slice_struct::{slice_struct, ArenaDescriptor};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[slice_struct(arena, zerocopy)]
pub struct Child {
    pub value: u32,
    #[slice] pub extra: [u16],
}

#[slice_struct(shared_layout)]
pub struct Parent {
    pub id: u32,
    #[slice] pub children: ArenaSlice<Child>,
}

#[test]
fn test_shared_layout_arena() {
    // 3 children, each with 2 extra items.
    let child_arena = Child::init_arena(2);
    let table = Parent::make_table(3, child_arena);
    
    let mut p = Parent::init_with_table(table.clone(), 99, (0, 2)).in_box();
    
    assert_eq!(*p.view().id, 99);
    assert_eq!(p.view().children.len(), 3);
    
    for i in 0..3 {
        assert_eq!(*p.view().children.get(i).unwrap().value, 0);
        assert_eq!(p.view().children.get(i).unwrap().extra.len(), 2);
    }
    
    // Modify one
    p.view_mut_unpin().children.get_mut(1).unwrap().extra[0] = 42;
    assert_eq!(p.view().children.get(1).unwrap().extra[0], 42);
    assert_eq!(p.view().children.get(0).unwrap().extra[0], 2);
}

// Let's also do a drop test for arena within shared layout
pub struct DropChild {
    counter: Arc<AtomicUsize>,
}
impl Drop for DropChild {
    fn drop(&mut self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
}
impl Clone for DropChild {
    fn clone(&self) -> Self { Self { counter: self.counter.clone() } }
}

#[slice_struct(arena)]
pub struct DropInner {
    pub dummy: u8,
    #[slice] pub items: [DropChild],
}

#[slice_struct(shared_layout)]
pub struct DropParent {
    pub id: u32,
    #[slice] pub children: ArenaSlice<DropInner>,
}

#[test]
fn test_shared_layout_arena_drop() {
    let counter = Arc::new(AtomicUsize::new(0));
    let tracker = DropChild { counter: counter.clone() };
    
    let inner_arena = DropInner::init_arena(4); // 4 items per child
    let table = DropParent::make_table(2, inner_arena); // 2 children per parent
    
    {
        // Init data for ArenaSlice is (InitData, usize).
        // Wait, for ArenaSlice<DropInner>, the init data is the init data of DropInner!
        // DropInner init data is (u8, (DropChild, usize)). Wait, no, it's (u8, DropChild) since the length is fixed by the arena!
        // Actually, for inner slices, init_def takes the values to clone.
        // Wait! The init data for DropInner when used in an Arena is generated based on its fields.
        let p = DropParent::init_with_table(table.clone(), 1, (9, tracker.clone())).in_box();
        assert_eq!(p.view().children.len(), 2);
        assert_eq!(p.view().children.get(0).unwrap().items.len(), 4);
    } // Drops p and local 	racker
    
    // Total elements = 2 children * 4 items = 8.
    // Plus the local 	racker dropping = 1.
    // Total = 9.
    assert_eq!(counter.load(Ordering::SeqCst), 9);
}
