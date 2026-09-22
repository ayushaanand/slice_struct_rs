use crate::inline::InlineSlice;
use core::marker::PhantomData;
use core::ptr::NonNull;

#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot be used as an ArenaElement inside an ArenaSlice",
    note = "Did you forget to add `#[slice_struct(arena)]` to the `{Self}` struct?"
)]
pub unsafe trait ArenaElement {
    type Arena: ArenaDescriptor;
    unsafe fn project_view<'a>(
        ptr: *const u8,
        arena: &'a Self::Arena,
    ) -> <Self::Arena as ArenaDescriptor>::View<'a>;
    unsafe fn project_view_mut<'a>(
        ptr: *mut u8,
        arena: &'a Self::Arena,
    ) -> <Self::Arena as ArenaDescriptor>::ViewMut<'a>;
}

pub trait ArenaDescriptor: Sized {
    type View<'a>
    where
        Self: 'a;
    type ViewMut<'a>
    where
        Self: 'a;
    type InitData;

    fn instance_size(&self) -> usize;
    fn instance_align() -> usize;

    /// Write one instance in-place at `ptr`. Caller guarantees `ptr` is valid and aligned.
    unsafe fn write_instance_def(&self, ptr: *mut u8, data: &Self::InitData);
    unsafe fn drop_instance(&self, ptr: *mut u8);
}

pub struct ArenaSlice<T: ArenaElement + ?Sized>(PhantomData<T>);

pub struct ArenaSliceView<'a, T: ArenaElement + ?Sized + 'a> {
    pub base: *const u8,
    pub count: usize,
    pub arena: &'a T::Arena,
}

impl<'a, T: ArenaElement + ?Sized> ArenaSliceView<'a, T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.count
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<<T::Arena as ArenaDescriptor>::View<'a>> {
        if i >= self.count {
            return None;
        }
        let ptr = unsafe { self.base.add(i * self.arena.instance_size()) };
        Some(unsafe { T::project_view(ptr, self.arena) })
    }

    #[inline]
    pub fn at(&self, i: usize) -> <T::Arena as ArenaDescriptor>::View<'a> {
        self.get(i).expect("index out of bounds")
    }
}

pub struct ArenaSliceViewMut<'a, T: ArenaElement + ?Sized + 'a> {
    pub base: *mut u8,
    pub count: usize,
    pub arena: &'a T::Arena,
}

impl<'a, T: ArenaElement + ?Sized> ArenaSliceViewMut<'a, T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.count
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    #[inline]
    pub fn get_mut(&mut self, i: usize) -> Option<<T::Arena as ArenaDescriptor>::ViewMut<'_>> {
        if i >= self.count {
            return None;
        }
        let ptr = unsafe { self.base.add(i * self.arena.instance_size()) };
        Some(unsafe { T::project_view_mut(ptr, self.arena) })
    }

    #[inline]
    pub fn at_mut(&mut self, i: usize) -> <T::Arena as ArenaDescriptor>::ViewMut<'_> {
        self.get_mut(i).expect("index out of bounds")
    }
}

impl<T: ArenaElement + ?Sized> InlineSlice for ArenaSlice<T> {
    type Element = u8;
    type State = T::Arena;
    type View<'a>
        = ArenaSliceView<'a, T>
    where
        Self: 'a;
    type ViewMut<'a>
        = ArenaSliceViewMut<'a, T>
    where
        Self: 'a;

    #[inline]
    fn init_state() -> Self::State {
        unreachable!("Arena slices must be initialized with an explicit arena")
    }

    #[inline]
    fn project<'a>(state: &'a Self::State, data: NonNull<[Self::Element]>) -> Self::View<'a> {
        ArenaSliceView {
            base: data.as_ptr() as *const u8,
            count: data.len(),
            arena: state,
        }
    }

    #[inline]
    fn project_mut<'a>(
        state: &'a Self::State,
        data: NonNull<[Self::Element]>,
    ) -> Self::ViewMut<'a> {
        ArenaSliceViewMut {
            base: data.as_ptr() as *mut u8,
            count: data.len(),
            arena: state,
        }
    }

    #[inline]
    unsafe fn drop_slice(state: &Self::State, data: core::ptr::NonNull<[Self::Element]>) {
        let count = data.len();
        let base = data.as_ptr() as *mut u8;
        let stride = state.instance_size();
        for i in 0..count {
            unsafe {
                state.drop_instance(base.add(i * stride));
            }
        }
    }
}
