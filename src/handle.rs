use core::marker::{PhantomData, PhantomPinned};
use core::pin::Pin;
use core::ptr::NonNull;

/// A handle to an inline slice dynamically placed within a `#[slice_struct]`.
///
/// This field is dynamically initialized by the macro-generated constructors.
/// Because it contains internal pointers to the struct's own allocation,
/// this type is `!Unpin` and safely accessible via projection.
pub struct SliceHandle<T> {
    ptr: NonNull<T>,
    len: usize,
    _marker: PhantomData<T>,
    _pin: PhantomPinned,
}

unsafe impl<T: Send> Send for SliceHandle<T> {}
unsafe impl<T: Sync> Sync for SliceHandle<T> {}

impl<T> SliceHandle<T> {
    #[doc(hidden)]
    #[inline]
    pub fn __new_unchecked(ptr: *mut T, len: usize) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap(),
            len,
            _marker: PhantomData,
            _pin: PhantomPinned,
        }
    }

    /// The number of elements in the slice.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the slice is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Mutable access to the slice.
    ///
    /// Requires pinning to ensure the struct is not moved,
    /// which would invalidate the internal pointers.
    #[inline]
    pub fn as_mut_slice(self: Pin<&mut Self>) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }

    /// Retrieve the raw pointer to the slice without constructing a reference.
    #[inline]
    pub fn as_non_null(&self) -> NonNull<[T]> {
        NonNull::slice_from_raw_parts(self.ptr, self.len)
    }
}

impl<T> core::ops::Deref for SliceHandle<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> core::ops::Drop for SliceHandle<T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(core::ptr::slice_from_raw_parts_mut(
                self.ptr.as_ptr(),
                self.len,
            ));
        }
    }
}

/// A temporary borrow of an inline slice field, produced by `.view_mut()`.
///
/// Implements both [`core::ops::Deref`] and [`core::ops::DerefMut`] to `[T]`, so you can index,
/// iterate, or reborrow as `&[T]` or `&mut [T]` freely.
///
/// The key property: two `SliceBorrow`s from the same `.view_mut()` call are
/// **disjoint** — one can be borrowed immutably while another is borrowed
/// mutably, at the same time.
pub struct SliceBorrow<'a, T> {
    ptr: NonNull<T>,
    len: usize,
    _marker: PhantomData<&'a mut [T]>,
}

unsafe impl<'a, T: Send> Send for SliceBorrow<'a, T> {}
unsafe impl<'a, T: Sync> Sync for SliceBorrow<'a, T> {}

impl<'a, T> SliceBorrow<'a, T> {
    #[doc(hidden)]
    #[inline]
    /// # Safety
    /// Caller must hold a `Pin<&'a mut SliceHandle<T>>` and must not alias
    /// `ptr` for the duration of `'a`.
    pub unsafe fn __from_handle(handle: Pin<&'a mut SliceHandle<T>>) -> Self {
        let h = unsafe { handle.get_unchecked_mut() };
        Self { ptr: h.ptr, len: h.len, _marker: PhantomData }
    }
}

impl<'a, T> core::ops::Deref for SliceBorrow<'a, T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<'a, T> core::ops::DerefMut for SliceBorrow<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}
