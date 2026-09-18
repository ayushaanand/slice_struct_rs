#![feature(prelude_import)]
#![allow(dead_code)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use slice_struct::slice_struct;
#[repr(C)]
#[doc(hidden)]
struct Str__Inner<T: Clone> {
    #[doc(hidden)]
    __a: u32,
    #[doc(hidden)]
    __b_state: <[T] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[T] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __b: ::slice_struct::__private::SliceHandle<
        <[T] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __c_state: <[u32] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_1: [<[u32] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __c: ::slice_struct::__private::SliceHandle<
        <[u32] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct Str<T: Clone>(Str__Inner<T>);
impl<T: Clone> ::core::ops::Drop for Str<T> {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__b.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__c.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct StrView<'__a, T: Clone>
where
    [T]: '__a,
    [u32]: '__a,
{
    a: &'__a u32,
    b: <[T] as ::slice_struct::__private::InlineSlice>::View<'__a>,
    c: <[u32] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct StrViewMut<'__a, T: Clone>
where
    [T]: '__a,
    [u32]: '__a,
{
    a: &'__a mut u32,
    b: <[T] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
    c: <[u32] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl<T: Clone> ::slice_struct::AsView for Str<T> {
    type View<'__a> = StrView<'__a, T> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        StrView {
            a: &this.0.__a,
            b: <[T] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__b_state,
                this.0.__b.as_non_null(base_ptr),
            ),
            c: <[u32] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__c_state,
                this.0.__c.as_non_null(base_ptr),
            ),
        }
    }
}
impl<T: Clone> ::slice_struct::AsViewMut for Str<T> {
    type ViewMut<'__a> = StrViewMut<'__a, T> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            StrViewMut {
                a: &mut this.0.__a,
                b: <[T] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__b_state,
                    this.0.__b.as_non_null(base_ptr),
                ),
                c: <[u32] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__c_state,
                    this.0.__c.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl<T: Clone> Str<T> {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> StrView<'__a, T> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> StrViewMut<'__a, T> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct Str_SizedPrefix<T: Clone> {
        #[doc(hidden)]
        __a: u32,
        #[doc(hidden)]
        __b_state: <[T] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[T] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __b: ::slice_struct::__private::SliceHandle<
            <[T] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __c_state: <[u32] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_1: [<[u32] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __c: ::slice_struct::__private::SliceHandle<
            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct Str_LayoutHelper<T: Clone>(::core::marker::PhantomData<Str<T>>);
    impl<T: Clone> Str_LayoutHelper<T> {
        fn calculate_layout(
            b_len: usize,
            c_len: usize,
        ) -> (::std::alloc::Layout, usize, usize) {
            let layout = ::std::alloc::Layout::new::<Str_SizedPrefix<T>>();
            let (layout, b_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[T] as ::slice_struct::__private::InlineSlice>::Element,
                    >(b_len)
                        .unwrap(),
                )
                .unwrap();
            let (layout, c_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                    >(c_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), b_offset, c_offset)
        }
    }
    #[doc(hidden)]
    pub struct __StrInitIter<T: Clone, __I0, __I1> {
        pub a: u32,
        pub b: __I0,
        pub c: __I1,
        _marker: ::core::marker::PhantomData<Str<T>>,
    }
    unsafe impl<T: Clone, __I0, __I1> ::slice_struct::__private::SliceInit<Str<T>>
    for __StrInitIter<T, __I0, __I1>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[T] as ::slice_struct::__private::InlineSlice>::Element,
        >,
        __I1: ::core::iter::ExactSizeIterator<
            Item = <[u32] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let b_len = self.b.len();
            let c_len = self.c.len();
            Str_LayoutHelper::<T>::calculate_layout(b_len, c_len).0
        }
        unsafe fn fixup(ptr: *mut Str<T>) {
            let (_, b_offset, c_offset) = Str_LayoutHelper::<
                T,
            >::calculate_layout((*ptr).0.__b.len, (*ptr).0.__c.len);
            ::core::ptr::write(
                &raw mut (*ptr).0.__b.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[T] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, b_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__c.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, c_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut Str<T> {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<Str_SizedPrefix<T>>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<Str<T>> {
            let b_len = self.b.len();
            let c_len = self.c.len();
            let (layout, b_offset, c_offset) = Str_LayoutHelper::<
                T,
            >::calculate_layout(b_len, c_len);
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<Str_SizedPrefix<T>>();
            ::core::ptr::write(
                sized_ptr,
                Str_SizedPrefix {
                    __a: self.a,
                    __align_0: [],
                    __align_1: [],
                    __b_state: <[T] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __b: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[T] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: b_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __c_state: <[u32] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __c: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: c_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(b_offset)
                .cast::<<[T] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.b.into_iter();
            for j in 0..b_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            let field_ptr = ptr
                .add(c_offset)
                .cast::<<[u32] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.c.into_iter();
            for j in 0..c_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __StrInitDef<T: Clone> {
        pub a: u32,
        pub b: (<[T] as ::slice_struct::__private::InlineSlice>::Element, usize),
        pub c: (<[u32] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<Str<T>>,
    }
    unsafe impl<T: Clone> ::slice_struct::__private::SliceInit<Str<T>>
    for __StrInitDef<T>
    where
        <[T] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        <[u32] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let b_len = self.b.1;
            let c_len = self.c.1;
            Str_LayoutHelper::<T>::calculate_layout(b_len, c_len).0
        }
        unsafe fn fixup(ptr: *mut Str<T>) {
            let (_, b_offset, c_offset) = Str_LayoutHelper::<
                T,
            >::calculate_layout((*ptr).0.__b.len, (*ptr).0.__c.len);
            ::core::ptr::write(
                &raw mut (*ptr).0.__b.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[T] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, b_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__c.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, c_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut Str<T> {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<Str_SizedPrefix<T>>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<Str<T>> {
            let b_len = self.b.1;
            let c_len = self.c.1;
            let (layout, b_offset, c_offset) = Str_LayoutHelper::<
                T,
            >::calculate_layout(b_len, c_len);
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<Str_SizedPrefix<T>>();
            ::core::ptr::write(
                sized_ptr,
                Str_SizedPrefix {
                    __a: self.a,
                    __align_0: [],
                    __align_1: [],
                    __b_state: <[T] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __b: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[T] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: b_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __c_state: <[u32] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __c: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: c_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.b.1;
            let field_ptr = ptr
                .add(b_offset)
                .cast::<<[T] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.b.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            let def_len = self.c.1;
            let field_ptr = ptr
                .add(c_offset)
                .cast::<<[u32] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.c.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl<T: Clone> Str<T> {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0, __I1>(
            a: u32,
            mut b: __I0,
            mut c: __I1,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[T] as ::slice_struct::__private::InlineSlice>::Element,
            >,
            __I1: ::core::iter::ExactSizeIterator<
                Item = <[u32] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__StrInitIter {
                a,
                b,
                c,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            a: u32,
            b: (<[T] as ::slice_struct::__private::InlineSlice>::Element, usize),
            c: (<[u32] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[T] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
            <[u32] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__StrInitDef {
                a,
                b,
                c,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_iter_basic"]
#[doc(hidden)]
pub const test_iter_basic: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_iter_basic"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 14usize,
        start_col: 4usize,
        end_line: 14usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_iter_basic()),
    ),
};
fn test_iter_basic() {
    let mut s = Str::init_iter(42, [1i32, 2, 3].into_iter(), [4, 5, 6, 7].into_iter())
        .in_box();
    (/*ERROR*/)
}
extern crate test;
#[rustc_test_marker = "test_mixed_borrow"]
#[doc(hidden)]
pub const test_mixed_borrow: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_mixed_borrow"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 30usize,
        start_col: 4usize,
        end_line: 30usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_mixed_borrow()),
    ),
};
fn test_mixed_borrow() {
    let mut s = Str::init_iter(
            0,
            [10_i32, 20, 30].into_iter(),
            [1_u32, 2, 3].into_iter(),
        )
        .in_box();
    let v = s.as_mut().view_mut();
    let b_ref: &[i32] = &v.b;
    v.c[0] = b_ref[2] as u32 * 10;
    match (&s.view().b, &&[10, 20, 30]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().c, &&[300, 2, 3]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_iter_from_range"]
#[doc(hidden)]
pub const test_iter_from_range: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_iter_from_range"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 42usize,
        start_col: 4usize,
        end_line: 42usize,
        end_col: 24usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_iter_from_range()),
    ),
};
fn test_iter_from_range() {
    let s = Str::<i32>::init_iter(0, (0..5).map(|x| x * x), 100_u32..104).in_box();
    match (&s.view().b, &&[0, 1, 4, 9, 16]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().c, &&[100, 101, 102, 103]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_iter_empty_slice"]
#[doc(hidden)]
pub const test_iter_empty_slice: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_iter_empty_slice"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 49usize,
        start_col: 4usize,
        end_line: 49usize,
        end_col: 25usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_iter_empty_slice()),
    ),
};
fn test_iter_empty_slice() {
    let s = Str::<i32>::init_iter(99, [].into_iter(), [].into_iter()).in_box();
    match (&*s.view().a, &99) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().b, &&[]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().c, &&[]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_def_basic"]
#[doc(hidden)]
pub const test_def_basic: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_def_basic"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 59usize,
        start_col: 4usize,
        end_line: 59usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_def_basic()),
    ),
};
fn test_def_basic() {
    let s = Str::<i32>::init_def(1, (0, 4), (7, 3)).in_box();
    match (&*s.view().a, &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().b, &&[0, 0, 0, 0]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().c, &&[7, 7, 7]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_def_zero_len"]
#[doc(hidden)]
pub const test_def_zero_len: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_def_zero_len"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 67usize,
        start_col: 4usize,
        end_line: 67usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_def_zero_len()),
    ),
};
fn test_def_zero_len() {
    let s = Str::<i32>::init_def(5, (99, 0), (42, 0)).in_box();
    match (&s.view().b, &&[]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().c, &&[]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(C)]
#[doc(hidden)]
struct SStr__Inner {
    #[doc(hidden)]
    __a: String,
    #[doc(hidden)]
    __b_state: <[String] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[String] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __b: ::slice_struct::__private::SliceHandle<
        <[String] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct SStr(SStr__Inner);
impl ::core::ops::Drop for SStr {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__b.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct SStrView<'__a>
where
    [String]: '__a,
{
    a: &'__a String,
    b: <[String] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct SStrViewMut<'__a>
where
    [String]: '__a,
{
    a: &'__a mut String,
    b: <[String] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for SStr {
    type View<'__a> = SStrView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        SStrView {
            a: &this.0.__a,
            b: <[String] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__b_state,
                this.0.__b.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for SStr {
    type ViewMut<'__a> = SStrViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            SStrViewMut {
                a: &mut this.0.__a,
                b: <[String] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__b_state,
                    this.0.__b.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl SStr {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> SStrView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> SStrViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct SStr_SizedPrefix {
        #[doc(hidden)]
        __a: String,
        #[doc(hidden)]
        __b_state: <[String] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[String] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __b: ::slice_struct::__private::SliceHandle<
            <[String] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct SStr_LayoutHelper(::core::marker::PhantomData<SStr>);
    impl SStr_LayoutHelper {
        fn calculate_layout(b_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<SStr_SizedPrefix>();
            let (layout, b_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[String] as ::slice_struct::__private::InlineSlice>::Element,
                    >(b_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), b_offset)
        }
    }
    #[doc(hidden)]
    pub struct __SStrInitIter<__I0> {
        pub a: String,
        pub b: __I0,
        _marker: ::core::marker::PhantomData<SStr>,
    }
    unsafe impl<__I0> ::slice_struct::__private::SliceInit<SStr> for __SStrInitIter<__I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[String] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let b_len = self.b.len();
            SStr_LayoutHelper::calculate_layout(b_len).0
        }
        unsafe fn fixup(ptr: *mut SStr) {
            let (_, b_offset) = SStr_LayoutHelper::calculate_layout((*ptr).0.__b.len);
            ::core::ptr::write(
                &raw mut (*ptr).0.__b.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[String] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, b_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut SStr {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<SStr_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<SStr> {
            let b_len = self.b.len();
            let (layout, b_offset) = SStr_LayoutHelper::calculate_layout(b_len);
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<SStr_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                SStr_SizedPrefix {
                    __a: self.a,
                    __align_0: [],
                    __b_state: <[String] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __b: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[String] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: b_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(b_offset)
                .cast::<<[String] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.b.into_iter();
            for j in 0..b_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __SStrInitDef {
        pub a: String,
        pub b: (<[String] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<SStr>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<SStr> for __SStrInitDef
    where
        <[String] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let b_len = self.b.1;
            SStr_LayoutHelper::calculate_layout(b_len).0
        }
        unsafe fn fixup(ptr: *mut SStr) {
            let (_, b_offset) = SStr_LayoutHelper::calculate_layout((*ptr).0.__b.len);
            ::core::ptr::write(
                &raw mut (*ptr).0.__b.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[String] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, b_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut SStr {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<SStr_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<SStr> {
            let b_len = self.b.1;
            let (layout, b_offset) = SStr_LayoutHelper::calculate_layout(b_len);
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<SStr_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                SStr_SizedPrefix {
                    __a: self.a,
                    __align_0: [],
                    __b_state: <[String] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __b: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[String] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: b_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.b.1;
            let field_ptr = ptr
                .add(b_offset)
                .cast::<<[String] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.b.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl SStr {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            a: String,
            mut b: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[String] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__SStrInitIter {
                a,
                b,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            a: String,
            b: (<[String] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[String] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__SStrInitDef {
                a,
                b,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_iter_drop"]
#[doc(hidden)]
pub const test_iter_drop: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_iter_drop"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 82usize,
        start_col: 4usize,
        end_line: 82usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_iter_drop()),
    ),
};
fn test_iter_drop() {
    let words = ["hello".to_string(), "world".to_string()];
    let s = SStr::init_iter("first".to_string(), words.into_iter()).in_box();
    match (&s.view().a, &"first") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().b, &&["hello", "world"]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_def_drop"]
#[doc(hidden)]
pub const test_def_drop: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_def_drop"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 90usize,
        start_col: 4usize,
        end_line: 90usize,
        end_col: 17usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_def_drop()),
    ),
};
fn test_def_drop() {
    let s = SStr::init_def("first".to_string(), ("hello".to_string(), 3)).in_box();
    match (&s.view().b, &&["hello", "hello", "hello"]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(C)]
#[doc(hidden)]
struct NoSlices__Inner {
    #[doc(hidden)]
    __a: u32,
    #[doc(hidden)]
    __b: bool,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct NoSlices(NoSlices__Inner);
impl ::core::ops::Drop for NoSlices {
    fn drop(&mut self) {}
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct NoSlicesView<'__a> {
    pub a: &'__a u32,
    pub b: &'__a bool,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct NoSlicesViewMut<'__a> {
    pub a: &'__a mut u32,
    pub b: &'__a mut bool,
}
impl ::slice_struct::AsView for NoSlices {
    type View<'__a> = NoSlicesView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        NoSlicesView {
            a: &this.0.__a,
            b: &this.0.__b,
        }
    }
}
impl ::slice_struct::AsViewMut for NoSlices {
    type ViewMut<'__a> = NoSlicesViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            NoSlicesViewMut {
                a: &mut this.0.__a,
                b: &mut this.0.__b,
            }
        }
    }
}
impl NoSlices {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> NoSlicesView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> NoSlicesViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct NoSlices_SizedPrefix {
        #[doc(hidden)]
        __a: u32,
        #[doc(hidden)]
        __b: bool,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct NoSlices_LayoutHelper(::core::marker::PhantomData<NoSlices>);
    impl NoSlices_LayoutHelper {
        fn calculate_layout() -> (::std::alloc::Layout,) {
            let layout = ::std::alloc::Layout::new::<NoSlices_SizedPrefix>();
            (layout.pad_to_align(),)
        }
    }
    #[doc(hidden)]
    pub struct __NoSlicesInitIter {
        pub a: u32,
        pub b: bool,
        _marker: ::core::marker::PhantomData<NoSlices>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<NoSlices> for __NoSlicesInitIter {
        fn layout(&self) -> ::std::alloc::Layout {
            NoSlices_LayoutHelper::calculate_layout().0
        }
        unsafe fn fixup(ptr: *mut NoSlices) {
            let (_,) = NoSlices_LayoutHelper::calculate_layout();
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut NoSlices {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<NoSlices_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<NoSlices> {
            let (layout,) = NoSlices_LayoutHelper::calculate_layout();
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<NoSlices_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                NoSlices_SizedPrefix {
                    __a: self.a,
                    __b: self.b,
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __NoSlicesInitDef {
        pub a: u32,
        pub b: bool,
        _marker: ::core::marker::PhantomData<NoSlices>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<NoSlices> for __NoSlicesInitDef {
        fn layout(&self) -> ::std::alloc::Layout {
            NoSlices_LayoutHelper::calculate_layout().0
        }
        unsafe fn fixup(ptr: *mut NoSlices) {
            let (_,) = NoSlices_LayoutHelper::calculate_layout();
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut NoSlices {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<NoSlices_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<NoSlices> {
            let (layout,) = NoSlices_LayoutHelper::calculate_layout();
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<NoSlices_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                NoSlices_SizedPrefix {
                    __a: self.a,
                    __b: self.b,
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl NoSlices {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter(
            a: u32,
            b: bool,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        > {
            ::slice_struct::SliceBuilder::new(__NoSlicesInitIter {
                a,
                b,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            a: u32,
            b: bool,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        > {
            ::slice_struct::SliceBuilder::new(__NoSlicesInitDef {
                a,
                b,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_no_slices_iter"]
#[doc(hidden)]
pub const test_no_slices_iter: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_no_slices_iter"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 102usize,
        start_col: 4usize,
        end_line: 102usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_no_slices_iter()),
    ),
};
fn test_no_slices_iter() {
    let mut s = NoSlices::init_iter(42, true).in_box();
    match (&*s.view().a, &42) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&*s.view().b, &true) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let v = s.as_mut().view_mut();
    *v.a = 99;
    match (&*s.view().a, &99) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_no_slices_def"]
#[doc(hidden)]
pub const test_no_slices_def: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_no_slices_def"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 113usize,
        start_col: 4usize,
        end_line: 113usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_no_slices_def()),
    ),
};
fn test_no_slices_def() {
    let s = NoSlices::init_def(42, true).in_box();
    match (&*s.view().a, &42) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&*s.view().b, &true) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(C)]
#[doc(hidden)]
struct OneSlice__Inner {
    #[doc(hidden)]
    __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[u8] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __data: ::slice_struct::__private::SliceHandle<
        <[u8] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct OneSlice(OneSlice__Inner);
impl ::core::ops::Drop for OneSlice {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__data.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct OneSliceView<'__a>
where
    [u8]: '__a,
{
    pub data: <[u8] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct OneSliceViewMut<'__a>
where
    [u8]: '__a,
{
    pub data: <[u8] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for OneSlice {
    type View<'__a> = OneSliceView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        OneSliceView {
            data: <[u8] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__data_state,
                this.0.__data.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for OneSlice {
    type ViewMut<'__a> = OneSliceViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            OneSliceViewMut {
                data: <[u8] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__data_state,
                    this.0.__data.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl OneSlice {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> OneSliceView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> OneSliceViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct OneSlice_SizedPrefix {
        #[doc(hidden)]
        __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[u8] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __data: ::slice_struct::__private::SliceHandle<
            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct OneSlice_LayoutHelper(::core::marker::PhantomData<OneSlice>);
    impl OneSlice_LayoutHelper {
        fn calculate_layout(data_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<OneSlice_SizedPrefix>();
            let (layout, data_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                    >(data_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), data_offset)
        }
    }
    #[doc(hidden)]
    pub struct __OneSliceInitIter<__I0> {
        pub data: __I0,
        _marker: ::core::marker::PhantomData<OneSlice>,
    }
    unsafe impl<__I0> ::slice_struct::__private::SliceInit<OneSlice>
    for __OneSliceInitIter<__I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[u8] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.len();
            OneSlice_LayoutHelper::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut OneSlice) {
            let (_, data_offset) = OneSlice_LayoutHelper::calculate_layout(
                (*ptr).0.__data.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut OneSlice {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<OneSlice_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<OneSlice> {
            let data_len = self.data.len();
            let (layout, data_offset) = OneSlice_LayoutHelper::calculate_layout(
                data_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<OneSlice_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                OneSlice_SizedPrefix {
                    __align_0: [],
                    __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(data_offset)
                .cast::<<[u8] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.data.into_iter();
            for j in 0..data_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __OneSliceInitDef {
        pub data: (<[u8] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<OneSlice>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<OneSlice> for __OneSliceInitDef
    where
        <[u8] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.1;
            OneSlice_LayoutHelper::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut OneSlice) {
            let (_, data_offset) = OneSlice_LayoutHelper::calculate_layout(
                (*ptr).0.__data.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut OneSlice {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<OneSlice_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<OneSlice> {
            let data_len = self.data.1;
            let (layout, data_offset) = OneSlice_LayoutHelper::calculate_layout(
                data_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<OneSlice_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                OneSlice_SizedPrefix {
                    __align_0: [],
                    __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.data.1;
            let field_ptr = ptr
                .add(data_offset)
                .cast::<<[u8] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.data.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl OneSlice {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            mut data: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[u8] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__OneSliceInitIter {
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            data: (<[u8] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[u8] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__OneSliceInitDef {
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_only_one_slice_iter"]
#[doc(hidden)]
pub const test_only_one_slice_iter: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_only_one_slice_iter"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 125usize,
        start_col: 4usize,
        end_line: 125usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_only_one_slice_iter()),
    ),
};
fn test_only_one_slice_iter() {
    let mut s = OneSlice::init_iter([10, 20].into_iter()).in_box();
    match (&s.view().data, &&[10, 20]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let v = s.as_mut().view_mut();
    v.data[1] = 99;
    match (&s.view().data, &&[10, 99]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_only_one_slice_def"]
#[doc(hidden)]
pub const test_only_one_slice_def: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_only_one_slice_def"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 135usize,
        start_col: 4usize,
        end_line: 135usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_only_one_slice_def()),
    ),
};
fn test_only_one_slice_def() {
    let s = OneSlice::init_def((5, 3)).in_box();
    match (&s.view().data, &&[5, 5, 5]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(C)]
#[doc(hidden)]
struct MixedEmpty__Inner {
    #[doc(hidden)]
    __a_state: <[u8] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[u8] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __a: ::slice_struct::__private::SliceHandle<
        <[u8] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __b_state: <[u16] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_1: [<[u16] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __b: ::slice_struct::__private::SliceHandle<
        <[u16] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __c_state: <[u32] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_2: [<[u32] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __c: ::slice_struct::__private::SliceHandle<
        <[u32] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct MixedEmpty(MixedEmpty__Inner);
impl ::core::ops::Drop for MixedEmpty {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__a.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__b.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__c.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct MixedEmptyView<'__a>
where
    [u8]: '__a,
    [u16]: '__a,
    [u32]: '__a,
{
    a: <[u8] as ::slice_struct::__private::InlineSlice>::View<'__a>,
    b: <[u16] as ::slice_struct::__private::InlineSlice>::View<'__a>,
    c: <[u32] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct MixedEmptyViewMut<'__a>
where
    [u8]: '__a,
    [u16]: '__a,
    [u32]: '__a,
{
    a: <[u8] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
    b: <[u16] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
    c: <[u32] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for MixedEmpty {
    type View<'__a> = MixedEmptyView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        MixedEmptyView {
            a: <[u8] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__a_state,
                this.0.__a.as_non_null(base_ptr),
            ),
            b: <[u16] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__b_state,
                this.0.__b.as_non_null(base_ptr),
            ),
            c: <[u32] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__c_state,
                this.0.__c.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for MixedEmpty {
    type ViewMut<'__a> = MixedEmptyViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            MixedEmptyViewMut {
                a: <[u8] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__a_state,
                    this.0.__a.as_non_null(base_ptr),
                ),
                b: <[u16] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__b_state,
                    this.0.__b.as_non_null(base_ptr),
                ),
                c: <[u32] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__c_state,
                    this.0.__c.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl MixedEmpty {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> MixedEmptyView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(
        self: ::core::pin::Pin<&'__a mut Self>,
    ) -> MixedEmptyViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct MixedEmpty_SizedPrefix {
        #[doc(hidden)]
        __a_state: <[u8] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[u8] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __a: ::slice_struct::__private::SliceHandle<
            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __b_state: <[u16] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_1: [<[u16] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __b: ::slice_struct::__private::SliceHandle<
            <[u16] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __c_state: <[u32] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_2: [<[u32] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __c: ::slice_struct::__private::SliceHandle<
            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct MixedEmpty_LayoutHelper(::core::marker::PhantomData<MixedEmpty>);
    impl MixedEmpty_LayoutHelper {
        fn calculate_layout(
            a_len: usize,
            b_len: usize,
            c_len: usize,
        ) -> (::std::alloc::Layout, usize, usize, usize) {
            let layout = ::std::alloc::Layout::new::<MixedEmpty_SizedPrefix>();
            let (layout, a_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                    >(a_len)
                        .unwrap(),
                )
                .unwrap();
            let (layout, b_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u16] as ::slice_struct::__private::InlineSlice>::Element,
                    >(b_len)
                        .unwrap(),
                )
                .unwrap();
            let (layout, c_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                    >(c_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), a_offset, b_offset, c_offset)
        }
    }
    #[doc(hidden)]
    pub struct __MixedEmptyInitIter<__I0, __I1, __I2> {
        pub a: __I0,
        pub b: __I1,
        pub c: __I2,
        _marker: ::core::marker::PhantomData<MixedEmpty>,
    }
    unsafe impl<__I0, __I1, __I2> ::slice_struct::__private::SliceInit<MixedEmpty>
    for __MixedEmptyInitIter<__I0, __I1, __I2>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[u8] as ::slice_struct::__private::InlineSlice>::Element,
        >,
        __I1: ::core::iter::ExactSizeIterator<
            Item = <[u16] as ::slice_struct::__private::InlineSlice>::Element,
        >,
        __I2: ::core::iter::ExactSizeIterator<
            Item = <[u32] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let a_len = self.a.len();
            let b_len = self.b.len();
            let c_len = self.c.len();
            MixedEmpty_LayoutHelper::calculate_layout(a_len, b_len, c_len).0
        }
        unsafe fn fixup(ptr: *mut MixedEmpty) {
            let (_, a_offset, b_offset, c_offset) = MixedEmpty_LayoutHelper::calculate_layout(
                (*ptr).0.__a.len,
                (*ptr).0.__b.len,
                (*ptr).0.__c.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__a.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, a_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__b.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u16] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, b_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__c.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, c_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut MixedEmpty {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<MixedEmpty_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<MixedEmpty> {
            let a_len = self.a.len();
            let b_len = self.b.len();
            let c_len = self.c.len();
            let (layout, a_offset, b_offset, c_offset) = MixedEmpty_LayoutHelper::calculate_layout(
                a_len,
                b_len,
                c_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<MixedEmpty_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                MixedEmpty_SizedPrefix {
                    __align_0: [],
                    __align_1: [],
                    __align_2: [],
                    __a_state: <[u8] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __a: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: a_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __b_state: <[u16] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __b: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u16] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: b_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __c_state: <[u32] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __c: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: c_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(a_offset)
                .cast::<<[u8] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.a.into_iter();
            for j in 0..a_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            let field_ptr = ptr
                .add(b_offset)
                .cast::<<[u16] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.b.into_iter();
            for j in 0..b_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            let field_ptr = ptr
                .add(c_offset)
                .cast::<<[u32] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.c.into_iter();
            for j in 0..c_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __MixedEmptyInitDef {
        pub a: (<[u8] as ::slice_struct::__private::InlineSlice>::Element, usize),
        pub b: (<[u16] as ::slice_struct::__private::InlineSlice>::Element, usize),
        pub c: (<[u32] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<MixedEmpty>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<MixedEmpty> for __MixedEmptyInitDef
    where
        <[u8] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        <[u16] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        <[u32] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let a_len = self.a.1;
            let b_len = self.b.1;
            let c_len = self.c.1;
            MixedEmpty_LayoutHelper::calculate_layout(a_len, b_len, c_len).0
        }
        unsafe fn fixup(ptr: *mut MixedEmpty) {
            let (_, a_offset, b_offset, c_offset) = MixedEmpty_LayoutHelper::calculate_layout(
                (*ptr).0.__a.len,
                (*ptr).0.__b.len,
                (*ptr).0.__c.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__a.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, a_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__b.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u16] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, b_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__c.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, c_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut MixedEmpty {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<MixedEmpty_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<MixedEmpty> {
            let a_len = self.a.1;
            let b_len = self.b.1;
            let c_len = self.c.1;
            let (layout, a_offset, b_offset, c_offset) = MixedEmpty_LayoutHelper::calculate_layout(
                a_len,
                b_len,
                c_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<MixedEmpty_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                MixedEmpty_SizedPrefix {
                    __align_0: [],
                    __align_1: [],
                    __align_2: [],
                    __a_state: <[u8] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __a: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: a_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __b_state: <[u16] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __b: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u16] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: b_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __c_state: <[u32] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __c: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: c_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.a.1;
            let field_ptr = ptr
                .add(a_offset)
                .cast::<<[u8] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.a.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            let def_len = self.b.1;
            let field_ptr = ptr
                .add(b_offset)
                .cast::<<[u16] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.b.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            let def_len = self.c.1;
            let field_ptr = ptr
                .add(c_offset)
                .cast::<<[u32] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.c.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl MixedEmpty {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0, __I1, __I2>(
            mut a: __I0,
            mut b: __I1,
            mut c: __I2,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[u8] as ::slice_struct::__private::InlineSlice>::Element,
            >,
            __I1: ::core::iter::ExactSizeIterator<
                Item = <[u16] as ::slice_struct::__private::InlineSlice>::Element,
            >,
            __I2: ::core::iter::ExactSizeIterator<
                Item = <[u32] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__MixedEmptyInitIter {
                a,
                b,
                c,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            a: (<[u8] as ::slice_struct::__private::InlineSlice>::Element, usize),
            b: (<[u16] as ::slice_struct::__private::InlineSlice>::Element, usize),
            c: (<[u32] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[u8] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
            <[u16] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
            <[u32] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__MixedEmptyInitDef {
                a,
                b,
                c,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_mixed_empty_and_full"]
#[doc(hidden)]
pub const test_mixed_empty_and_full: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_mixed_empty_and_full"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 150usize,
        start_col: 4usize,
        end_line: 150usize,
        end_col: 29usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_mixed_empty_and_full()),
    ),
};
fn test_mixed_empty_and_full() {
    let s = MixedEmpty::init_def((1, 0), (2, 5), (3, 0)).in_box();
    match (&s.view().a, &&[]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().b, &&[2, 2, 2, 2, 2]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().c, &&[]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let s2 = MixedEmpty::init_iter(
            [1, 2].into_iter(),
            [].into_iter(),
            [3, 4, 5].into_iter(),
        )
        .in_box();
    match (&s2.view().a, &&[1, 2]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s2.view().b, &&[]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s2.view().c, &&[3, 4, 5]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(C)]
#[doc(hidden)]
struct ZstStruct__Inner {
    #[doc(hidden)]
    __zst_state: <[()] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[()] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __zst: ::slice_struct::__private::SliceHandle<
        <[()] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct ZstStruct(ZstStruct__Inner);
impl ::core::ops::Drop for ZstStruct {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__zst.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct ZstStructView<'__a>
where
    [()]: '__a,
{
    zst: <[()] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct ZstStructViewMut<'__a>
where
    [()]: '__a,
{
    zst: <[()] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for ZstStruct {
    type View<'__a> = ZstStructView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        ZstStructView {
            zst: <[()] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__zst_state,
                this.0.__zst.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for ZstStruct {
    type ViewMut<'__a> = ZstStructViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            ZstStructViewMut {
                zst: <[()] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__zst_state,
                    this.0.__zst.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl ZstStruct {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> ZstStructView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> ZstStructViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct ZstStruct_SizedPrefix {
        #[doc(hidden)]
        __zst_state: <[()] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[()] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __zst: ::slice_struct::__private::SliceHandle<
            <[()] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct ZstStruct_LayoutHelper(::core::marker::PhantomData<ZstStruct>);
    impl ZstStruct_LayoutHelper {
        fn calculate_layout(zst_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<ZstStruct_SizedPrefix>();
            let (layout, zst_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[()] as ::slice_struct::__private::InlineSlice>::Element,
                    >(zst_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), zst_offset)
        }
    }
    #[doc(hidden)]
    pub struct __ZstStructInitIter<__I0> {
        pub zst: __I0,
        _marker: ::core::marker::PhantomData<ZstStruct>,
    }
    unsafe impl<__I0> ::slice_struct::__private::SliceInit<ZstStruct>
    for __ZstStructInitIter<__I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[()] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let zst_len = self.zst.len();
            ZstStruct_LayoutHelper::calculate_layout(zst_len).0
        }
        unsafe fn fixup(ptr: *mut ZstStruct) {
            let (_, zst_offset) = ZstStruct_LayoutHelper::calculate_layout(
                (*ptr).0.__zst.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__zst.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[()] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, zst_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut ZstStruct {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<ZstStruct_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<ZstStruct> {
            let zst_len = self.zst.len();
            let (layout, zst_offset) = ZstStruct_LayoutHelper::calculate_layout(zst_len);
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<ZstStruct_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                ZstStruct_SizedPrefix {
                    __align_0: [],
                    __zst_state: <[()] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __zst: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[()] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: zst_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(zst_offset)
                .cast::<<[()] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.zst.into_iter();
            for j in 0..zst_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __ZstStructInitDef {
        pub zst: (<[()] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<ZstStruct>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<ZstStruct> for __ZstStructInitDef
    where
        <[()] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let zst_len = self.zst.1;
            ZstStruct_LayoutHelper::calculate_layout(zst_len).0
        }
        unsafe fn fixup(ptr: *mut ZstStruct) {
            let (_, zst_offset) = ZstStruct_LayoutHelper::calculate_layout(
                (*ptr).0.__zst.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__zst.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[()] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, zst_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut ZstStruct {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<ZstStruct_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<ZstStruct> {
            let zst_len = self.zst.1;
            let (layout, zst_offset) = ZstStruct_LayoutHelper::calculate_layout(zst_len);
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<ZstStruct_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                ZstStruct_SizedPrefix {
                    __align_0: [],
                    __zst_state: <[()] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __zst: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[()] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: zst_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.zst.1;
            let field_ptr = ptr
                .add(zst_offset)
                .cast::<<[()] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.zst.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl ZstStruct {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            mut zst: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[()] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__ZstStructInitIter {
                zst,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            zst: (<[()] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[()] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__ZstStructInitDef {
                zst,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_zst"]
#[doc(hidden)]
pub const test_zst: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_zst"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 172usize,
        start_col: 4usize,
        end_line: 172usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_zst())),
};
fn test_zst() {
    let s = ZstStruct::init_def(((), 100)).in_box();
    match (&s.view().zst.len(), &100) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(align(64))]
struct Aligned64(u8);
#[automatically_derived]
impl ::core::clone::Clone for Aligned64 {
    #[inline]
    fn clone(&self) -> Aligned64 {
        Aligned64(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Aligned64 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Aligned64 {
    #[inline]
    fn eq(&self, other: &Aligned64) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Aligned64 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Aligned64", &&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
struct HighlyAligned__Inner {
    #[doc(hidden)]
    __a: u8,
    #[doc(hidden)]
    __data_state: <[Aligned64] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[Aligned64] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __data: ::slice_struct::__private::SliceHandle<
        <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct HighlyAligned(HighlyAligned__Inner);
impl ::core::ops::Drop for HighlyAligned {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__data.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct HighlyAlignedView<'__a>
where
    [Aligned64]: '__a,
{
    a: &'__a u8,
    data: <[Aligned64] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct HighlyAlignedViewMut<'__a>
where
    [Aligned64]: '__a,
{
    a: &'__a mut u8,
    data: <[Aligned64] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for HighlyAligned {
    type View<'__a> = HighlyAlignedView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        HighlyAlignedView {
            a: &this.0.__a,
            data: <[Aligned64] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__data_state,
                this.0.__data.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for HighlyAligned {
    type ViewMut<'__a> = HighlyAlignedViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            HighlyAlignedViewMut {
                a: &mut this.0.__a,
                data: <[Aligned64] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__data_state,
                    this.0.__data.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl HighlyAligned {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> HighlyAlignedView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(
        self: ::core::pin::Pin<&'__a mut Self>,
    ) -> HighlyAlignedViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct HighlyAligned_SizedPrefix {
        #[doc(hidden)]
        __a: u8,
        #[doc(hidden)]
        __data_state: <[Aligned64] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[Aligned64] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __data: ::slice_struct::__private::SliceHandle<
            <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct HighlyAligned_LayoutHelper(::core::marker::PhantomData<HighlyAligned>);
    impl HighlyAligned_LayoutHelper {
        fn calculate_layout(data_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<HighlyAligned_SizedPrefix>();
            let (layout, data_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
                    >(data_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), data_offset)
        }
    }
    #[doc(hidden)]
    pub struct __HighlyAlignedInitIter<__I0> {
        pub a: u8,
        pub data: __I0,
        _marker: ::core::marker::PhantomData<HighlyAligned>,
    }
    unsafe impl<__I0> ::slice_struct::__private::SliceInit<HighlyAligned>
    for __HighlyAlignedInitIter<__I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.len();
            HighlyAligned_LayoutHelper::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut HighlyAligned) {
            let (_, data_offset) = HighlyAligned_LayoutHelper::calculate_layout(
                (*ptr).0.__data.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut HighlyAligned {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<HighlyAligned_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<HighlyAligned> {
            let data_len = self.data.len();
            let (layout, data_offset) = HighlyAligned_LayoutHelper::calculate_layout(
                data_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<HighlyAligned_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                HighlyAligned_SizedPrefix {
                    __a: self.a,
                    __align_0: [],
                    __data_state: <[Aligned64] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(data_offset)
                .cast::<
                    <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
                >();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.data.into_iter();
            for j in 0..data_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __HighlyAlignedInitDef {
        pub a: u8,
        pub data: (
            <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
            usize,
        ),
        _marker: ::core::marker::PhantomData<HighlyAligned>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<HighlyAligned>
    for __HighlyAlignedInitDef
    where
        <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.1;
            HighlyAligned_LayoutHelper::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut HighlyAligned) {
            let (_, data_offset) = HighlyAligned_LayoutHelper::calculate_layout(
                (*ptr).0.__data.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut HighlyAligned {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<HighlyAligned_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<HighlyAligned> {
            let data_len = self.data.1;
            let (layout, data_offset) = HighlyAligned_LayoutHelper::calculate_layout(
                data_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<HighlyAligned_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                HighlyAligned_SizedPrefix {
                    __a: self.a,
                    __align_0: [],
                    __data_state: <[Aligned64] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.data.1;
            let field_ptr = ptr
                .add(data_offset)
                .cast::<
                    <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
                >();
            if def_len > 0 {
                let def_val = self.data.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl HighlyAligned {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            a: u8,
            mut data: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__HighlyAlignedInitIter {
                a,
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            a: u8,
            data: (
                <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element,
                usize,
            ),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[Aligned64] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__HighlyAlignedInitDef {
                a,
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_highly_aligned"]
#[doc(hidden)]
pub const test_highly_aligned: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_highly_aligned"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 188usize,
        start_col: 4usize,
        end_line: 188usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_highly_aligned()),
    ),
};
fn test_highly_aligned() {
    let s = HighlyAligned::init_def(42, (Aligned64(99), 3)).in_box();
    match (&*s.view().a, &42) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().data, &&[Aligned64(99), Aligned64(99), Aligned64(99)]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let data_ptr = s.view().data.as_ptr() as usize;
    match (&(data_ptr % 64), &0) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::Some(
                        format_args!("Slice data must be 64-byte aligned"),
                    ),
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_send_sync"]
#[doc(hidden)]
pub const test_send_sync: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_send_sync"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 199usize,
        start_col: 4usize,
        end_line: 199usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_send_sync()),
    ),
};
fn test_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    assert_send::<std::pin::Pin<Box<Str<i32>>>>();
    assert_sync::<std::pin::Pin<Box<Str<i32>>>>();
}
use std::sync::atomic::{AtomicUsize, Ordering};
static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);
struct DropCounter(usize);
#[automatically_derived]
impl ::core::clone::Clone for DropCounter {
    #[inline]
    fn clone(&self) -> DropCounter {
        DropCounter(::core::clone::Clone::clone(&self.0))
    }
}
impl Drop for DropCounter {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}
#[repr(C)]
#[doc(hidden)]
struct DropTest__Inner {
    #[doc(hidden)]
    __items_state: <[DropCounter] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[DropCounter] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __items: ::slice_struct::__private::SliceHandle<
        <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct DropTest(DropTest__Inner);
impl ::core::ops::Drop for DropTest {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__items.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct DropTestView<'__a>
where
    [DropCounter]: '__a,
{
    items: <[DropCounter] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct DropTestViewMut<'__a>
where
    [DropCounter]: '__a,
{
    items: <[DropCounter] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for DropTest {
    type View<'__a> = DropTestView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        DropTestView {
            items: <[DropCounter] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__items_state,
                this.0.__items.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for DropTest {
    type ViewMut<'__a> = DropTestViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            DropTestViewMut {
                items: <[DropCounter] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__items_state,
                    this.0.__items.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl DropTest {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> DropTestView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> DropTestViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct DropTest_SizedPrefix {
        #[doc(hidden)]
        __items_state: <[DropCounter] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[DropCounter] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __items: ::slice_struct::__private::SliceHandle<
            <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct DropTest_LayoutHelper(::core::marker::PhantomData<DropTest>);
    impl DropTest_LayoutHelper {
        fn calculate_layout(items_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<DropTest_SizedPrefix>();
            let (layout, items_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
                    >(items_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), items_offset)
        }
    }
    #[doc(hidden)]
    pub struct __DropTestInitIter<__I0> {
        pub items: __I0,
        _marker: ::core::marker::PhantomData<DropTest>,
    }
    unsafe impl<__I0> ::slice_struct::__private::SliceInit<DropTest>
    for __DropTestInitIter<__I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let items_len = self.items.len();
            DropTest_LayoutHelper::calculate_layout(items_len).0
        }
        unsafe fn fixup(ptr: *mut DropTest) {
            let (_, items_offset) = DropTest_LayoutHelper::calculate_layout(
                (*ptr).0.__items.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__items.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, items_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut DropTest {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<DropTest_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<DropTest> {
            let items_len = self.items.len();
            let (layout, items_offset) = DropTest_LayoutHelper::calculate_layout(
                items_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<DropTest_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                DropTest_SizedPrefix {
                    __align_0: [],
                    __items_state: <[DropCounter] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __items: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: items_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(items_offset)
                .cast::<
                    <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
                >();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.items.into_iter();
            for j in 0..items_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __DropTestInitDef {
        pub items: (
            <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
            usize,
        ),
        _marker: ::core::marker::PhantomData<DropTest>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<DropTest> for __DropTestInitDef
    where
        <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let items_len = self.items.1;
            DropTest_LayoutHelper::calculate_layout(items_len).0
        }
        unsafe fn fixup(ptr: *mut DropTest) {
            let (_, items_offset) = DropTest_LayoutHelper::calculate_layout(
                (*ptr).0.__items.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__items.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, items_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut DropTest {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<DropTest_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<DropTest> {
            let items_len = self.items.1;
            let (layout, items_offset) = DropTest_LayoutHelper::calculate_layout(
                items_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<DropTest_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                DropTest_SizedPrefix {
                    __align_0: [],
                    __items_state: <[DropCounter] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __items: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: items_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.items.1;
            let field_ptr = ptr
                .add(items_offset)
                .cast::<
                    <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
                >();
            if def_len > 0 {
                let def_val = self.items.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl DropTest {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            mut items: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__DropTestInitIter {
                items,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            items: (
                <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element,
                usize,
            ),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[DropCounter] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__DropTestInitDef {
                items,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_exact_drop_count"]
#[doc(hidden)]
pub const test_exact_drop_count: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_exact_drop_count"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 229usize,
        start_col: 4usize,
        end_line: 229usize,
        end_col: 25usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_exact_drop_count()),
    ),
};
fn test_exact_drop_count() {
    DROP_COUNT.store(0, Ordering::SeqCst);
    {
        let s = DropTest::init_def((DropCounter(1), 5)).in_box();
        match (&s.view().items.len(), &5) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&DROP_COUNT.load(Ordering::SeqCst), &0) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    match (&DROP_COUNT.load(Ordering::SeqCst), &5) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
struct BadIter {
    yields: usize,
    claims: usize,
}
impl Iterator for BadIter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.yields > 0 {
            self.yields -= 1;
            Some(1)
        } else {
            None
        }
    }
}
impl ExactSizeIterator for BadIter {
    fn len(&self) -> usize {
        self.claims
    }
}
#[repr(C)]
#[doc(hidden)]
struct IterTest__Inner {
    #[doc(hidden)]
    __data_state: <[u32] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[u32] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __data: ::slice_struct::__private::SliceHandle<
        <[u32] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct IterTest(IterTest__Inner);
impl ::core::ops::Drop for IterTest {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__data.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct IterTestView<'__a>
where
    [u32]: '__a,
{
    data: <[u32] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct IterTestViewMut<'__a>
where
    [u32]: '__a,
{
    data: <[u32] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for IterTest {
    type View<'__a> = IterTestView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        IterTestView {
            data: <[u32] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__data_state,
                this.0.__data.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for IterTest {
    type ViewMut<'__a> = IterTestViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            IterTestViewMut {
                data: <[u32] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__data_state,
                    this.0.__data.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl IterTest {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> IterTestView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> IterTestViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct IterTest_SizedPrefix {
        #[doc(hidden)]
        __data_state: <[u32] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[u32] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __data: ::slice_struct::__private::SliceHandle<
            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct IterTest_LayoutHelper(::core::marker::PhantomData<IterTest>);
    impl IterTest_LayoutHelper {
        fn calculate_layout(data_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<IterTest_SizedPrefix>();
            let (layout, data_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                    >(data_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), data_offset)
        }
    }
    #[doc(hidden)]
    pub struct __IterTestInitIter<__I0> {
        pub data: __I0,
        _marker: ::core::marker::PhantomData<IterTest>,
    }
    unsafe impl<__I0> ::slice_struct::__private::SliceInit<IterTest>
    for __IterTestInitIter<__I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[u32] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.len();
            IterTest_LayoutHelper::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut IterTest) {
            let (_, data_offset) = IterTest_LayoutHelper::calculate_layout(
                (*ptr).0.__data.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut IterTest {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<IterTest_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<IterTest> {
            let data_len = self.data.len();
            let (layout, data_offset) = IterTest_LayoutHelper::calculate_layout(
                data_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<IterTest_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                IterTest_SizedPrefix {
                    __align_0: [],
                    __data_state: <[u32] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(data_offset)
                .cast::<<[u32] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.data.into_iter();
            for j in 0..data_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __IterTestInitDef {
        pub data: (<[u32] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<IterTest>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<IterTest> for __IterTestInitDef
    where
        <[u32] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.1;
            IterTest_LayoutHelper::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut IterTest) {
            let (_, data_offset) = IterTest_LayoutHelper::calculate_layout(
                (*ptr).0.__data.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut IterTest {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<IterTest_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<IterTest> {
            let data_len = self.data.1;
            let (layout, data_offset) = IterTest_LayoutHelper::calculate_layout(
                data_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<IterTest_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                IterTest_SizedPrefix {
                    __align_0: [],
                    __data_state: <[u32] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.data.1;
            let field_ptr = ptr
                .add(data_offset)
                .cast::<<[u32] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.data.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl IterTest {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            mut data: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[u32] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__IterTestInitIter {
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            data: (<[u32] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[u32] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__IterTestInitDef {
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_lying_iterator_long"]
#[doc(hidden)]
pub const test_lying_iterator_long: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_lying_iterator_long"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 269usize,
        start_col: 4usize,
        end_line: 269usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_lying_iterator_long()),
    ),
};
fn test_lying_iterator_long() {
    let iter = BadIter { yields: 10, claims: 5 };
    let s = IterTest::init_iter(iter).in_box();
    match (&s.view().data.len(), &5) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_lying_iterator_short"]
#[doc(hidden)]
pub const test_lying_iterator_short: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_lying_iterator_short"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 278usize,
        start_col: 4usize,
        end_line: 278usize,
        end_col: 29usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::YesWithMessage("yielded fewer elements"),
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_lying_iterator_short()),
    ),
};
#[should_panic(expected = "yielded fewer elements")]
fn test_lying_iterator_short() {
    let iter = BadIter { yields: 5, claims: 10 };
    let _s = IterTest::init_iter(iter).in_box();
}
#[repr(C)]
#[doc(hidden)]
struct WithLifetimes__Inner<'a, T: Clone> {
    #[doc(hidden)]
    __prefix: &'a str,
    #[doc(hidden)]
    __data_state: <[T] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[T] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __data: ::slice_struct::__private::SliceHandle<
        <[T] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct WithLifetimes<'a, T: Clone>(WithLifetimes__Inner<'a, T>);
impl<'a, T: Clone> ::core::ops::Drop for WithLifetimes<'a, T> {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__data.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct WithLifetimesView<'__a, 'a, T: Clone>
where
    [T]: '__a,
{
    prefix: &'__a &'a str,
    data: <[T] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct WithLifetimesViewMut<'__a, 'a, T: Clone>
where
    [T]: '__a,
{
    prefix: &'__a mut &'a str,
    data: <[T] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl<'a, T: Clone> ::slice_struct::AsView for WithLifetimes<'a, T> {
    type View<'__a> = WithLifetimesView<'__a, 'a, T> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        WithLifetimesView {
            prefix: &this.0.__prefix,
            data: <[T] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__data_state,
                this.0.__data.as_non_null(base_ptr),
            ),
        }
    }
}
impl<'a, T: Clone> ::slice_struct::AsViewMut for WithLifetimes<'a, T> {
    type ViewMut<'__a> = WithLifetimesViewMut<'__a, 'a, T> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            WithLifetimesViewMut {
                prefix: &mut this.0.__prefix,
                data: <[T] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__data_state,
                    this.0.__data.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl<'a, T: Clone> WithLifetimes<'a, T> {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> WithLifetimesView<'__a, 'a, T> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(
        self: ::core::pin::Pin<&'__a mut Self>,
    ) -> WithLifetimesViewMut<'__a, 'a, T> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct WithLifetimes_SizedPrefix<'a, T: Clone> {
        #[doc(hidden)]
        __prefix: &'a str,
        #[doc(hidden)]
        __data_state: <[T] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[T] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __data: ::slice_struct::__private::SliceHandle<
            <[T] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct WithLifetimes_LayoutHelper<'a, T: Clone>(
        ::core::marker::PhantomData<WithLifetimes<'a, T>>,
    );
    impl<'a, T: Clone> WithLifetimes_LayoutHelper<'a, T> {
        fn calculate_layout(data_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<WithLifetimes_SizedPrefix<'a, T>>();
            let (layout, data_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[T] as ::slice_struct::__private::InlineSlice>::Element,
                    >(data_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), data_offset)
        }
    }
    #[doc(hidden)]
    pub struct __WithLifetimesInitIter<'a, T: Clone, __I0> {
        pub prefix: &'a str,
        pub data: __I0,
        _marker: ::core::marker::PhantomData<WithLifetimes<'a, T>>,
    }
    unsafe impl<
        'a,
        T: Clone,
        __I0,
    > ::slice_struct::__private::SliceInit<WithLifetimes<'a, T>>
    for __WithLifetimesInitIter<'a, T, __I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[T] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.len();
            WithLifetimes_LayoutHelper::<'a, T>::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut WithLifetimes<'a, T>) {
            let (_, data_offset) = WithLifetimes_LayoutHelper::<
                'a,
                T,
            >::calculate_layout((*ptr).0.__data.len);
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[T] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut WithLifetimes<'a, T> {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<WithLifetimes_SizedPrefix<'a, T>>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<WithLifetimes<'a, T>> {
            let data_len = self.data.len();
            let (layout, data_offset) = WithLifetimes_LayoutHelper::<
                'a,
                T,
            >::calculate_layout(data_len);
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<WithLifetimes_SizedPrefix<'a, T>>();
            ::core::ptr::write(
                sized_ptr,
                WithLifetimes_SizedPrefix {
                    __prefix: self.prefix,
                    __align_0: [],
                    __data_state: <[T] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[T] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(data_offset)
                .cast::<<[T] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.data.into_iter();
            for j in 0..data_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __WithLifetimesInitDef<'a, T: Clone> {
        pub prefix: &'a str,
        pub data: (<[T] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<WithLifetimes<'a, T>>,
    }
    unsafe impl<'a, T: Clone> ::slice_struct::__private::SliceInit<WithLifetimes<'a, T>>
    for __WithLifetimesInitDef<'a, T>
    where
        <[T] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.1;
            WithLifetimes_LayoutHelper::<'a, T>::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut WithLifetimes<'a, T>) {
            let (_, data_offset) = WithLifetimes_LayoutHelper::<
                'a,
                T,
            >::calculate_layout((*ptr).0.__data.len);
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[T] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut WithLifetimes<'a, T> {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<WithLifetimes_SizedPrefix<'a, T>>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<WithLifetimes<'a, T>> {
            let data_len = self.data.1;
            let (layout, data_offset) = WithLifetimes_LayoutHelper::<
                'a,
                T,
            >::calculate_layout(data_len);
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<WithLifetimes_SizedPrefix<'a, T>>();
            ::core::ptr::write(
                sized_ptr,
                WithLifetimes_SizedPrefix {
                    __prefix: self.prefix,
                    __align_0: [],
                    __data_state: <[T] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[T] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.data.1;
            let field_ptr = ptr
                .add(data_offset)
                .cast::<<[T] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.data.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl<'a, T: Clone> WithLifetimes<'a, T> {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            prefix: &'a str,
            mut data: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[T] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__WithLifetimesInitIter {
                prefix,
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            prefix: &'a str,
            data: (<[T] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[T] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__WithLifetimesInitDef {
                prefix,
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_lifetimes_and_generics"]
#[doc(hidden)]
pub const test_lifetimes_and_generics: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_lifetimes_and_generics"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 293usize,
        start_col: 4usize,
        end_line: 293usize,
        end_col: 31usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_lifetimes_and_generics()),
    ),
};
fn test_lifetimes_and_generics() {
    let local_str = String::from("hello");
    let s = WithLifetimes::init_def(local_str.as_str(), (42_u32, 3)).in_box();
    match (&*s.view().prefix, &"hello") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().data, &&[42, 42, 42]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(C)]
#[doc(hidden)]
struct KitchenSink__Inner {
    #[doc(hidden)]
    __a: u8,
    #[doc(hidden)]
    __b_state: <[u64] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[u64] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __b: ::slice_struct::__private::SliceHandle<
        <[u64] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __c_state: <[u8] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_1: [<[u8] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __c: ::slice_struct::__private::SliceHandle<
        <[u8] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __d_state: <[()] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_2: [<[()] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __d: ::slice_struct::__private::SliceHandle<
        <[()] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __e_state: <[u32] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_3: [<[u32] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __e: ::slice_struct::__private::SliceHandle<
        <[u32] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct KitchenSink(KitchenSink__Inner);
impl ::core::ops::Drop for KitchenSink {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__b.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__c.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__d.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__e.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct KitchenSinkView<'__a>
where
    [u64]: '__a,
    [u8]: '__a,
    [()]: '__a,
    [u32]: '__a,
{
    a: &'__a u8,
    b: <[u64] as ::slice_struct::__private::InlineSlice>::View<'__a>,
    c: <[u8] as ::slice_struct::__private::InlineSlice>::View<'__a>,
    d: <[()] as ::slice_struct::__private::InlineSlice>::View<'__a>,
    e: <[u32] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct KitchenSinkViewMut<'__a>
where
    [u64]: '__a,
    [u8]: '__a,
    [()]: '__a,
    [u32]: '__a,
{
    a: &'__a mut u8,
    b: <[u64] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
    c: <[u8] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
    d: <[()] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
    e: <[u32] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for KitchenSink {
    type View<'__a> = KitchenSinkView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        KitchenSinkView {
            a: &this.0.__a,
            b: <[u64] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__b_state,
                this.0.__b.as_non_null(base_ptr),
            ),
            c: <[u8] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__c_state,
                this.0.__c.as_non_null(base_ptr),
            ),
            d: <[()] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__d_state,
                this.0.__d.as_non_null(base_ptr),
            ),
            e: <[u32] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__e_state,
                this.0.__e.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for KitchenSink {
    type ViewMut<'__a> = KitchenSinkViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            KitchenSinkViewMut {
                a: &mut this.0.__a,
                b: <[u64] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__b_state,
                    this.0.__b.as_non_null(base_ptr),
                ),
                c: <[u8] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__c_state,
                    this.0.__c.as_non_null(base_ptr),
                ),
                d: <[()] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__d_state,
                    this.0.__d.as_non_null(base_ptr),
                ),
                e: <[u32] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__e_state,
                    this.0.__e.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl KitchenSink {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> KitchenSinkView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(
        self: ::core::pin::Pin<&'__a mut Self>,
    ) -> KitchenSinkViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct KitchenSink_SizedPrefix {
        #[doc(hidden)]
        __a: u8,
        #[doc(hidden)]
        __b_state: <[u64] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[u64] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __b: ::slice_struct::__private::SliceHandle<
            <[u64] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __c_state: <[u8] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_1: [<[u8] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __c: ::slice_struct::__private::SliceHandle<
            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __d_state: <[()] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_2: [<[()] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __d: ::slice_struct::__private::SliceHandle<
            <[()] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __e_state: <[u32] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_3: [<[u32] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __e: ::slice_struct::__private::SliceHandle<
            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct KitchenSink_LayoutHelper(::core::marker::PhantomData<KitchenSink>);
    impl KitchenSink_LayoutHelper {
        fn calculate_layout(
            b_len: usize,
            c_len: usize,
            d_len: usize,
            e_len: usize,
        ) -> (::std::alloc::Layout, usize, usize, usize, usize) {
            let layout = ::std::alloc::Layout::new::<KitchenSink_SizedPrefix>();
            let (layout, b_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u64] as ::slice_struct::__private::InlineSlice>::Element,
                    >(b_len)
                        .unwrap(),
                )
                .unwrap();
            let (layout, c_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                    >(c_len)
                        .unwrap(),
                )
                .unwrap();
            let (layout, d_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[()] as ::slice_struct::__private::InlineSlice>::Element,
                    >(d_len)
                        .unwrap(),
                )
                .unwrap();
            let (layout, e_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                    >(e_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), b_offset, c_offset, d_offset, e_offset)
        }
    }
    #[doc(hidden)]
    pub struct __KitchenSinkInitIter<__I0, __I1, __I2, __I3> {
        pub a: u8,
        pub b: __I0,
        pub c: __I1,
        pub d: __I2,
        pub e: __I3,
        _marker: ::core::marker::PhantomData<KitchenSink>,
    }
    unsafe impl<__I0, __I1, __I2, __I3> ::slice_struct::__private::SliceInit<KitchenSink>
    for __KitchenSinkInitIter<__I0, __I1, __I2, __I3>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[u64] as ::slice_struct::__private::InlineSlice>::Element,
        >,
        __I1: ::core::iter::ExactSizeIterator<
            Item = <[u8] as ::slice_struct::__private::InlineSlice>::Element,
        >,
        __I2: ::core::iter::ExactSizeIterator<
            Item = <[()] as ::slice_struct::__private::InlineSlice>::Element,
        >,
        __I3: ::core::iter::ExactSizeIterator<
            Item = <[u32] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let b_len = self.b.len();
            let c_len = self.c.len();
            let d_len = self.d.len();
            let e_len = self.e.len();
            KitchenSink_LayoutHelper::calculate_layout(b_len, c_len, d_len, e_len).0
        }
        unsafe fn fixup(ptr: *mut KitchenSink) {
            let (_, b_offset, c_offset, d_offset, e_offset) = KitchenSink_LayoutHelper::calculate_layout(
                (*ptr).0.__b.len,
                (*ptr).0.__c.len,
                (*ptr).0.__d.len,
                (*ptr).0.__e.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__b.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u64] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, b_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__c.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, c_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__d.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[()] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, d_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__e.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, e_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut KitchenSink {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<KitchenSink_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<KitchenSink> {
            let b_len = self.b.len();
            let c_len = self.c.len();
            let d_len = self.d.len();
            let e_len = self.e.len();
            let (layout, b_offset, c_offset, d_offset, e_offset) = KitchenSink_LayoutHelper::calculate_layout(
                b_len,
                c_len,
                d_len,
                e_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<KitchenSink_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                KitchenSink_SizedPrefix {
                    __a: self.a,
                    __align_0: [],
                    __align_1: [],
                    __align_2: [],
                    __align_3: [],
                    __b_state: <[u64] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __b: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u64] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: b_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __c_state: <[u8] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __c: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: c_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __d_state: <[()] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __d: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[()] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: d_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __e_state: <[u32] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __e: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: e_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(b_offset)
                .cast::<<[u64] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.b.into_iter();
            for j in 0..b_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            let field_ptr = ptr
                .add(c_offset)
                .cast::<<[u8] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.c.into_iter();
            for j in 0..c_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            let field_ptr = ptr
                .add(d_offset)
                .cast::<<[()] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.d.into_iter();
            for j in 0..d_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            let field_ptr = ptr
                .add(e_offset)
                .cast::<<[u32] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.e.into_iter();
            for j in 0..e_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __KitchenSinkInitDef {
        pub a: u8,
        pub b: (<[u64] as ::slice_struct::__private::InlineSlice>::Element, usize),
        pub c: (<[u8] as ::slice_struct::__private::InlineSlice>::Element, usize),
        pub d: (<[()] as ::slice_struct::__private::InlineSlice>::Element, usize),
        pub e: (<[u32] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<KitchenSink>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<KitchenSink>
    for __KitchenSinkInitDef
    where
        <[u64] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        <[u8] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        <[()] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        <[u32] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let b_len = self.b.1;
            let c_len = self.c.1;
            let d_len = self.d.1;
            let e_len = self.e.1;
            KitchenSink_LayoutHelper::calculate_layout(b_len, c_len, d_len, e_len).0
        }
        unsafe fn fixup(ptr: *mut KitchenSink) {
            let (_, b_offset, c_offset, d_offset, e_offset) = KitchenSink_LayoutHelper::calculate_layout(
                (*ptr).0.__b.len,
                (*ptr).0.__c.len,
                (*ptr).0.__d.len,
                (*ptr).0.__e.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__b.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u64] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, b_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__c.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, c_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__d.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[()] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, d_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__e.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, e_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut KitchenSink {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<KitchenSink_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<KitchenSink> {
            let b_len = self.b.1;
            let c_len = self.c.1;
            let d_len = self.d.1;
            let e_len = self.e.1;
            let (layout, b_offset, c_offset, d_offset, e_offset) = KitchenSink_LayoutHelper::calculate_layout(
                b_len,
                c_len,
                d_len,
                e_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<KitchenSink_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                KitchenSink_SizedPrefix {
                    __a: self.a,
                    __align_0: [],
                    __align_1: [],
                    __align_2: [],
                    __align_3: [],
                    __b_state: <[u64] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __b: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u64] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: b_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __c_state: <[u8] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __c: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: c_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __d_state: <[()] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __d: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[()] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: d_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __e_state: <[u32] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __e: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u32] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: e_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.b.1;
            let field_ptr = ptr
                .add(b_offset)
                .cast::<<[u64] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.b.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            let def_len = self.c.1;
            let field_ptr = ptr
                .add(c_offset)
                .cast::<<[u8] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.c.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            let def_len = self.d.1;
            let field_ptr = ptr
                .add(d_offset)
                .cast::<<[()] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.d.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            let def_len = self.e.1;
            let field_ptr = ptr
                .add(e_offset)
                .cast::<<[u32] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.e.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl KitchenSink {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0, __I1, __I2, __I3>(
            a: u8,
            mut b: __I0,
            mut c: __I1,
            mut d: __I2,
            mut e: __I3,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[u64] as ::slice_struct::__private::InlineSlice>::Element,
            >,
            __I1: ::core::iter::ExactSizeIterator<
                Item = <[u8] as ::slice_struct::__private::InlineSlice>::Element,
            >,
            __I2: ::core::iter::ExactSizeIterator<
                Item = <[()] as ::slice_struct::__private::InlineSlice>::Element,
            >,
            __I3: ::core::iter::ExactSizeIterator<
                Item = <[u32] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__KitchenSinkInitIter {
                a,
                b,
                c,
                d,
                e,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            a: u8,
            b: (<[u64] as ::slice_struct::__private::InlineSlice>::Element, usize),
            c: (<[u8] as ::slice_struct::__private::InlineSlice>::Element, usize),
            d: (<[()] as ::slice_struct::__private::InlineSlice>::Element, usize),
            e: (<[u32] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[u64] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
            <[u8] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
            <[()] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
            <[u32] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__KitchenSinkInitDef {
                a,
                b,
                c,
                d,
                e,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_kitchen_sink_layout"]
#[doc(hidden)]
pub const test_kitchen_sink_layout: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_kitchen_sink_layout"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 312usize,
        start_col: 4usize,
        end_line: 312usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_kitchen_sink_layout()),
    ),
};
fn test_kitchen_sink_layout() {
    let s = KitchenSink::init_def(1, (2_u64, 3), (3_u8, 5), ((), 10), (5_u32, 2))
        .in_box();
    let v = s.view();
    match (&*v.a, &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&v.b, &&[2, 2, 2]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&v.c, &&[3, 3, 3, 3, 3]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&v.d.len(), &10) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&v.e, &&[5, 5]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_multiple_simultaneous_views"]
#[doc(hidden)]
pub const test_multiple_simultaneous_views: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_multiple_simultaneous_views"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 336usize,
        start_col: 4usize,
        end_line: 336usize,
        end_col: 36usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_multiple_simultaneous_views()),
    ),
};
fn test_multiple_simultaneous_views() {
    let s = Str::<i32>::init_iter(10, [1, 2, 3].into_iter(), [4, 5].into_iter())
        .in_box();
    let v1 = s.view();
    let v2 = s.view();
    match (&v1.b, &v2.b) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&v1.c, &v2.c) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_view_mut_then_view"]
#[doc(hidden)]
pub const test_view_mut_then_view: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_view_mut_then_view"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 349usize,
        start_col: 4usize,
        end_line: 349usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_view_mut_then_view()),
    ),
};
fn test_view_mut_then_view() {
    let mut s = Str::<i32>::init_iter(0, [1, 2, 3].into_iter(), [10].into_iter())
        .in_box();
    {
        let v = s.as_mut().view_mut();
        v.b[0] = 99;
    }
    match (&s.view().b[0], &99) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_slice_independence"]
#[doc(hidden)]
pub const test_slice_independence: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_slice_independence"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 361usize,
        start_col: 4usize,
        end_line: 361usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_slice_independence()),
    ),
};
fn test_slice_independence() {
    let mut s = Str::<i32>::init_iter(0, [1, 2, 3].into_iter(), [10, 20, 30].into_iter())
        .in_box();
    let v = s.as_mut().view_mut();
    let sum_c: u32 = v.c.iter().sum();
    v.b.iter_mut().for_each(|x| *x = sum_c as i32);
    drop(v);
    match (&s.view().b, &&[60, 60, 60]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().c, &&[10, 20, 30]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(C)]
#[doc(hidden)]
struct BigBuf__Inner {
    #[doc(hidden)]
    __tag: u64,
    #[doc(hidden)]
    __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[u8] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __data: ::slice_struct::__private::SliceHandle<
        <[u8] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct BigBuf(BigBuf__Inner);
impl ::core::ops::Drop for BigBuf {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__data.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct BigBufView<'__a>
where
    [u8]: '__a,
{
    tag: &'__a u64,
    data: <[u8] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct BigBufViewMut<'__a>
where
    [u8]: '__a,
{
    tag: &'__a mut u64,
    data: <[u8] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for BigBuf {
    type View<'__a> = BigBufView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        BigBufView {
            tag: &this.0.__tag,
            data: <[u8] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__data_state,
                this.0.__data.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for BigBuf {
    type ViewMut<'__a> = BigBufViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            BigBufViewMut {
                tag: &mut this.0.__tag,
                data: <[u8] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__data_state,
                    this.0.__data.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl BigBuf {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> BigBufView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> BigBufViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct BigBuf_SizedPrefix {
        #[doc(hidden)]
        __tag: u64,
        #[doc(hidden)]
        __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[u8] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __data: ::slice_struct::__private::SliceHandle<
            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct BigBuf_LayoutHelper(::core::marker::PhantomData<BigBuf>);
    impl BigBuf_LayoutHelper {
        fn calculate_layout(data_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<BigBuf_SizedPrefix>();
            let (layout, data_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                    >(data_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), data_offset)
        }
    }
    #[doc(hidden)]
    pub struct __BigBufInitIter<__I0> {
        pub tag: u64,
        pub data: __I0,
        _marker: ::core::marker::PhantomData<BigBuf>,
    }
    unsafe impl<__I0> ::slice_struct::__private::SliceInit<BigBuf>
    for __BigBufInitIter<__I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[u8] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.len();
            BigBuf_LayoutHelper::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut BigBuf) {
            let (_, data_offset) = BigBuf_LayoutHelper::calculate_layout(
                (*ptr).0.__data.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut BigBuf {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<BigBuf_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<BigBuf> {
            let data_len = self.data.len();
            let (layout, data_offset) = BigBuf_LayoutHelper::calculate_layout(data_len);
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<BigBuf_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                BigBuf_SizedPrefix {
                    __tag: self.tag,
                    __align_0: [],
                    __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(data_offset)
                .cast::<<[u8] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.data.into_iter();
            for j in 0..data_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __BigBufInitDef {
        pub tag: u64,
        pub data: (<[u8] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<BigBuf>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<BigBuf> for __BigBufInitDef
    where
        <[u8] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.1;
            BigBuf_LayoutHelper::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut BigBuf) {
            let (_, data_offset) = BigBuf_LayoutHelper::calculate_layout(
                (*ptr).0.__data.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut BigBuf {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<BigBuf_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<BigBuf> {
            let data_len = self.data.1;
            let (layout, data_offset) = BigBuf_LayoutHelper::calculate_layout(data_len);
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<BigBuf_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                BigBuf_SizedPrefix {
                    __tag: self.tag,
                    __align_0: [],
                    __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.data.1;
            let field_ptr = ptr
                .add(data_offset)
                .cast::<<[u8] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.data.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl BigBuf {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            tag: u64,
            mut data: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[u8] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__BigBufInitIter {
                tag,
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            tag: u64,
            data: (<[u8] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[u8] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__BigBufInitDef {
                tag,
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_large_allocation"]
#[doc(hidden)]
pub const test_large_allocation: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_large_allocation"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 383usize,
        start_col: 4usize,
        end_line: 383usize,
        end_col: 25usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_large_allocation()),
    ),
};
fn test_large_allocation() {
    const N: usize = 1 << 20;
    let s = BigBuf::init_iter(0xdeadbeef, (0..N).map(|i| (i % 256) as u8)).in_box();
    match (&s.view().data.len(), &N) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().data[0], &0) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().data[255], &255) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().data[256], &0) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&s.view().data[N - 1], &(((N - 1) % 256) as u8)) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
use std::sync::Arc;
#[repr(C)]
#[doc(hidden)]
struct ArcSlice__Inner {
    #[doc(hidden)]
    __items_state: <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __items: ::slice_struct::__private::SliceHandle<
        <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct ArcSlice(ArcSlice__Inner);
impl ::core::ops::Drop for ArcSlice {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__items.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct ArcSliceView<'__a>
where
    [Arc<u32>]: '__a,
{
    items: <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct ArcSliceViewMut<'__a>
where
    [Arc<u32>]: '__a,
{
    items: <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for ArcSlice {
    type View<'__a> = ArcSliceView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        ArcSliceView {
            items: <[Arc<
                u32,
            >] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__items_state,
                this.0.__items.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for ArcSlice {
    type ViewMut<'__a> = ArcSliceViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            ArcSliceViewMut {
                items: <[Arc<
                    u32,
                >] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__items_state,
                    this.0.__items.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl ArcSlice {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> ArcSliceView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> ArcSliceViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct ArcSlice_SizedPrefix {
        #[doc(hidden)]
        __items_state: <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __items: ::slice_struct::__private::SliceHandle<
            <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct ArcSlice_LayoutHelper(::core::marker::PhantomData<ArcSlice>);
    impl ArcSlice_LayoutHelper {
        fn calculate_layout(items_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<ArcSlice_SizedPrefix>();
            let (layout, items_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
                    >(items_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), items_offset)
        }
    }
    #[doc(hidden)]
    pub struct __ArcSliceInitIter<__I0> {
        pub items: __I0,
        _marker: ::core::marker::PhantomData<ArcSlice>,
    }
    unsafe impl<__I0> ::slice_struct::__private::SliceInit<ArcSlice>
    for __ArcSliceInitIter<__I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let items_len = self.items.len();
            ArcSlice_LayoutHelper::calculate_layout(items_len).0
        }
        unsafe fn fixup(ptr: *mut ArcSlice) {
            let (_, items_offset) = ArcSlice_LayoutHelper::calculate_layout(
                (*ptr).0.__items.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__items.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, items_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut ArcSlice {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<ArcSlice_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<ArcSlice> {
            let items_len = self.items.len();
            let (layout, items_offset) = ArcSlice_LayoutHelper::calculate_layout(
                items_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<ArcSlice_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                ArcSlice_SizedPrefix {
                    __align_0: [],
                    __items_state: <[Arc<
                        u32,
                    >] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __items: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[Arc<
                                u32,
                            >] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: items_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(items_offset)
                .cast::<
                    <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
                >();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.items.into_iter();
            for j in 0..items_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __ArcSliceInitDef {
        pub items: (
            <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
            usize,
        ),
        _marker: ::core::marker::PhantomData<ArcSlice>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<ArcSlice> for __ArcSliceInitDef
    where
        <[Arc<
            u32,
        >] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let items_len = self.items.1;
            ArcSlice_LayoutHelper::calculate_layout(items_len).0
        }
        unsafe fn fixup(ptr: *mut ArcSlice) {
            let (_, items_offset) = ArcSlice_LayoutHelper::calculate_layout(
                (*ptr).0.__items.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__items.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, items_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut ArcSlice {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<ArcSlice_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<ArcSlice> {
            let items_len = self.items.1;
            let (layout, items_offset) = ArcSlice_LayoutHelper::calculate_layout(
                items_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<ArcSlice_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                ArcSlice_SizedPrefix {
                    __align_0: [],
                    __items_state: <[Arc<
                        u32,
                    >] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __items: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[Arc<
                                u32,
                            >] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: items_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.items.1;
            let field_ptr = ptr
                .add(items_offset)
                .cast::<
                    <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
                >();
            if def_len > 0 {
                let def_val = self.items.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl ArcSlice {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            mut items: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__ArcSliceInitIter {
                items,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            items: (
                <[Arc<u32>] as ::slice_struct::__private::InlineSlice>::Element,
                usize,
            ),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[Arc<
                u32,
            >] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__ArcSliceInitDef {
                items,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_arc_element_drop"]
#[doc(hidden)]
pub const test_arc_element_drop: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_arc_element_drop"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 404usize,
        start_col: 4usize,
        end_line: 404usize,
        end_col: 25usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_arc_element_drop()),
    ),
};
fn test_arc_element_drop() {
    let shared = Arc::new(42_u32);
    match (&Arc::strong_count(&shared), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    {
        let s = ArcSlice::init_iter(
                [shared.clone(), shared.clone(), shared.clone()].into_iter(),
            )
            .in_box();
        match (&Arc::strong_count(&shared), &4) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&*s.view().items[1], &42) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    match (&Arc::strong_count(&shared), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
struct PanicsAt {
    current: usize,
    panics_at: usize,
}
impl Iterator for PanicsAt {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.current == self.panics_at {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Deliberate panic at element {0}", self.current),
                );
            };
        }
        let v = self.current;
        self.current += 1;
        Some(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("item_{0}", v))
            }),
        )
    }
}
impl ExactSizeIterator for PanicsAt {
    fn len(&self) -> usize {
        self.panics_at + 1
    }
}
#[repr(C)]
#[doc(hidden)]
struct StringSlice__Inner {
    #[doc(hidden)]
    __words_state: <[String] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[String] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __words: ::slice_struct::__private::SliceHandle<
        <[String] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct StringSlice(StringSlice__Inner);
impl ::core::ops::Drop for StringSlice {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__words.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct StringSliceView<'__a>
where
    [String]: '__a,
{
    words: <[String] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct StringSliceViewMut<'__a>
where
    [String]: '__a,
{
    words: <[String] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for StringSlice {
    type View<'__a> = StringSliceView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        StringSliceView {
            words: <[String] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__words_state,
                this.0.__words.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for StringSlice {
    type ViewMut<'__a> = StringSliceViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            StringSliceViewMut {
                words: <[String] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__words_state,
                    this.0.__words.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl StringSlice {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> StringSliceView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(
        self: ::core::pin::Pin<&'__a mut Self>,
    ) -> StringSliceViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct StringSlice_SizedPrefix {
        #[doc(hidden)]
        __words_state: <[String] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[String] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __words: ::slice_struct::__private::SliceHandle<
            <[String] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct StringSlice_LayoutHelper(::core::marker::PhantomData<StringSlice>);
    impl StringSlice_LayoutHelper {
        fn calculate_layout(words_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<StringSlice_SizedPrefix>();
            let (layout, words_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[String] as ::slice_struct::__private::InlineSlice>::Element,
                    >(words_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), words_offset)
        }
    }
    #[doc(hidden)]
    pub struct __StringSliceInitIter<__I0> {
        pub words: __I0,
        _marker: ::core::marker::PhantomData<StringSlice>,
    }
    unsafe impl<__I0> ::slice_struct::__private::SliceInit<StringSlice>
    for __StringSliceInitIter<__I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[String] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let words_len = self.words.len();
            StringSlice_LayoutHelper::calculate_layout(words_len).0
        }
        unsafe fn fixup(ptr: *mut StringSlice) {
            let (_, words_offset) = StringSlice_LayoutHelper::calculate_layout(
                (*ptr).0.__words.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__words.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[String] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, words_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut StringSlice {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<StringSlice_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<StringSlice> {
            let words_len = self.words.len();
            let (layout, words_offset) = StringSlice_LayoutHelper::calculate_layout(
                words_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<StringSlice_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                StringSlice_SizedPrefix {
                    __align_0: [],
                    __words_state: <[String] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __words: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[String] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: words_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(words_offset)
                .cast::<<[String] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.words.into_iter();
            for j in 0..words_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __StringSliceInitDef {
        pub words: (
            <[String] as ::slice_struct::__private::InlineSlice>::Element,
            usize,
        ),
        _marker: ::core::marker::PhantomData<StringSlice>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<StringSlice>
    for __StringSliceInitDef
    where
        <[String] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let words_len = self.words.1;
            StringSlice_LayoutHelper::calculate_layout(words_len).0
        }
        unsafe fn fixup(ptr: *mut StringSlice) {
            let (_, words_offset) = StringSlice_LayoutHelper::calculate_layout(
                (*ptr).0.__words.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__words.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[String] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, words_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut StringSlice {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<StringSlice_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<StringSlice> {
            let words_len = self.words.1;
            let (layout, words_offset) = StringSlice_LayoutHelper::calculate_layout(
                words_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<StringSlice_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                StringSlice_SizedPrefix {
                    __align_0: [],
                    __words_state: <[String] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __words: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[String] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: words_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.words.1;
            let field_ptr = ptr
                .add(words_offset)
                .cast::<<[String] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.words.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl StringSlice {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            mut words: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[String] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__StringSliceInitIter {
                words,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            words: (<[String] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[String] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__StringSliceInitDef {
                words,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_panic_mid_iter_no_leak"]
#[doc(hidden)]
pub const test_panic_mid_iter_no_leak: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_panic_mid_iter_no_leak"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 446usize,
        start_col: 4usize,
        end_line: 446usize,
        end_col: 31usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_panic_mid_iter_no_leak()),
    ),
};
fn test_panic_mid_iter_no_leak() {
    DROP_COUNT.store(0, Ordering::SeqCst);
    struct Tracked(String);
    #[automatically_derived]
    impl ::core::clone::Clone for Tracked {
        #[inline]
        fn clone(&self) -> Tracked {
            Tracked(::core::clone::Clone::clone(&self.0))
        }
    }
    impl Drop for Tracked {
        fn drop(&mut self) {
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    struct TrackedSlice__Inner {
        #[doc(hidden)]
        __items_state: <[Tracked] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[Tracked] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __items: ::slice_struct::__private::SliceHandle<
            <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
        #[doc(hidden)]
        __data_tail: [::core::mem::MaybeUninit<u8>],
    }
    #[repr(transparent)]
    struct TrackedSlice(TrackedSlice__Inner);
    impl ::core::ops::Drop for TrackedSlice {
        fn drop(&mut self) {
            unsafe {
                let base_ptr = self as *mut _ as *mut u8;
                let data = self.0.__items.as_non_null(base_ptr);
                ::core::ptr::drop_in_place(data.as_ptr());
            }
        }
    }
    ///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
    struct TrackedSliceView<'__a>
    where
        [Tracked]: '__a,
    {
        items: <[Tracked] as ::slice_struct::__private::InlineSlice>::View<'__a>,
    }
    /**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
    struct TrackedSliceViewMut<'__a>
    where
        [Tracked]: '__a,
    {
        items: <[Tracked] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
    }
    impl ::slice_struct::AsView for TrackedSlice {
        type View<'__a> = TrackedSliceView<'__a> where Self: '__a;
        fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
            let this = self;
            let base_ptr = this as *const _ as *const u8;
            TrackedSliceView {
                items: <[Tracked] as ::slice_struct::__private::InlineSlice>::project(
                    &this.0.__items_state,
                    this.0.__items.as_non_null(base_ptr),
                ),
            }
        }
    }
    impl ::slice_struct::AsViewMut for TrackedSlice {
        type ViewMut<'__a> = TrackedSliceViewMut<'__a> where Self: '__a;
        fn as_view_mut<'__a>(
            self: ::core::pin::Pin<&'__a mut Self>,
        ) -> Self::ViewMut<'__a> {
            unsafe {
                let this = self.get_unchecked_mut();
                let base_ptr = this as *const _ as *const u8;
                TrackedSliceViewMut {
                    items: <[Tracked] as ::slice_struct::__private::InlineSlice>::project_mut(
                        &this.0.__items_state,
                        this.0.__items.as_non_null(base_ptr),
                    ),
                }
            }
        }
    }
    impl TrackedSlice {
        ///Returns a struct containing immutable references to all fields.
        fn view<'__a>(&'__a self) -> TrackedSliceView<'__a> {
            ::slice_struct::AsView::as_view(self)
        }
        ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
        fn view_mut<'__a>(
            self: ::core::pin::Pin<&'__a mut Self>,
        ) -> TrackedSliceViewMut<'__a> {
            ::slice_struct::AsViewMut::as_view_mut(self)
        }
    }
    const _: () = {
        #[repr(C)]
        struct TrackedSlice_SizedPrefix {
            #[doc(hidden)]
            __items_state: <[Tracked] as ::slice_struct::__private::InlineSlice>::State,
            #[doc(hidden)]
            __align_0: [<[Tracked] as ::slice_struct::__private::InlineSlice>::Element; 0],
            #[doc(hidden)]
            __items: ::slice_struct::__private::SliceHandle<
                <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                ::slice_struct::__private::AbsoluteMode,
            >,
            #[doc(hidden)]
            __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
        }
        #[allow(non_camel_case_types)]
        struct TrackedSlice_LayoutHelper(::core::marker::PhantomData<TrackedSlice>);
        impl TrackedSlice_LayoutHelper {
            fn calculate_layout(items_len: usize) -> (::std::alloc::Layout, usize) {
                let layout = ::std::alloc::Layout::new::<TrackedSlice_SizedPrefix>();
                let (layout, items_offset) = layout
                    .extend(
                        ::std::alloc::Layout::array::<
                            <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                        >(items_len)
                            .unwrap(),
                    )
                    .unwrap();
                (layout.pad_to_align(), items_offset)
            }
        }
        #[doc(hidden)]
        pub struct __TrackedSliceInitIter<__I0> {
            pub items: __I0,
            _marker: ::core::marker::PhantomData<TrackedSlice>,
        }
        unsafe impl<__I0> ::slice_struct::__private::SliceInit<TrackedSlice>
        for __TrackedSliceInitIter<__I0>
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            fn layout(&self) -> ::std::alloc::Layout {
                let items_len = self.items.len();
                TrackedSlice_LayoutHelper::calculate_layout(items_len).0
            }
            unsafe fn fixup(ptr: *mut TrackedSlice) {
                let (_, items_offset) = TrackedSlice_LayoutHelper::calculate_layout(
                    (*ptr).0.__items.len,
                );
                ::core::ptr::write(
                    &raw mut (*ptr).0.__items.ptr_data,
                    <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                        <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                    >(ptr as *mut u8, items_offset),
                );
            }
            fn make_fat_ptr(&self, base: *mut u8) -> *mut TrackedSlice {
                let data_len = self.layout().size()
                    - ::core::mem::size_of::<TrackedSlice_SizedPrefix>();
                ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len)
                    as *mut _
            }
            unsafe fn write_data(
                mut self,
                ptr: *mut u8,
            ) -> ::slice_struct::__private::OwnedDst<TrackedSlice> {
                let items_len = self.items.len();
                let (layout, items_offset) = TrackedSlice_LayoutHelper::calculate_layout(
                    items_len,
                );
                let fat_ptr = self.make_fat_ptr(ptr);
                let sized_ptr = ptr.cast::<TrackedSlice_SizedPrefix>();
                ::core::ptr::write(
                    sized_ptr,
                    TrackedSlice_SizedPrefix {
                        __align_0: [],
                        __items_state: <[Tracked] as ::slice_struct::__private::InlineSlice>::init_state(),
                        __items: ::slice_struct::__private::SliceHandle {
                            ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                                <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                            >(),
                            len: items_len,
                            _marker: ::core::marker::PhantomData,
                        },
                        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                    },
                );
                let field_ptr = ptr
                    .add(items_offset)
                    .cast::<
                        <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                    >();
                let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
                let mut iter = self.items.into_iter();
                for j in 0..items_len {
                    let item = iter
                        .next()
                        .expect(
                            "ExactSizeIterator yielded fewer elements than its len()",
                        );
                    ::core::ptr::write(field_ptr.add(j), item);
                    guard.written += 1;
                }
                ::core::mem::forget(guard);
                ::slice_struct::__private::OwnedDst {
                    ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                    layout,
                }
            }
        }
        #[doc(hidden)]
        pub struct __TrackedSliceInitDef {
            pub items: (
                <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                usize,
            ),
            _marker: ::core::marker::PhantomData<TrackedSlice>,
        }
        unsafe impl ::slice_struct::__private::SliceInit<TrackedSlice>
        for __TrackedSliceInitDef
        where
            <[Tracked] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            fn layout(&self) -> ::std::alloc::Layout {
                let items_len = self.items.1;
                TrackedSlice_LayoutHelper::calculate_layout(items_len).0
            }
            unsafe fn fixup(ptr: *mut TrackedSlice) {
                let (_, items_offset) = TrackedSlice_LayoutHelper::calculate_layout(
                    (*ptr).0.__items.len,
                );
                ::core::ptr::write(
                    &raw mut (*ptr).0.__items.ptr_data,
                    <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                        <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                    >(ptr as *mut u8, items_offset),
                );
            }
            fn make_fat_ptr(&self, base: *mut u8) -> *mut TrackedSlice {
                let data_len = self.layout().size()
                    - ::core::mem::size_of::<TrackedSlice_SizedPrefix>();
                ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len)
                    as *mut _
            }
            unsafe fn write_data(
                self,
                ptr: *mut u8,
            ) -> ::slice_struct::__private::OwnedDst<TrackedSlice> {
                let items_len = self.items.1;
                let (layout, items_offset) = TrackedSlice_LayoutHelper::calculate_layout(
                    items_len,
                );
                let fat_ptr = self.make_fat_ptr(ptr);
                let sized_ptr = ptr.cast::<TrackedSlice_SizedPrefix>();
                ::core::ptr::write(
                    sized_ptr,
                    TrackedSlice_SizedPrefix {
                        __align_0: [],
                        __items_state: <[Tracked] as ::slice_struct::__private::InlineSlice>::init_state(),
                        __items: ::slice_struct::__private::SliceHandle {
                            ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                                <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                            >(),
                            len: items_len,
                            _marker: ::core::marker::PhantomData,
                        },
                        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                    },
                );
                let def_len = self.items.1;
                let field_ptr = ptr
                    .add(items_offset)
                    .cast::<
                        <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                    >();
                if def_len > 0 {
                    let def_val = self.items.0;
                    for j in 0..def_len - 1 {
                        ::core::ptr::write(
                            field_ptr.add(j),
                            ::core::clone::Clone::clone(&def_val),
                        );
                    }
                    ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
                }
                ::slice_struct::__private::OwnedDst {
                    ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                    layout,
                }
            }
        }
        impl TrackedSlice {
            ///Initialize this struct from iterators, returning a builder.
            fn init_iter<__I0>(
                mut items: __I0,
            ) -> ::slice_struct::SliceBuilder<
                Self,
                impl ::slice_struct::__private::SliceInit<Self>,
            >
            where
                __I0: ::core::iter::ExactSizeIterator<
                    Item = <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                >,
            {
                ::slice_struct::SliceBuilder::new(__TrackedSliceInitIter {
                    items,
                    _marker: ::core::marker::PhantomData,
                })
            }
            ///Initialize this struct from cloned values, returning a builder.
            fn init_def(
                items: (
                    <[Tracked] as ::slice_struct::__private::InlineSlice>::Element,
                    usize,
                ),
            ) -> ::slice_struct::SliceBuilder<
                Self,
                impl ::slice_struct::__private::SliceInit<Self>,
            >
            where
                <[Tracked] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
            {
                ::slice_struct::SliceBuilder::new(__TrackedSliceInitDef {
                    items,
                    _marker: ::core::marker::PhantomData,
                })
            }
        }
    };
    struct TrackedPanicsAt {
        current: usize,
        panics_at: usize,
    }
    impl Iterator for TrackedPanicsAt {
        type Item = Tracked;
        fn next(&mut self) -> Option<Tracked> {
            if self.current == self.panics_at {
                {
                    ::core::panicking::panic_fmt(format_args!("deliberate"));
                };
            }
            self.current += 1;
            Some(
                Tracked(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("item"))
                    }),
                ),
            )
        }
    }
    impl ExactSizeIterator for TrackedPanicsAt {
        fn len(&self) -> usize {
            self.panics_at + 1
        }
    }
    let result = std::panic::catch_unwind(|| {
        let _s = TrackedSlice::init_iter(TrackedPanicsAt {
                current: 0,
                panics_at: 3,
            })
            .in_box();
    });
    if !result.is_err() {
        {
            ::core::panicking::panic_fmt(format_args!("should have panicked"));
        }
    }
    match (&DROP_COUNT.load(Ordering::SeqCst), &3) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(align(16))]
struct A16(u8);
#[automatically_derived]
impl ::core::clone::Clone for A16 {
    #[inline]
    fn clone(&self) -> A16 {
        A16(::core::clone::Clone::clone(&self.0))
    }
}
#[repr(align(32))]
struct A32(u8);
#[automatically_derived]
impl ::core::clone::Clone for A32 {
    #[inline]
    fn clone(&self) -> A32 {
        A32(::core::clone::Clone::clone(&self.0))
    }
}
#[repr(C)]
#[doc(hidden)]
struct TwoAligned__Inner {
    #[doc(hidden)]
    __tag: u8,
    #[doc(hidden)]
    __a_state: <[A16] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[A16] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __a: ::slice_struct::__private::SliceHandle<
        <[A16] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __b_state: <[A32] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_1: [<[A32] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __b: ::slice_struct::__private::SliceHandle<
        <[A32] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct TwoAligned(TwoAligned__Inner);
impl ::core::ops::Drop for TwoAligned {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__a.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__b.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct TwoAlignedView<'__a>
where
    [A16]: '__a,
    [A32]: '__a,
{
    tag: &'__a u8,
    a: <[A16] as ::slice_struct::__private::InlineSlice>::View<'__a>,
    b: <[A32] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct TwoAlignedViewMut<'__a>
where
    [A16]: '__a,
    [A32]: '__a,
{
    tag: &'__a mut u8,
    a: <[A16] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
    b: <[A32] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for TwoAligned {
    type View<'__a> = TwoAlignedView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        TwoAlignedView {
            tag: &this.0.__tag,
            a: <[A16] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__a_state,
                this.0.__a.as_non_null(base_ptr),
            ),
            b: <[A32] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__b_state,
                this.0.__b.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for TwoAligned {
    type ViewMut<'__a> = TwoAlignedViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            TwoAlignedViewMut {
                tag: &mut this.0.__tag,
                a: <[A16] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__a_state,
                    this.0.__a.as_non_null(base_ptr),
                ),
                b: <[A32] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__b_state,
                    this.0.__b.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl TwoAligned {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> TwoAlignedView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(
        self: ::core::pin::Pin<&'__a mut Self>,
    ) -> TwoAlignedViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct TwoAligned_SizedPrefix {
        #[doc(hidden)]
        __tag: u8,
        #[doc(hidden)]
        __a_state: <[A16] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[A16] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __a: ::slice_struct::__private::SliceHandle<
            <[A16] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __b_state: <[A32] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_1: [<[A32] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __b: ::slice_struct::__private::SliceHandle<
            <[A32] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct TwoAligned_LayoutHelper(::core::marker::PhantomData<TwoAligned>);
    impl TwoAligned_LayoutHelper {
        fn calculate_layout(
            a_len: usize,
            b_len: usize,
        ) -> (::std::alloc::Layout, usize, usize) {
            let layout = ::std::alloc::Layout::new::<TwoAligned_SizedPrefix>();
            let (layout, a_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[A16] as ::slice_struct::__private::InlineSlice>::Element,
                    >(a_len)
                        .unwrap(),
                )
                .unwrap();
            let (layout, b_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[A32] as ::slice_struct::__private::InlineSlice>::Element,
                    >(b_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), a_offset, b_offset)
        }
    }
    #[doc(hidden)]
    pub struct __TwoAlignedInitIter<__I0, __I1> {
        pub tag: u8,
        pub a: __I0,
        pub b: __I1,
        _marker: ::core::marker::PhantomData<TwoAligned>,
    }
    unsafe impl<__I0, __I1> ::slice_struct::__private::SliceInit<TwoAligned>
    for __TwoAlignedInitIter<__I0, __I1>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[A16] as ::slice_struct::__private::InlineSlice>::Element,
        >,
        __I1: ::core::iter::ExactSizeIterator<
            Item = <[A32] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let a_len = self.a.len();
            let b_len = self.b.len();
            TwoAligned_LayoutHelper::calculate_layout(a_len, b_len).0
        }
        unsafe fn fixup(ptr: *mut TwoAligned) {
            let (_, a_offset, b_offset) = TwoAligned_LayoutHelper::calculate_layout(
                (*ptr).0.__a.len,
                (*ptr).0.__b.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__a.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[A16] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, a_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__b.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[A32] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, b_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut TwoAligned {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<TwoAligned_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<TwoAligned> {
            let a_len = self.a.len();
            let b_len = self.b.len();
            let (layout, a_offset, b_offset) = TwoAligned_LayoutHelper::calculate_layout(
                a_len,
                b_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<TwoAligned_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                TwoAligned_SizedPrefix {
                    __tag: self.tag,
                    __align_0: [],
                    __align_1: [],
                    __a_state: <[A16] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __a: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[A16] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: a_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __b_state: <[A32] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __b: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[A32] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: b_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(a_offset)
                .cast::<<[A16] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.a.into_iter();
            for j in 0..a_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            let field_ptr = ptr
                .add(b_offset)
                .cast::<<[A32] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.b.into_iter();
            for j in 0..b_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __TwoAlignedInitDef {
        pub tag: u8,
        pub a: (<[A16] as ::slice_struct::__private::InlineSlice>::Element, usize),
        pub b: (<[A32] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<TwoAligned>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<TwoAligned> for __TwoAlignedInitDef
    where
        <[A16] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        <[A32] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let a_len = self.a.1;
            let b_len = self.b.1;
            TwoAligned_LayoutHelper::calculate_layout(a_len, b_len).0
        }
        unsafe fn fixup(ptr: *mut TwoAligned) {
            let (_, a_offset, b_offset) = TwoAligned_LayoutHelper::calculate_layout(
                (*ptr).0.__a.len,
                (*ptr).0.__b.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__a.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[A16] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, a_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__b.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[A32] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, b_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut TwoAligned {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<TwoAligned_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<TwoAligned> {
            let a_len = self.a.1;
            let b_len = self.b.1;
            let (layout, a_offset, b_offset) = TwoAligned_LayoutHelper::calculate_layout(
                a_len,
                b_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<TwoAligned_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                TwoAligned_SizedPrefix {
                    __tag: self.tag,
                    __align_0: [],
                    __align_1: [],
                    __a_state: <[A16] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __a: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[A16] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: a_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __b_state: <[A32] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __b: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[A32] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: b_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.a.1;
            let field_ptr = ptr
                .add(a_offset)
                .cast::<<[A16] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.a.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            let def_len = self.b.1;
            let field_ptr = ptr
                .add(b_offset)
                .cast::<<[A32] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.b.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl TwoAligned {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0, __I1>(
            tag: u8,
            mut a: __I0,
            mut b: __I1,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[A16] as ::slice_struct::__private::InlineSlice>::Element,
            >,
            __I1: ::core::iter::ExactSizeIterator<
                Item = <[A32] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__TwoAlignedInitIter {
                tag,
                a,
                b,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            tag: u8,
            a: (<[A16] as ::slice_struct::__private::InlineSlice>::Element, usize),
            b: (<[A32] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[A16] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
            <[A32] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__TwoAlignedInitDef {
                tag,
                a,
                b,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_two_aligned_slices"]
#[doc(hidden)]
pub const test_two_aligned_slices: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_two_aligned_slices"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 493usize,
        start_col: 4usize,
        end_line: 493usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_two_aligned_slices()),
    ),
};
fn test_two_aligned_slices() {
    let s = TwoAligned::init_def(7, (A16(1), 4), (A32(2), 2)).in_box();
    let ptr_a = s.view().a.as_ptr() as usize;
    let ptr_b = s.view().b.as_ptr() as usize;
    match (&(ptr_a % 16), &0) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::Some(
                        format_args!("A16 slice not 16-byte aligned"),
                    ),
                );
            }
        }
    };
    match (&(ptr_b % 32), &0) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::Some(
                        format_args!("A32 slice not 32-byte aligned"),
                    ),
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_iter_mut_all_elements"]
#[doc(hidden)]
pub const test_iter_mut_all_elements: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_iter_mut_all_elements"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 504usize,
        start_col: 4usize,
        end_line: 504usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_iter_mut_all_elements()),
    ),
};
fn test_iter_mut_all_elements() {
    let mut s = Str::<i32>::init_def(0, (1, 100), (2, 100)).in_box();
    {
        let v = s.as_mut().view_mut();
        v.b.iter_mut().enumerate().for_each(|(i, x)| *x = i as i32);
        v.c.iter_mut().enumerate().for_each(|(i, x)| *x = (i * 2) as u32);
    }
    let v = s.view();
    match (&v.b[0], &0) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&v.b[99], &99) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&v.c[0], &0) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&v.c[99], &198) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(C)]
#[doc(hidden)]
struct AllSized__Inner {
    #[doc(hidden)]
    __x: i64,
    #[doc(hidden)]
    __y: f64,
    #[doc(hidden)]
    __z: bool,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct AllSized(AllSized__Inner);
impl ::core::ops::Drop for AllSized {
    fn drop(&mut self) {}
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct AllSizedView<'__a> {
    pub x: &'__a i64,
    pub y: &'__a f64,
    pub z: &'__a bool,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct AllSizedViewMut<'__a> {
    pub x: &'__a mut i64,
    pub y: &'__a mut f64,
    pub z: &'__a mut bool,
}
impl ::slice_struct::AsView for AllSized {
    type View<'__a> = AllSizedView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        AllSizedView {
            x: &this.0.__x,
            y: &this.0.__y,
            z: &this.0.__z,
        }
    }
}
impl ::slice_struct::AsViewMut for AllSized {
    type ViewMut<'__a> = AllSizedViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            AllSizedViewMut {
                x: &mut this.0.__x,
                y: &mut this.0.__y,
                z: &mut this.0.__z,
            }
        }
    }
}
impl AllSized {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> AllSizedView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> AllSizedViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct AllSized_SizedPrefix {
        #[doc(hidden)]
        __x: i64,
        #[doc(hidden)]
        __y: f64,
        #[doc(hidden)]
        __z: bool,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct AllSized_LayoutHelper(::core::marker::PhantomData<AllSized>);
    impl AllSized_LayoutHelper {
        fn calculate_layout() -> (::std::alloc::Layout,) {
            let layout = ::std::alloc::Layout::new::<AllSized_SizedPrefix>();
            (layout.pad_to_align(),)
        }
    }
    #[doc(hidden)]
    pub struct __AllSizedInitIter {
        pub x: i64,
        pub y: f64,
        pub z: bool,
        _marker: ::core::marker::PhantomData<AllSized>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<AllSized> for __AllSizedInitIter {
        fn layout(&self) -> ::std::alloc::Layout {
            AllSized_LayoutHelper::calculate_layout().0
        }
        unsafe fn fixup(ptr: *mut AllSized) {
            let (_,) = AllSized_LayoutHelper::calculate_layout();
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut AllSized {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<AllSized_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<AllSized> {
            let (layout,) = AllSized_LayoutHelper::calculate_layout();
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<AllSized_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                AllSized_SizedPrefix {
                    __x: self.x,
                    __y: self.y,
                    __z: self.z,
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __AllSizedInitDef {
        pub x: i64,
        pub y: f64,
        pub z: bool,
        _marker: ::core::marker::PhantomData<AllSized>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<AllSized> for __AllSizedInitDef {
        fn layout(&self) -> ::std::alloc::Layout {
            AllSized_LayoutHelper::calculate_layout().0
        }
        unsafe fn fixup(ptr: *mut AllSized) {
            let (_,) = AllSized_LayoutHelper::calculate_layout();
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut AllSized {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<AllSized_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<AllSized> {
            let (layout,) = AllSized_LayoutHelper::calculate_layout();
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<AllSized_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                AllSized_SizedPrefix {
                    __x: self.x,
                    __y: self.y,
                    __z: self.z,
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl AllSized {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter(
            x: i64,
            y: f64,
            z: bool,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        > {
            ::slice_struct::SliceBuilder::new(__AllSizedInitIter {
                x,
                y,
                z,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            x: i64,
            y: f64,
            z: bool,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        > {
            ::slice_struct::SliceBuilder::new(__AllSizedInitDef {
                x,
                y,
                z,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_all_sized_fields_read_write"]
#[doc(hidden)]
pub const test_all_sized_fields_read_write: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_all_sized_fields_read_write"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 529usize,
        start_col: 4usize,
        end_line: 529usize,
        end_col: 36usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_all_sized_fields_read_write()),
    ),
};
fn test_all_sized_fields_read_write() {
    let mut s = AllSized::init_def(1_i64, 3.14_f64, true).in_box();
    match (&*s.view().x, &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !((s.view().y - 3.14).abs() < 1e-10) {
        ::core::panicking::panic("assertion failed: (s.view().y - 3.14).abs() < 1e-10")
    }
    match (&*s.view().z, &true) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    {
        let v = s.as_mut().view_mut();
        *v.x = -99;
        *v.y = 2.718;
        *v.z = false;
    }
    match (&*s.view().x, &-99) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !((s.view().y - 2.718).abs() < 1e-10) {
        ::core::panicking::panic("assertion failed: (s.view().y - 2.718).abs() < 1e-10")
    }
    match (&*s.view().z, &false) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_sliceborrow_coercions"]
#[doc(hidden)]
pub const test_sliceborrow_coercions: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_sliceborrow_coercions"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 550usize,
        start_col: 4usize,
        end_line: 550usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_sliceborrow_coercions()),
    ),
};
fn test_sliceborrow_coercions() {
    fn takes_slice(s: &[i32]) -> i32 {
        s.iter().sum()
    }
    fn takes_mut_slice(s: &mut [i32]) {
        s.iter_mut().for_each(|x| *x *= 2);
    }
    let mut s = Str::<i32>::init_iter(0, [1, 2, 3, 4].into_iter(), [0].into_iter())
        .in_box();
    {
        let mut v = s.as_mut().view_mut();
        let sum = takes_slice(&v.b);
        match (&sum, &10) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        takes_mut_slice(&mut v.b);
    }
    match (&s.view().b, &&[2, 4, 6, 8]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[repr(C)]
#[doc(hidden)]
struct WrapTest__Inner {
    #[doc(hidden)]
    __counter: u32,
    #[doc(hidden)]
    __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<[u8] as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __data: ::slice_struct::__private::SliceHandle<
        <[u8] as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct WrapTest(WrapTest__Inner);
impl ::core::ops::Drop for WrapTest {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__data.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct WrapTestView<'__a>
where
    [u8]: '__a,
{
    pub counter: &'__a u32,
    pub data: <[u8] as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct WrapTestViewMut<'__a>
where
    [u8]: '__a,
{
    pub counter: &'__a mut u32,
    pub data: <[u8] as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for WrapTest {
    type View<'__a> = WrapTestView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        WrapTestView {
            counter: &this.0.__counter,
            data: <[u8] as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__data_state,
                this.0.__data.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for WrapTest {
    type ViewMut<'__a> = WrapTestViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            WrapTestViewMut {
                counter: &mut this.0.__counter,
                data: <[u8] as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__data_state,
                    this.0.__data.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl WrapTest {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> WrapTestView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> WrapTestViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct WrapTest_SizedPrefix {
        #[doc(hidden)]
        __counter: u32,
        #[doc(hidden)]
        __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<[u8] as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __data: ::slice_struct::__private::SliceHandle<
            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct WrapTest_LayoutHelper(::core::marker::PhantomData<WrapTest>);
    impl WrapTest_LayoutHelper {
        fn calculate_layout(data_len: usize) -> (::std::alloc::Layout, usize) {
            let layout = ::std::alloc::Layout::new::<WrapTest_SizedPrefix>();
            let (layout, data_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                    >(data_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), data_offset)
        }
    }
    #[doc(hidden)]
    pub struct __WrapTestInitIter<__I0> {
        pub counter: u32,
        pub data: __I0,
        _marker: ::core::marker::PhantomData<WrapTest>,
    }
    unsafe impl<__I0> ::slice_struct::__private::SliceInit<WrapTest>
    for __WrapTestInitIter<__I0>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <[u8] as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.len();
            WrapTest_LayoutHelper::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut WrapTest) {
            let (_, data_offset) = WrapTest_LayoutHelper::calculate_layout(
                (*ptr).0.__data.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut WrapTest {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<WrapTest_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<WrapTest> {
            let data_len = self.data.len();
            let (layout, data_offset) = WrapTest_LayoutHelper::calculate_layout(
                data_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<WrapTest_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                WrapTest_SizedPrefix {
                    __counter: self.counter,
                    __align_0: [],
                    __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(data_offset)
                .cast::<<[u8] as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.data.into_iter();
            for j in 0..data_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __WrapTestInitDef {
        pub counter: u32,
        pub data: (<[u8] as ::slice_struct::__private::InlineSlice>::Element, usize),
        _marker: ::core::marker::PhantomData<WrapTest>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<WrapTest> for __WrapTestInitDef
    where
        <[u8] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let data_len = self.data.1;
            WrapTest_LayoutHelper::calculate_layout(data_len).0
        }
        unsafe fn fixup(ptr: *mut WrapTest) {
            let (_, data_offset) = WrapTest_LayoutHelper::calculate_layout(
                (*ptr).0.__data.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__data.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, data_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut WrapTest {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<WrapTest_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<WrapTest> {
            let data_len = self.data.1;
            let (layout, data_offset) = WrapTest_LayoutHelper::calculate_layout(
                data_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<WrapTest_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                WrapTest_SizedPrefix {
                    __counter: self.counter,
                    __align_0: [],
                    __data_state: <[u8] as ::slice_struct::__private::InlineSlice>::init_state(),
                    __data: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <[u8] as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: data_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.data.1;
            let field_ptr = ptr
                .add(data_offset)
                .cast::<<[u8] as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.data.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl WrapTest {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0>(
            counter: u32,
            mut data: __I0,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <[u8] as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__WrapTestInitIter {
                counter,
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            counter: u32,
            data: (<[u8] as ::slice_struct::__private::InlineSlice>::Element, usize),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <[u8] as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__WrapTestInitDef {
                counter,
                data,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_in_arc"]
#[doc(hidden)]
pub const test_in_arc: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_in_arc"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 574usize,
        start_col: 4usize,
        end_line: 574usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_in_arc()),
    ),
};
fn test_in_arc() {
    let arc = WrapTest::init_def(10, (5, 4)).in_arc();
    match (&*arc.view().counter, &10) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&arc.view().data, &&[5, 5, 5, 5]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let clone1 = arc.clone();
    match (&clone1.view().data.len(), &4) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let raw_arc = unsafe { std::pin::Pin::into_inner_unchecked(arc) };
    match (&Arc::strong_count(&raw_arc), &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_in_rc"]
#[doc(hidden)]
pub const test_in_rc: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_in_rc"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 587usize,
        start_col: 4usize,
        end_line: 587usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_in_rc()),
    ),
};
fn test_in_rc() {
    let rc = WrapTest::init_def(99, (1, 2)).in_rc();
    let clone1 = rc.clone();
    match (&clone1.view().data, &&[1, 1]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let raw_rc = unsafe { std::pin::Pin::into_inner_unchecked(rc) };
    match (&std::rc::Rc::strong_count(&raw_rc), &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_arc_mutex_concurrency"]
#[doc(hidden)]
pub const test_arc_mutex_concurrency: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_arc_mutex_concurrency"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 598usize,
        start_col: 4usize,
        end_line: 598usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_arc_mutex_concurrency()),
    ),
};
fn test_arc_mutex_concurrency() {
    use std::thread;
    let s = WrapTest::init_def(0, (0, 100)).with_mutex().in_arc();
    let mut handles = ::alloc::vec::Vec::new();
    for _ in 0..10 {
        let s_clone = s.clone();
        handles
            .push(
                thread::spawn(move || {
                    let mut guard = s_clone.lock();
                    let view = guard.as_mut().view_mut();
                    *view.counter += 1;
                    for i in 0..100 {
                        view.data[i] += 1;
                    }
                }),
            );
    }
    for h in handles {
        h.join().unwrap();
    }
    let guard = s.lock();
    let view = guard.view();
    match (&*view.counter, &10) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    for i in 0..100 {
        match (&view.data[i], &10) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
}
extern crate test;
#[rustc_test_marker = "test_rc_refcell"]
#[doc(hidden)]
pub const test_rc_refcell: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_rc_refcell"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 619usize,
        start_col: 4usize,
        end_line: 619usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_rc_refcell()),
    ),
};
fn test_rc_refcell() {
    let rc = WrapTest::init_def(0, (0, 10)).with_refcell().in_rc();
    {
        let mut guard = rc.borrow_mut();
        let view = guard.as_mut().view_mut();
        *view.counter = 42;
        view.data[0] = 99;
    }
    let view = rc.borrow();
    match (&*view.view().counter, &42) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&view.view().data[0], &99) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
use std::sync::Mutex;
use std::cell::RefCell;
#[repr(C)]
#[doc(hidden)]
struct Advanced__Inner {
    #[doc(hidden)]
    __string_state: <str as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_0: [<str as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __string: ::slice_struct::__private::SliceHandle<
        <str as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __locked_state: <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_1: [<Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __locked: ::slice_struct::__private::SliceHandle<
        <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __cell_state: <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::State,
    #[doc(hidden)]
    __align_2: [<RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::Element; 0],
    #[doc(hidden)]
    __cell: ::slice_struct::__private::SliceHandle<
        <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::Element,
        ::slice_struct::__private::AbsoluteMode,
    >,
    #[doc(hidden)]
    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    #[doc(hidden)]
    __data_tail: [::core::mem::MaybeUninit<u8>],
}
#[repr(transparent)]
struct Advanced(Advanced__Inner);
impl ::core::ops::Drop for Advanced {
    fn drop(&mut self) {
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__string.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__locked.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
        unsafe {
            let base_ptr = self as *mut _ as *mut u8;
            let data = self.0.__cell.as_non_null(base_ptr);
            ::core::ptr::drop_in_place(data.as_ptr());
        }
    }
}
///An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields.
struct AdvancedView<'__a>
where
    str: '__a,
    Mutex<[u8]>: '__a,
    RefCell<[u32]>: '__a,
{
    string: <str as ::slice_struct::__private::InlineSlice>::View<'__a>,
    locked: <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::View<'__a>,
    cell: <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::View<'__a>,
}
/**A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.

This struct enables safe disjoint borrowing of multiple slices simultaneously.*/
struct AdvancedViewMut<'__a>
where
    str: '__a,
    Mutex<[u8]>: '__a,
    RefCell<[u32]>: '__a,
{
    string: <str as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
    locked: <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
    cell: <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::ViewMut<'__a>,
}
impl ::slice_struct::AsView for Advanced {
    type View<'__a> = AdvancedView<'__a> where Self: '__a;
    fn as_view<'__a>(&'__a self) -> Self::View<'__a> {
        let this = self;
        let base_ptr = this as *const _ as *const u8;
        AdvancedView {
            string: <str as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__string_state,
                this.0.__string.as_non_null(base_ptr),
            ),
            locked: <Mutex<
                [u8],
            > as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__locked_state,
                this.0.__locked.as_non_null(base_ptr),
            ),
            cell: <RefCell<
                [u32],
            > as ::slice_struct::__private::InlineSlice>::project(
                &this.0.__cell_state,
                this.0.__cell.as_non_null(base_ptr),
            ),
        }
    }
}
impl ::slice_struct::AsViewMut for Advanced {
    type ViewMut<'__a> = AdvancedViewMut<'__a> where Self: '__a;
    fn as_view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> Self::ViewMut<'__a> {
        unsafe {
            let this = self.get_unchecked_mut();
            let base_ptr = this as *const _ as *const u8;
            AdvancedViewMut {
                string: <str as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__string_state,
                    this.0.__string.as_non_null(base_ptr),
                ),
                locked: <Mutex<
                    [u8],
                > as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__locked_state,
                    this.0.__locked.as_non_null(base_ptr),
                ),
                cell: <RefCell<
                    [u32],
                > as ::slice_struct::__private::InlineSlice>::project_mut(
                    &this.0.__cell_state,
                    this.0.__cell.as_non_null(base_ptr),
                ),
            }
        }
    }
}
impl Advanced {
    ///Returns a struct containing immutable references to all fields.
    fn view<'__a>(&'__a self) -> AdvancedView<'__a> {
        ::slice_struct::AsView::as_view(self)
    }
    ///Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields.
    fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> AdvancedViewMut<'__a> {
        ::slice_struct::AsViewMut::as_view_mut(self)
    }
}
const _: () = {
    #[repr(C)]
    struct Advanced_SizedPrefix {
        #[doc(hidden)]
        __string_state: <str as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_0: [<str as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __string: ::slice_struct::__private::SliceHandle<
            <str as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __locked_state: <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_1: [<Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __locked: ::slice_struct::__private::SliceHandle<
            <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __cell_state: <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::State,
        #[doc(hidden)]
        __align_2: [<RefCell<
            [u32],
        > as ::slice_struct::__private::InlineSlice>::Element; 0],
        #[doc(hidden)]
        __cell: ::slice_struct::__private::SliceHandle<
            <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::AbsoluteMode,
        >,
        #[doc(hidden)]
        __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::Marker,
    }
    #[allow(non_camel_case_types)]
    struct Advanced_LayoutHelper(::core::marker::PhantomData<Advanced>);
    impl Advanced_LayoutHelper {
        fn calculate_layout(
            string_len: usize,
            locked_len: usize,
            cell_len: usize,
        ) -> (::std::alloc::Layout, usize, usize, usize) {
            let layout = ::std::alloc::Layout::new::<Advanced_SizedPrefix>();
            let (layout, string_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <str as ::slice_struct::__private::InlineSlice>::Element,
                    >(string_len)
                        .unwrap(),
                )
                .unwrap();
            let (layout, locked_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
                    >(locked_len)
                        .unwrap(),
                )
                .unwrap();
            let (layout, cell_offset) = layout
                .extend(
                    ::std::alloc::Layout::array::<
                        <RefCell<
                            [u32],
                        > as ::slice_struct::__private::InlineSlice>::Element,
                    >(cell_len)
                        .unwrap(),
                )
                .unwrap();
            (layout.pad_to_align(), string_offset, locked_offset, cell_offset)
        }
    }
    #[doc(hidden)]
    pub struct __AdvancedInitIter<__I0, __I1, __I2> {
        pub string: __I0,
        pub locked: __I1,
        pub cell: __I2,
        _marker: ::core::marker::PhantomData<Advanced>,
    }
    unsafe impl<__I0, __I1, __I2> ::slice_struct::__private::SliceInit<Advanced>
    for __AdvancedInitIter<__I0, __I1, __I2>
    where
        __I0: ::core::iter::ExactSizeIterator<
            Item = <str as ::slice_struct::__private::InlineSlice>::Element,
        >,
        __I1: ::core::iter::ExactSizeIterator<
            Item = <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
        >,
        __I2: ::core::iter::ExactSizeIterator<
            Item = <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::Element,
        >,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let string_len = self.string.len();
            let locked_len = self.locked.len();
            let cell_len = self.cell.len();
            Advanced_LayoutHelper::calculate_layout(string_len, locked_len, cell_len).0
        }
        unsafe fn fixup(ptr: *mut Advanced) {
            let (_, string_offset, locked_offset, cell_offset) = Advanced_LayoutHelper::calculate_layout(
                (*ptr).0.__string.len,
                (*ptr).0.__locked.len,
                (*ptr).0.__cell.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__string.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <str as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, string_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__locked.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, locked_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__cell.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, cell_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut Advanced {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<Advanced_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            mut self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<Advanced> {
            let string_len = self.string.len();
            let locked_len = self.locked.len();
            let cell_len = self.cell.len();
            let (layout, string_offset, locked_offset, cell_offset) = Advanced_LayoutHelper::calculate_layout(
                string_len,
                locked_len,
                cell_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<Advanced_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                Advanced_SizedPrefix {
                    __align_0: [],
                    __align_1: [],
                    __align_2: [],
                    __string_state: <str as ::slice_struct::__private::InlineSlice>::init_state(),
                    __string: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <str as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: string_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __locked_state: <Mutex<
                        [u8],
                    > as ::slice_struct::__private::InlineSlice>::init_state(),
                    __locked: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <Mutex<
                                [u8],
                            > as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: locked_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __cell_state: <RefCell<
                        [u32],
                    > as ::slice_struct::__private::InlineSlice>::init_state(),
                    __cell: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <RefCell<
                                [u32],
                            > as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: cell_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let field_ptr = ptr
                .add(string_offset)
                .cast::<<str as ::slice_struct::__private::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.string.into_iter();
            for j in 0..string_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            let field_ptr = ptr
                .add(locked_offset)
                .cast::<
                    <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
                >();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.locked.into_iter();
            for j in 0..locked_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            let field_ptr = ptr
                .add(cell_offset)
                .cast::<
                    <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::Element,
                >();
            let mut guard = ::slice_struct::__private::__DropGuard::new(field_ptr);
            let mut iter = self.cell.into_iter();
            for j in 0..cell_len {
                let item = iter
                    .next()
                    .expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    #[doc(hidden)]
    pub struct __AdvancedInitDef {
        pub string: (<str as ::slice_struct::__private::InlineSlice>::Element, usize),
        pub locked: (
            <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
            usize,
        ),
        pub cell: (
            <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::Element,
            usize,
        ),
        _marker: ::core::marker::PhantomData<Advanced>,
    }
    unsafe impl ::slice_struct::__private::SliceInit<Advanced> for __AdvancedInitDef
    where
        <str as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        <Mutex<
            [u8],
        > as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        <RefCell<
            [u32],
        > as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
    {
        fn layout(&self) -> ::std::alloc::Layout {
            let string_len = self.string.1;
            let locked_len = self.locked.1;
            let cell_len = self.cell.1;
            Advanced_LayoutHelper::calculate_layout(string_len, locked_len, cell_len).0
        }
        unsafe fn fixup(ptr: *mut Advanced) {
            let (_, string_offset, locked_offset, cell_offset) = Advanced_LayoutHelper::calculate_layout(
                (*ptr).0.__string.len,
                (*ptr).0.__locked.len,
                (*ptr).0.__cell.len,
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__string.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <str as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, string_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__locked.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, locked_offset),
            );
            ::core::ptr::write(
                &raw mut (*ptr).0.__cell.ptr_data,
                <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::store::<
                    <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::Element,
                >(ptr as *mut u8, cell_offset),
            );
        }
        fn make_fat_ptr(&self, base: *mut u8) -> *mut Advanced {
            let data_len = self.layout().size()
                - ::core::mem::size_of::<Advanced_SizedPrefix>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
        unsafe fn write_data(
            self,
            ptr: *mut u8,
        ) -> ::slice_struct::__private::OwnedDst<Advanced> {
            let string_len = self.string.1;
            let locked_len = self.locked.1;
            let cell_len = self.cell.1;
            let (layout, string_offset, locked_offset, cell_offset) = Advanced_LayoutHelper::calculate_layout(
                string_len,
                locked_len,
                cell_len,
            );
            let fat_ptr = self.make_fat_ptr(ptr);
            let sized_ptr = ptr.cast::<Advanced_SizedPrefix>();
            ::core::ptr::write(
                sized_ptr,
                Advanced_SizedPrefix {
                    __align_0: [],
                    __align_1: [],
                    __align_2: [],
                    __string_state: <str as ::slice_struct::__private::InlineSlice>::init_state(),
                    __string: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <str as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: string_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __locked_state: <Mutex<
                        [u8],
                    > as ::slice_struct::__private::InlineSlice>::init_state(),
                    __locked: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <Mutex<
                                [u8],
                            > as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: locked_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __cell_state: <RefCell<
                        [u32],
                    > as ::slice_struct::__private::InlineSlice>::init_state(),
                    __cell: ::slice_struct::__private::SliceHandle {
                        ptr_data: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::dummy::<
                            <RefCell<
                                [u32],
                            > as ::slice_struct::__private::InlineSlice>::Element,
                        >(),
                        len: cell_len,
                        _marker: ::core::marker::PhantomData,
                    },
                    __pin: <::slice_struct::__private::AbsoluteMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT,
                },
            );
            let def_len = self.string.1;
            let field_ptr = ptr
                .add(string_offset)
                .cast::<<str as ::slice_struct::__private::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.string.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            let def_len = self.locked.1;
            let field_ptr = ptr
                .add(locked_offset)
                .cast::<
                    <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
                >();
            if def_len > 0 {
                let def_val = self.locked.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            let def_len = self.cell.1;
            let field_ptr = ptr
                .add(cell_offset)
                .cast::<
                    <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::Element,
                >();
            if def_len > 0 {
                let def_val = self.cell.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(
                        field_ptr.add(j),
                        ::core::clone::Clone::clone(&def_val),
                    );
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
            ::slice_struct::__private::OwnedDst {
                ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                layout,
            }
        }
    }
    impl Advanced {
        ///Initialize this struct from iterators, returning a builder.
        fn init_iter<__I0, __I1, __I2>(
            mut string: __I0,
            mut locked: __I1,
            mut cell: __I2,
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            __I0: ::core::iter::ExactSizeIterator<
                Item = <str as ::slice_struct::__private::InlineSlice>::Element,
            >,
            __I1: ::core::iter::ExactSizeIterator<
                Item = <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
            >,
            __I2: ::core::iter::ExactSizeIterator<
                Item = <RefCell<
                    [u32],
                > as ::slice_struct::__private::InlineSlice>::Element,
            >,
        {
            ::slice_struct::SliceBuilder::new(__AdvancedInitIter {
                string,
                locked,
                cell,
                _marker: ::core::marker::PhantomData,
            })
        }
        ///Initialize this struct from cloned values, returning a builder.
        fn init_def(
            string: (<str as ::slice_struct::__private::InlineSlice>::Element, usize),
            locked: (
                <Mutex<[u8]> as ::slice_struct::__private::InlineSlice>::Element,
                usize,
            ),
            cell: (
                <RefCell<[u32]> as ::slice_struct::__private::InlineSlice>::Element,
                usize,
            ),
        ) -> ::slice_struct::SliceBuilder<
            Self,
            impl ::slice_struct::__private::SliceInit<Self>,
        >
        where
            <str as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
            <Mutex<
                [u8],
            > as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
            <RefCell<
                [u32],
            > as ::slice_struct::__private::InlineSlice>::Element: ::core::clone::Clone,
        {
            ::slice_struct::SliceBuilder::new(__AdvancedInitDef {
                string,
                locked,
                cell,
                _marker: ::core::marker::PhantomData,
            })
        }
    }
};
extern crate test;
#[rustc_test_marker = "test_advanced"]
#[doc(hidden)]
pub const test_advanced: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_advanced"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\test.rs",
        start_line: 646usize,
        start_col: 4usize,
        end_line: 646usize,
        end_col: 17usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_advanced()),
    ),
};
fn test_advanced() {
    let a = Advanced::init_iter(
            "hello".bytes(),
            [1, 2, 3].into_iter(),
            [100, 200].into_iter(),
        )
        .in_box();
    match (&a.view().string, &"hello") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&*a.view().locked, &[1, 2, 3]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&*a.view().cell, &[100, 200]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    a.view().locked[0] = 99;
    a.view().cell[1] = 999;
    match (&*a.view().locked, &[99, 2, 3]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&*a.view().cell, &[100, 999]) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &test_advanced,
            &test_all_sized_fields_read_write,
            &test_arc_element_drop,
            &test_arc_mutex_concurrency,
            &test_def_basic,
            &test_def_drop,
            &test_def_zero_len,
            &test_exact_drop_count,
            &test_highly_aligned,
            &test_in_arc,
            &test_in_rc,
            &test_iter_basic,
            &test_iter_drop,
            &test_iter_empty_slice,
            &test_iter_from_range,
            &test_iter_mut_all_elements,
            &test_kitchen_sink_layout,
            &test_large_allocation,
            &test_lifetimes_and_generics,
            &test_lying_iterator_long,
            &test_lying_iterator_short,
            &test_mixed_borrow,
            &test_mixed_empty_and_full,
            &test_multiple_simultaneous_views,
            &test_no_slices_def,
            &test_no_slices_iter,
            &test_only_one_slice_def,
            &test_only_one_slice_iter,
            &test_panic_mid_iter_no_leak,
            &test_rc_refcell,
            &test_send_sync,
            &test_slice_independence,
            &test_sliceborrow_coercions,
            &test_two_aligned_slices,
            &test_view_mut_then_view,
            &test_zst,
        ],
    )
}
