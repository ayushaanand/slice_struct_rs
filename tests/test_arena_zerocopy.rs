#![cfg(feature = "arena")]
#![cfg(feature = "zerocopy")]
use slice_struct::{slice_struct, ArenaDescriptor};

#[slice_struct(arena, zerocopy)]
pub struct ZeroInner {
    pub x: u32,
    #[slice]
    pub ys: [u32],
}

#[slice_struct(zerocopy)]
pub struct ZeroOuter {
    pub a: u32,
    #[slice]
    pub inners: ArenaSlice<ZeroInner>,
}

#[test]
fn test_zerocopy_arena_populated() {
    let arena = ZeroInner::init_arena(2);
    let original = ZeroOuter::init_def(99, arena, ((42, 10), 3)).in_box();
    
    // Convert to bytes directly by casting the original allocated block
    let bytes = unsafe {
        let size = std::mem::size_of_val(&*original);
        let ptr = &*original as *const _ as *const u8;
        std::slice::from_raw_parts(ptr, size)
    };
    
    // Cast back from bytes
    let recovered = ZeroOuter::ref_from_bytes(bytes).expect("Zerocopy failed!");
    
    // Validate everything perfectly matches!
    assert_eq!(*recovered.view().a, 99);
    assert_eq!(recovered.view().inners.len(), 3);
    
    for i in 0..3 {
        assert_eq!(*recovered.view().inners.at(i).x, 42);
        assert_eq!(recovered.view().inners.at(i).ys.len(), 2);
        assert_eq!(&*recovered.view().inners.at(i).ys, &[10, 10]);
    }
}

