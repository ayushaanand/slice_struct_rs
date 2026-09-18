use slice_struct::slice_struct;
use std::ptr;

#[slice_struct(unpin)]
pub struct PositionIndependent {
    pub id: u32,
    #[slice] pub data: [u8],
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
        
        // Raw memcpy the bytes! (This would instantly corrupt absolute pointers)
        ptr::copy_nonoverlapping(&*boxed as *const _ as *const u8, new_ptr, layout.size());
        
        let old_raw = Box::into_raw(boxed);
        
        // Construct the new fat pointer by replacing the address of the old fat pointer
        let mut new_fat = old_raw as *mut ();
        new_fat = new_ptr as *mut ();
        
        // Actually the safest way to replace the address of a fat pointer is pointer::with_addr
        // Since with_addr is unstable, we can use slice_from_raw_parts_mut with the same length
        let metadata = {
            let slice_ptr = old_raw as *mut [()];
            slice_ptr.len()
        };
        let new_fat_ptr = std::ptr::slice_from_raw_parts_mut(new_ptr as *mut (), metadata) as *mut PositionIndependent;
        
        let moved_box = Box::from_raw(new_fat_ptr);
        
        let view = moved_box.view();
        assert_eq!(*view.id, 42);
        assert_eq!(view.data, &[1, 2, 3]);
        
        // DO NOT free the old raw pointer, as we copied the memory and the new Box owns the drop.
        // Wait, we DO need to free the old raw allocation because alloc(layout) created a second allocation!
        // But if we drop moved_box, it will drop the slice elements.
        // If we also drop old_raw, it will double-drop the slice elements!
        // So we must manually deallocate old_raw WITHOUT dropping it!
        std::alloc::dealloc(old_raw as *mut u8, layout);
    }
}
