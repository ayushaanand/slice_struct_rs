use slice_struct::slice_struct;

#[slice_struct(shared_layout)]
pub struct Entity {
    pub id: u32,
    #[slice] pub positions: [f32],
    #[slice] pub velocities: [f32],
}

#[test]
fn test_shared_layout_basic() {
    let table = Entity::make_table(3, 3);
    
    assert_eq!(table.data.positions_len, 3);
    assert_eq!(table.data.velocities_len, 3);
    
    let mut e1 = Entity::init_with_table(table.clone(), 1, 0.0, 1.0).in_box();
    let mut e2 = Entity::init_with_table(table.clone(), 2, 5.0, 2.0).in_box();
    
    // Check lengths
    assert_eq!(e1.view().positions.len(), 3);
    assert_eq!(e2.view().velocities.len(), 3);
    
    // Check initial values
    assert_eq!(*e1.view().id, 1);
    assert_eq!(e1.view().positions[0], 0.0);
    assert_eq!(e1.view().velocities[2], 1.0);
    
    // Check metadata overhead
    // A standard Box is 16 bytes. Entity contains id (4 bytes) + padding (4 bytes) = 8 bytes.
    // Plus the Arc<Table> (8 bytes).
    // Plus the 2 slices: 6 * 4 = 24 bytes.
    // Total size should be 40 bytes exactly! (Instead of 56 bytes with inline handlers)
    
    // Wait, let's just make sure it compiles and runs without segfaults!
    
    e1.view_mut_unpin().positions[0] = 99.0;
    assert_eq!(e1.view().positions[0], 99.0);
    assert_eq!(e2.view().positions[0], 5.0); // Completely independent!
    
    assert_eq!(std::sync::Arc::strong_count(&table), 3);
    assert_eq!(std::mem::size_of_val(&*e1), 40); // 4 for id + 4 for padding + 8 for Arc + 24 for two [f32; 3] arrays // 1 local + 2 boxes
}

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct DropTracker {
    counter: Arc<AtomicUsize>,
}

impl Drop for DropTracker {
    fn drop(&mut self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
}

impl Clone for DropTracker {
    fn clone(&self) -> Self {
        Self {
            counter: self.counter.clone(),
        }
    }
}

#[slice_struct(shared_layout)]
pub struct ComplexEntity {
    pub name: String,
    #[slice] pub trackers: [DropTracker],
}

#[test]
fn test_shared_layout_drop() {
    let table = ComplexEntity::make_table(5);
    let counter = Arc::new(AtomicUsize::new(0));
    
    {
        let tracker = DropTracker { counter: counter.clone() };
        let e = ComplexEntity::init_with_table(table.clone(), "Test".to_string(), tracker).in_box();
        assert_eq!(e.view().trackers.len(), 5);
        assert_eq!(e.view().name, "Test");
        assert_eq!(counter.load(Ordering::SeqCst), 0); // None dropped yet
    }
    
    // The struct dropped, including its name and the 5 DropTrackers.
    // +1 drop from the local 	racker dropping at the end of the block? Wait, local 	racker drops at end of block.
    // Let's drop it explicitly before checking.
    // Actually, local tracker drops inside the block.  drops inside the block.
    // The total drops should be 1 (local) + 5 (in array) = 6.
    assert_eq!(counter.load(Ordering::SeqCst), 5);
}

#[test]
fn test_shared_layout_zero_len() {
    let table = Entity::make_table(0, 0);
    let e = Entity::init_with_table(table.clone(), 42, 0.0, 0.0).in_box();
    assert_eq!(e.view().positions.len(), 0);
    assert_eq!(e.view().velocities.len(), 0);
    assert_eq!(*e.view().id, 42);
    
    // Size should be 4 (id) + 4 (padding) + 8 (Arc) = 16 bytes.
    assert_eq!(std::mem::size_of_val(&*e), 16);
}

#[slice_struct(shared_layout)]
pub struct NoSlices {
    pub a: u32,
    pub b: f32,
}

#[test]
fn test_shared_layout_no_slices() {
    let table = NoSlices::make_table();
    let n = NoSlices::init_with_table(table.clone(), 42, 3.14).in_box();

    assert_eq!(*n.view().a, 42);
    assert_eq!(*n.view().b, 3.14);
    assert_eq!(std::mem::size_of_val(&*n), 16); // 8 bytes for Arc + 4 bytes for a + 4 bytes for b = 16 bytes
}
