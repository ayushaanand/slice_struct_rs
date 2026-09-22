use slice_struct::slice_struct;
use core::pin::Pin;
use std::cell::RefCell;

// =========================================================================
// Example 1: Standard slice_struct requiring Pin for mutable access
// =========================================================================

#[slice_struct]
pub struct ServerLog {
    pub timestamp: u64,
    // RefCell allows interior mutability without Pin!
    #[slice] pub events: RefCell<[String]>, 
    // Standard slice requires Pin<&mut Self> for mutation
    #[slice] pub buffer: [u8],
}

impl ServerLog {
    /// Read-only method using standard &self
    pub fn get_timestamp(&self) -> u64 {
        *self.view().timestamp
    }

    /// Mutable method on a standard slice_struct REQUIRES Pin<&mut Self>
    pub fn write_buffer(self: Pin<&mut Self>, offset: usize, data: &[u8]) {
        let v = self.view_mut();
        let end = (offset + data.len()).min(v.buffer.len());
        let copy_len = end - offset;
        v.buffer[offset..end].copy_from_slice(&data[..copy_len]);
    }

    /// Using interior mutability (RefCell), we can mutate a slice using ONLY &self!
    pub fn add_event(&self, index: usize, event: String) {
        let v = self.view();
        let mut events_guard = v.events; // This returns a SliceRefGuard (like RefMut)
        if index < events_guard.len() {
            events_guard[index] = event;
        }
    }
}

#[test]
fn test_standard_impl() {
    let empty_string = String::new();
    let mut log = ServerLog::init_def(1700000000, (empty_string, 2), (0, 10)).in_box();

    // 1. Immutable call
    assert_eq!(log.get_timestamp(), 1700000000);

    // 2. Mutable call requiring Pin (using .as_mut())
    log.as_mut().write_buffer(2, &[0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(&log.view().buffer[2..6], &[0xDE, 0xAD, 0xBE, 0xEF]);

    // 3. Mutable call via interior mutability using ONLY a shared reference
    log.add_event(1, "Server Started".to_string());
    assert_eq!(&log.view().events[1], "Server Started");
}


// =========================================================================
// Example 2: shared_layout which auto-forces Unpin (no Pin needed!)
// =========================================================================

#[slice_struct(shared_layout)]
pub struct PlayerState {
    pub player_id: u64,
    #[slice] pub permissions: [u8],
    #[slice] pub inventory_slots: [u32],
}

impl PlayerState {
    pub fn has_permission(&self, perm: u8) -> bool {
        self.view().permissions.contains(&perm)
    }

    /// Because shared_layout calculates offsets statically, it implements Unpin!
    /// We can use standard &mut self view_mut_unpin().
    pub fn give_item(&mut self, slot_index: usize, item_id: u32) -> Result<(), &'static str> {
        let v = self.view_mut_unpin();
        if slot_index >= v.inventory_slots.len() {
            return Err("Slot out of bounds");
        }
        v.inventory_slots[slot_index] = item_id;
        Ok(())
    }
}

#[test]
fn test_shared_layout_impl() {
    let table = PlayerState::make_table(2, 5);
    
    // Init player 1
    let mut p1 = PlayerState::init_with_table(table.clone(), 99, 0, 0).in_box();
    
    // Grant admin permission manually
    p1.view_mut_unpin().permissions[1] = 0xFF; 

    // 1. Immutable call
    assert!(p1.has_permission(0xFF));
    assert!(!p1.has_permission(0x01));

    // 2. Mutable call WITHOUT needing Pin!
    assert!(p1.give_item(0, 1001).is_ok());
    assert!(p1.give_item(4, 9999).is_ok());
    assert!(p1.give_item(5, 1234).is_err()); // Out of bounds

    assert_eq!(p1.view().inventory_slots[0], 1001);
    assert_eq!(p1.view().inventory_slots[4], 9999);
}
