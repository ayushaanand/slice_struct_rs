#![allow(unused_imports)]
use slice_struct::{ArenaDescriptor, ArenaSlice, slice_struct};

#[slice_struct(arena)]
pub struct EmptyInner {
    pub a: u8,
    #[slice]
    pub b: [u8],
}

#[slice_struct]
pub struct EmptyOuter {
    pub c: u8,
    #[slice]
    pub d: ArenaSlice<EmptyInner>,
}

#[test]
fn test_arena_zero_len() {
    let arena_empty = EmptyInner::init_arena(0);

    // 0 elements in outer
    let out1 = EmptyOuter::init_def(1, arena_empty.clone(), ((2, 3), 0)).in_box();
    assert_eq!(*out1.view().c, 1);
    assert_eq!(out1.view().d.len(), 0);

    // 3 elements in outer, 0 in inner
    let out2 = EmptyOuter::init_def(1, arena_empty, ((2, 0), 3)).in_box();
    assert_eq!(*out2.view().c, 1);
    assert_eq!(out2.view().d.len(), 3);
    for i in 0..3 {
        assert_eq!(*out2.view().d.at(i).a, 2);
        assert_eq!(out2.view().d.at(i).b.len(), 0);
    }
}
