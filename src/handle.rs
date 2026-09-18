use core::marker::PhantomData;
use core::marker::PhantomPinned;
use core::ptr::NonNull;

pub trait AddressingMode {
    type Marker;
    const MARKER_INIT: Self::Marker;

    type PointerData<T>;

    fn store<T>(base_ptr: *mut u8, offset: usize) -> Self::PointerData<T>;
    fn resolve<T>(data: &Self::PointerData<T>, base_ptr: *const u8) -> NonNull<T>;
    fn dummy<T>() -> Self::PointerData<T>;
}

pub struct AbsoluteMode;
impl AddressingMode for AbsoluteMode {
    type Marker = PhantomPinned;
    const MARKER_INIT: Self::Marker = PhantomPinned;
    type PointerData<T> = NonNull<T>;

    #[inline(always)]
    fn store<T>(base_ptr: *mut u8, offset: usize) -> NonNull<T> {
        unsafe { NonNull::new_unchecked(base_ptr.add(offset) as *mut T) }
    }

    #[inline(always)]
    fn resolve<T>(data: &NonNull<T>, _base_ptr: *const u8) -> NonNull<T> {
        *data
    }

    #[inline(always)]
    fn dummy<T>() -> NonNull<T> {
        NonNull::dangling()
    }
}

pub struct RelativeMode;
impl AddressingMode for RelativeMode {
    type Marker = ();
    const MARKER_INIT: Self::Marker = ();
    type PointerData<T> = usize;

    #[inline(always)]
    fn store<T>(_base_ptr: *mut u8, offset: usize) -> usize {
        offset
    }

    #[inline(always)]
    fn resolve<T>(data: &usize, base_ptr: *const u8) -> NonNull<T> {
        unsafe { NonNull::new_unchecked(base_ptr.add(*data) as *mut T) }
    }

    #[inline(always)]
    fn dummy<T>() -> usize {
        0
    }
}

pub struct SliceHandle<T, Mode: AddressingMode> {
    pub ptr_data: Mode::PointerData<T>,
    pub len: usize,
    pub _marker: PhantomData<T>,
}

unsafe impl<T: Send, Mode: AddressingMode> Send for SliceHandle<T, Mode> {}
unsafe impl<T: Sync, Mode: AddressingMode> Sync for SliceHandle<T, Mode> {}

impl<T, Mode: AddressingMode> SliceHandle<T, Mode> {
    #[doc(hidden)]
    #[inline]
    pub fn __new_unchecked(ptr_data: Mode::PointerData<T>, len: usize) -> Self {
        Self { ptr_data, len, _marker: PhantomData }
    }

    #[inline(always)]
    pub fn as_non_null(&self, base_ptr: *const u8) -> NonNull<[T]> {
        NonNull::slice_from_raw_parts(Mode::resolve(&self.ptr_data, base_ptr), self.len)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }
    
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// SliceBorrow restores safe disjoint mutable borrowing
pub struct SliceBorrow<'a, T> {
    ptr: NonNull<[T]>,
    _marker: PhantomData<&'a mut [T]>,
}

unsafe impl<'a, T: Send> Send for SliceBorrow<'a, T> {}
unsafe impl<'a, T: Sync> Sync for SliceBorrow<'a, T> {}

impl<'a, T> SliceBorrow<'a, T> {
    #[inline(always)]
    pub fn new(ptr: NonNull<[T]>) -> Self {
        Self { ptr, _marker: PhantomData }
    }
}

impl<'a, T> core::ops::Deref for SliceBorrow<'a, T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { self.ptr.as_ref() }
    }
}

impl<'a, T> core::ops::DerefMut for SliceBorrow<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { self.ptr.as_mut() }
    }
}
