#![allow(unsafe_op_in_unsafe_fn)]
use core::pin::Pin;
use core::ptr::NonNull;
use std::alloc::Layout;
use std::sync::Arc;
use std::rc::Rc;
use crate::wrappers::{DstMutex, WithMutex, DstRefCell, WithRefCell};

/// A heap-allocated, fully data-initialized, but NOT YET PINNED `slice_struct`.
///
/// The internal `SliceHandle` pointers have not been written. Because it contains
/// no self-referential pointers, this type is safely moveable.
pub struct OwnedDst<T: ?Sized> {
    pub ptr: NonNull<T>,
    pub layout: Layout,
}

impl<T: ?Sized> Drop for OwnedDst<T> {
    fn drop(&mut self) {
        unsafe {
            // Drop only the raw memory allocation, do NOT call T's destructor.
            // SliceHandles are uninitialized and would cause a crash.
            std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, self.layout);
        }
    }
}

/// Core trait implemented by the macro to initialize a `slice_struct`.
pub unsafe trait SliceInit<T: ?Sized> {
    /// Computes the exact memory layout required for the struct.
    fn layout(&self) -> Layout;

    /// Creates a fat pointer to `T` at the given base address.
    fn make_fat_ptr(&self, base: *mut u8) -> *mut T;

    /// Writes all user data to the allocated pointer, leaving internal pointers uninitialized.
    /// Returns an `OwnedDst` that safely guards the allocation until it is pinned.
    unsafe fn write_data(self, ptr: *mut u8) -> OwnedDst<T>;

    /// Writes the internal absolute pointers (e.g. `SliceHandle`) into the struct
    /// based on its final pinned memory location.
    unsafe fn fixup(ptr: *mut T);
}

/// A composable builder for allocating a `slice_struct`.
pub struct SliceBuilder<T: ?Sized, I: SliceInit<T>> {
    pub(crate) init: I,
    _marker: core::marker::PhantomData<T>,
}

struct RawAllocGuard {
    ptr: *mut u8,
    layout: Layout,
}

impl Drop for RawAllocGuard {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.ptr, self.layout);
        }
    }
}

impl<T: ?Sized, I: SliceInit<T>> SliceBuilder<T, I> {
    pub fn new(init: I) -> Self {
        Self { init, _marker: core::marker::PhantomData }
    }

    /// Allocates the struct into a `Box`.
    pub fn in_box(self) -> Pin<Box<T>> {
        let layout = self.init.layout();
        unsafe {
            let raw = std::alloc::alloc(layout);
            if raw.is_null() { std::alloc::handle_alloc_error(layout); }
            let alloc_guard = RawAllocGuard { ptr: raw, layout };
            
            let owned = self.init.write_data(raw);
            core::mem::forget(alloc_guard);
            
            let ptr = owned.ptr.as_ptr();
            core::mem::forget(owned);
            
            I::fixup(ptr);
            Pin::new_unchecked(Box::from_raw(ptr))
        }
    }

    /// Allocates the struct into an `Arc`, ensuring zero-cost safe initialization.
    pub fn in_arc(self) -> Pin<Arc<T>> {
        let layout = self.init.layout();
        unsafe {
            let raw = std::alloc::alloc(layout);
            if raw.is_null() { std::alloc::handle_alloc_error(layout); }
            let alloc_guard = RawAllocGuard { ptr: raw, layout };
            
            let owned = self.init.write_data(raw);
            core::mem::forget(alloc_guard);
            
            let ptr = owned.ptr.as_ptr();
            core::mem::forget(owned);
            
            let mut arc = Arc::from(Box::from_raw(ptr));
            let new_ptr = Arc::get_mut(&mut arc).unwrap() as *mut T;
            I::fixup(new_ptr);
            Pin::new_unchecked(arc)
        }
    }
    
    /// Allocates the struct into an `Rc`, ensuring zero-cost safe initialization.
    pub fn in_rc(self) -> Pin<Rc<T>> {
        let layout = self.init.layout();
        unsafe {
            let raw = std::alloc::alloc(layout);
            if raw.is_null() { std::alloc::handle_alloc_error(layout); }
            let alloc_guard = RawAllocGuard { ptr: raw, layout };
            
            let owned = self.init.write_data(raw);
            core::mem::forget(alloc_guard);
            
            let ptr = owned.ptr.as_ptr();
            core::mem::forget(owned);
            
            let mut rc = Rc::from(Box::from_raw(ptr));
            let new_ptr = Rc::get_mut(&mut rc).unwrap() as *mut T;
            I::fixup(new_ptr);
            Pin::new_unchecked(rc)
        }
    }
    
    /// Wraps the entire struct in a thread-safe `Mutex`, yielding `Arc<DstMutex<T>>`.
    /// 
    /// ```rust
    /// # use slice_struct::slice_struct;
    /// # #[slice_struct]
    /// # pub struct Packet { pub id: u32, #[slice] pub payload: u8 }
    /// let arc_mutex = Packet::init_def(1, (0, 10))
    ///     .with_mutex()
    ///     .in_arc();
    /// 
    /// let mut guard = arc_mutex.lock();
    /// guard.as_mut().view_mut().payload[0] = 99;
    /// ```
    pub fn with_mutex(self) -> SliceBuilder<DstMutex<T>, WithMutex<I>> {
        SliceBuilder::new(WithMutex(self.init))
    }
    
    /// Wraps the entire struct in a `RefCell`, yielding `Rc<DstRefCell<T>>`.
    /// 
    /// ```rust
    /// # use slice_struct::slice_struct;
    /// # #[slice_struct]
    /// # pub struct Packet { pub id: u32, #[slice] pub payload: u8 }
    /// let rc_refcell = Packet::init_def(1, (0, 10))
    ///     .with_refcell()
    ///     .in_rc();
    /// 
    /// let mut guard = rc_refcell.borrow_mut();
    /// guard.as_mut().view_mut().payload[0] = 99;
    /// ```
    pub fn with_refcell(self) -> SliceBuilder<DstRefCell<T>, WithRefCell<I>> {
        SliceBuilder::new(WithRefCell(self.init))
    }
}
