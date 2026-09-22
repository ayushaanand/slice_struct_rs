use core::alloc::Layout;

/// Trait implemented by structs that use #[slice_struct(shared_layout)].
pub trait SharedLayout {
    #[doc(hidden)]
    type LayoutData: SharedLayoutData;
}

/// Internal trait for the macro-generated layout data.
#[doc(hidden)]
pub trait SharedLayoutData {
    fn total_layout(&self) -> Layout;
}

/// A shared layout table for a specific slice_struct.
///
/// This table caches the lengths and memory offsets of all slice fields 
/// so they can be perfectly shared across thousands of standalone allocations.
pub struct LayoutTable<T: SharedLayout + ?Sized> {
    #[doc(hidden)]
    pub data: T::LayoutData,
}

impl<T: SharedLayout + ?Sized> LayoutTable<T> {
    #[inline]
    #[doc(hidden)]
    pub fn total_layout(&self) -> Layout {
        self.data.total_layout()
    }
}
