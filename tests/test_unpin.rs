use slice_struct::slice_struct;
use std::ptr;

#[slice_struct(unpin)]
pub struct PositionIndependent {
    pub id: u32,
    #[slice]
    pub data: [u8],
}

#[slice_struct]
pub struct PinnedStruct {
    pub id: u32,
    #[slice]
    pub data: [u8],
}

#[test]
fn test_unpin_trait_bounds() {
    fn assert_unpin<T: Unpin + ?Sized>() {}
    assert_unpin::<PositionIndependent>();
    // assert_unpin::<PinnedStruct>(); // COMPILE ERROR: `PinnedStruct` does not implement `Unpin`
}

#[test]
fn test_unpin_memory_move() {
    let boxed = PositionIndependent::init_iter(42, [1, 2, 3].into_iter()).in_box_unpin();

    // Prove it works normally via &self (no Pin required!)
    {
        let view = boxed.view();
        assert_eq!(*view.id, 42);
        assert_eq!(view.data, &[1, 2, 3]);
    }

    unsafe {
        let layout = std::alloc::Layout::for_value(&*boxed);
        let new_ptr = std::alloc::alloc(layout);

        ptr::copy_nonoverlapping(&*boxed as *const _ as *const u8, new_ptr, layout.size());

        let old_raw = Box::into_raw(boxed);

        let metadata = {
            let slice_ptr = old_raw as *mut [()];
            slice_ptr.len()
        };
        let new_fat_ptr = std::ptr::slice_from_raw_parts_mut(new_ptr as *mut (), metadata)
            as *mut PositionIndependent;

        let moved_box = Box::from_raw(new_fat_ptr);

        let view = moved_box.view();
        assert_eq!(*view.id, 42);
        assert_eq!(view.data, &[1, 2, 3]);

        std::alloc::dealloc(old_raw as *mut u8, layout);
    }
}

#[test]
fn test_pinned_memory_corruption_on_move() {
    let boxed = PinnedStruct::init_iter(42, [1, 2, 3].into_iter()).in_box();

    unsafe {
        let layout = std::alloc::Layout::for_value(&*boxed);
        let new_ptr = std::alloc::alloc(layout);

        // Illegally copy the pinned struct to a new address
        ptr::copy_nonoverlapping(&*boxed as *const _ as *const u8, new_ptr, layout.size());

        let old_raw = Box::into_raw(std::pin::Pin::into_inner_unchecked(boxed));

        let metadata = {
            let slice_ptr = old_raw as *mut [()];
            slice_ptr.len()
        };
        let new_fat_ptr =
            std::ptr::slice_from_raw_parts_mut(new_ptr as *mut (), metadata) as *mut PinnedStruct;

        let moved_box = Box::from_raw(new_fat_ptr);

        let view = moved_box.view();
        let current_slice_addr = view.data.as_ptr() as usize;
        let old_base_addr = old_raw as *const u8 as usize;
        let new_base_addr = new_ptr as usize;

        // Prove that the slice pointer is inside the OLD memory allocation, NOT the new one!
        assert!(
            current_slice_addr >= old_base_addr
                && current_slice_addr < old_base_addr + layout.size(),
            "The slice pointer should still point inside the old allocation! {current_slice_addr} vs {old_base_addr}"
        );

        assert!(
            current_slice_addr < new_base_addr
                || current_slice_addr >= new_base_addr + layout.size(),
            "The slice pointer did NOT update to the new allocation!"
        );

        // Note: we DO NOT access `view.data`'s elements here.
        // If we did, Miri would trigger UB because we are accessing `old_raw` which we are about to free,
        // or wait, `old_raw` is not freed yet! So we COULD access it!
        // But semantically, the pointer is corrupt because it points outside `moved_box`.

        // Clean up
        std::alloc::dealloc(old_raw as *mut u8, layout);
        // We must forget moved_box because dropping it would drop the corrupt pointers!
        // Wait! Dropping moved_box drops PinnedStruct, which has a Drop impl!
        // PinnedStruct's drop impl reads `self.__data.as_non_null(base)`.
        // For AbsoluteMode, `as_non_null` ignores `base` and returns the absolute pointer!
        // So dropping `moved_box` will call `drop_in_place` on `old_raw`'s slice elements!
        // But `old_raw`'s slice elements are ALREADY dropped when `old_raw` is deallocated?
        // No, we didn't drop `old_raw`, we just deallocated it.
        // Wait, if we let `moved_box` drop, it WILL drop the elements at `old_raw`, which is fine since we didn't drop them!
        // But then we must let `moved_box` drop, and then dealloc `old_raw`.
        // Let's prevent dropping `moved_box` to be absolutely safe and avoid Miri UB on dropping corrupt absolute pointers.
        std::mem::forget(moved_box);
        std::alloc::dealloc(new_ptr, layout);
    }
}
