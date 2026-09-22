#![cfg(feature = "arena")]
use slice_struct::{slice_struct, ArenaDescriptor};
use std::cell::RefCell;

#[slice_struct(arena)]
pub struct CellInner {
    pub x: u32,
    #[slice]
    pub ys: RefCell<[u32]>,
}

#[slice_struct]
pub struct CellOuter {
    pub a: u32,
    #[slice]
    pub inners: ArenaSlice<CellInner>,
}

#[test]
fn test_cell_arena() {
    let arena = CellInner::init_arena(2);
    let mut original = CellOuter::init_def(99, arena, ((42, 10), 3)).in_box();
    
    let mut outer_view = original.as_mut().view_mut();
    for i in 0..3 {
        let mut view = outer_view.inners.at_mut(i);
        assert_eq!(*view.x, 42);
        view.ys[0] = 99;
    }
}

