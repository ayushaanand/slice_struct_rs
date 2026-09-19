#![allow(unused_imports)]
use slice_struct::{ArenaDescriptor, ArenaSlice, slice_struct};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[slice_struct(arena)]
pub struct Basic {
    pub x: u32,
    #[slice]
    pub ys: [u32],
}

#[slice_struct]
pub struct BasicOuter {
    pub x: u32,
    #[slice]
    pub ys: ArenaSlice<Basic>,
}

#[test]
fn test_basic_arena() {
    let arena = BasicArena { ys_len: 3 };
    let b = BasicOuter::init_def(99, arena, ((10, 1), 2)).in_box();
    assert_eq!(*b.view().x, 99);
    assert_eq!(b.view().ys.len(), 2);
    assert_eq!(*b.view().ys.at(0).x, 10);
    assert_eq!(b.view().ys.at(0).ys.len(), 3);
    assert_eq!(&*b.view().ys.at(0).ys, &[1, 1, 1]);
}

#[slice_struct(arena)]
pub struct DeepInner {
    #[slice]
    pub elems: [u32],
}

#[slice_struct(arena)]
pub struct DeepMiddle {
    #[slice]
    pub inners: ArenaSlice<DeepInner>,
}

#[slice_struct]
pub struct DeepOuter {
    #[slice]
    pub inners: ArenaSlice<DeepMiddle>,
}

#[test]
fn test_deep_nesting() {
    let inner_arena = DeepInnerArena { elems_len: 2 };
    let middle_arena = DeepMiddleArena {
        inners_arena: inner_arena,
        inners_len: 2,
    };

    let outer = DeepOuter::init_def(middle_arena, (((10,),), 3)).in_box();

    assert_eq!(outer.view().inners.len(), 3);
    assert_eq!(outer.view().inners.at(0).inners.len(), 2);
    assert_eq!(outer.view().inners.at(0).inners.at(0).elems.len(), 2);
    assert_eq!(&*outer.view().inners.at(0).inners.at(0).elems, &[10, 10]);
}

#[slice_struct]
pub struct Mixed {
    #[slice]
    pub normal: [u32],
    #[slice]
    pub arenas: ArenaSlice<Basic>,
}

#[test]
fn test_mixed_fields() {
    let arena = BasicArena { ys_len: 2 };
    let m = Mixed::init_def((100, 2), arena, ((42, 5), 2)).in_box();

    assert_eq!(m.view().normal.len(), 2);
    assert_eq!(&*m.view().normal, &[100, 100]);

    assert_eq!(m.view().arenas.len(), 2);
    assert_eq!(*m.view().arenas.at(0).x, 42);
    assert_eq!(m.view().arenas.at(0).ys.len(), 2);
    assert_eq!(&*m.view().arenas.at(0).ys, &[5, 5]);
}

#[test]
fn test_arena_zero_len() {
    let inner_arena = DeepInnerArena { elems_len: 0 };
    let middle_arena = DeepMiddleArena {
        inners_arena: inner_arena,
        inners_len: 0,
    };
    let outer = DeepOuter::init_def(middle_arena, (((10,),), 0)).in_box();
    assert_eq!(outer.view().inners.len(), 0);
}

#[test]
fn test_arena_mutation() {
    let arena = BasicArena { ys_len: 3 };
    let mut b = BasicOuter::init_def(99, arena, ((10, 1), 2)).in_box();
    b.as_mut().view_mut().ys.at_mut(1).ys[1] = 99;
    assert_eq!(&*b.view().ys.at(1).ys, &[1, 99, 1]);
}

#[derive(Clone)]
pub struct DropCounter {
    pub id: u32,
    pub arc: Arc<AtomicUsize>,
}
impl Drop for DropCounter {
    fn drop(&mut self) {
        self.arc.fetch_add(1, Ordering::SeqCst);
    }
}

#[slice_struct(arena)]
pub struct DropInner {
    #[slice]
    pub counters: [DropCounter],
}

#[slice_struct]
pub struct DropOuter {
    #[slice]
    pub inners: ArenaSlice<DropInner>,
}

#[test]
fn test_nested_drop() {
    let counter = Arc::new(AtomicUsize::new(0));
    {
        let c1 = DropCounter {
            id: 1,
            arc: counter.clone(),
        };
        let arena = DropInnerArena { counters_len: 2 };
        let outer = DropOuter::init_def(arena, ((c1,), 3)).in_box();
        assert_eq!(outer.view().inners.len(), 3);
        assert_eq!(outer.view().inners.at(0).counters.len(), 2);
        counter.store(0, Ordering::SeqCst);
    }
    // 3 outer * 2 inner = 6 dropped!
    assert_eq!(counter.load(Ordering::SeqCst), 6);
}
