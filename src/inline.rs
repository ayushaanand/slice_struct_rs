use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Mutex, MutexGuard};

/// Core trait to decouple a slice's state from its memory allocation elements.
pub trait InlineSlice {
    type Element;
    type State;
    type View<'a>
    where
        Self: 'a;
    type ViewMut<'a>
    where
        Self: 'a;

    fn init_state() -> Self::State;
    fn project<'a>(
        state: &'a Self::State,
        data: core::ptr::NonNull<[Self::Element]>,
    ) -> Self::View<'a>;
    fn project_mut<'a>(
        state: &'a Self::State,
        data: core::ptr::NonNull<[Self::Element]>,
    ) -> Self::ViewMut<'a>;

    /// Drops the dynamically-sized elements of the slice.
    unsafe fn drop_slice(state: &Self::State, data: core::ptr::NonNull<[Self::Element]>);
}

// ── Standard [T] ──────────────────────────────────────────────────────────

impl<T> InlineSlice for [T] {
    type Element = T;
    type State = ();
    type View<'a>
        = &'a [T]
    where
        Self: 'a;
    type ViewMut<'a>
        = &'a mut [T]
    where
        Self: 'a;

    #[inline]
    fn init_state() -> Self::State {
        
    }

    #[inline]
    fn project<'a>(
        _state: &'a Self::State,
        data: core::ptr::NonNull<[Self::Element]>,
    ) -> Self::View<'a> {
        unsafe { data.as_ref() }
    }

    #[inline]
    fn project_mut<'a>(
        _state: &'a Self::State,
        mut data: core::ptr::NonNull<[Self::Element]>,
    ) -> Self::ViewMut<'a> {
        unsafe { data.as_mut() }
    }

    #[inline]
    unsafe fn drop_slice(_state: &Self::State, data: core::ptr::NonNull<[Self::Element]>) {
        unsafe { core::ptr::drop_in_place(data.as_ptr()) };
    }
}

// ── str ───────────────────────────────────────────────────────────────────

impl InlineSlice for str {
    type Element = u8;
    type State = ();
    type View<'a>
        = &'a str
    where
        Self: 'a;
    type ViewMut<'a>
        = &'a mut str
    where
        Self: 'a;

    #[inline]
    fn init_state() -> Self::State {
        
    }

    #[inline]
    fn project<'a>(
        _state: &'a Self::State,
        data: core::ptr::NonNull<[Self::Element]>,
    ) -> Self::View<'a> {
        std::str::from_utf8(unsafe { data.as_ref() })
            .expect("slice_struct: invalid utf-8 in str payload")
    }

    #[inline]
    fn project_mut<'a>(
        _state: &'a Self::State,
        mut data: core::ptr::NonNull<[Self::Element]>,
    ) -> Self::ViewMut<'a> {
        std::str::from_utf8_mut(unsafe { data.as_mut() })
            .expect("slice_struct: invalid utf-8 in str payload")
    }

    #[inline]
    unsafe fn drop_slice(_state: &Self::State, _data: core::ptr::NonNull<[Self::Element]>) {}
}

// ── Mutex<[T]> ────────────────────────────────────────────────────────────

pub struct SliceMutexGuard<'a, T> {
    _guard: MutexGuard<'a, ()>,
    data: *mut [T],
    _marker: PhantomData<&'a mut [T]>,
}

unsafe impl<'a, T: Send> Send for SliceMutexGuard<'a, T> {}
unsafe impl<'a, T: Sync> Sync for SliceMutexGuard<'a, T> {}

impl<'a, T> core::ops::Deref for SliceMutexGuard<'a, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { &*self.data }
    }
}

impl<'a, T> core::ops::DerefMut for SliceMutexGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { &mut *self.data }
    }
}

impl<T> InlineSlice for Mutex<[T]> {
    type Element = T;
    type State = Mutex<()>;
    type View<'a>
        = SliceMutexGuard<'a, T>
    where
        Self: 'a;
    type ViewMut<'a>
        = SliceMutexGuard<'a, T>
    where
        Self: 'a;

    #[inline]
    fn init_state() -> Self::State {
        Mutex::new(())
    }

    #[inline]
    fn project<'a>(
        state: &'a Self::State,
        data: core::ptr::NonNull<[Self::Element]>,
    ) -> Self::View<'a> {
        SliceMutexGuard {
            _guard: state.lock().unwrap(),
            data: data.as_ptr(),
            _marker: PhantomData,
        }
    }

    #[inline]
    fn project_mut<'a>(
        state: &'a Self::State,
        data: core::ptr::NonNull<[Self::Element]>,
    ) -> Self::ViewMut<'a> {
        SliceMutexGuard {
            _guard: state.lock().unwrap(),
            data: data.as_ptr(),
            _marker: PhantomData,
        }
    }

    #[inline]
    unsafe fn drop_slice(_state: &Self::State, data: core::ptr::NonNull<[Self::Element]>) {
        unsafe { core::ptr::drop_in_place(data.as_ptr()) };
    }
}

// ── RefCell<[T]> ──────────────────────────────────────────────────────────

pub struct SliceRefGuard<'a, T> {
    _guard: RefMut<'a, ()>,
    data: *mut [T],
    _marker: PhantomData<&'a mut [T]>,
}

impl<'a, T> core::ops::Deref for SliceRefGuard<'a, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { &*self.data }
    }
}

impl<'a, T> core::ops::DerefMut for SliceRefGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { &mut *self.data }
    }
}

impl<T> InlineSlice for RefCell<[T]> {
    type Element = T;
    type State = RefCell<()>;
    type View<'a>
        = SliceRefGuard<'a, T>
    where
        Self: 'a;
    type ViewMut<'a>
        = SliceRefGuard<'a, T>
    where
        Self: 'a;

    #[inline]
    fn init_state() -> Self::State {
        RefCell::new(())
    }

    #[inline]
    fn project<'a>(
        state: &'a Self::State,
        data: core::ptr::NonNull<[Self::Element]>,
    ) -> Self::View<'a> {
        SliceRefGuard {
            _guard: state.borrow_mut(),
            data: data.as_ptr(),
            _marker: PhantomData,
        }
    }

    #[inline]
    fn project_mut<'a>(
        state: &'a Self::State,
        data: core::ptr::NonNull<[Self::Element]>,
    ) -> Self::ViewMut<'a> {
        SliceRefGuard {
            _guard: state.borrow_mut(),
            data: data.as_ptr(),
            _marker: PhantomData,
        }
    }

    #[inline]
    unsafe fn drop_slice(_state: &Self::State, data: core::ptr::NonNull<[Self::Element]>) {
        unsafe { core::ptr::drop_in_place(data.as_ptr()) };
    }
}
