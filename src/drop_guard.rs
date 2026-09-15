use core::marker::PhantomData;

/// A RAII guard that drops `written` already-initialized elements of a raw
/// slice pointer if the guarded scope panics before `forget()` is called.
///
/// Used internally by the `.init()` constructor to prevent memory leaks
/// when an iterator panics mid-write.
#[doc(hidden)]
pub struct __DropGuard<T> {
    pub ptr: *mut T,
    pub written: usize,
    _marker: PhantomData<T>,
}

impl<T> __DropGuard<T> {
    #[inline]
    pub fn new(ptr: *mut T) -> Self {
        Self { ptr, written: 0, _marker: PhantomData }
    }
}

impl<T> Drop for __DropGuard<T> {
    fn drop(&mut self) {
        // SAFETY: `ptr..ptr+written` were all initialized before the panic.
        unsafe {
            core::ptr::drop_in_place(core::ptr::slice_from_raw_parts_mut(self.ptr, self.written));
        }
    }
}

unsafe impl<T: Send> Send for __DropGuard<T> {}
unsafe impl<T: Sync> Sync for __DropGuard<T> {}
