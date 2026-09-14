//! # slice_struct
//!
//! A macro for structs with **multiple inline, variable-length slices** packed
//! into a single heap allocation — without any pointer indirection.
//!
//! ## Quick start
//!
//! ```rust
//! use slice_struct::slice_struct;
//!
//! #[slice_struct]
//! pub struct Packet {
//!     pub id: u32,
//!     #[slice] pub payload: u8,
//!     #[slice] pub tags:    u32,
//! }
//!
//! // Fill from iterators — no Clone required
//! let p = Packet::new_box_iter(
//!     42,
//!     [10_u8, 20, 30].into_iter(),
//!     100_u32..104,
//! );
//! assert_eq!(&*p.payload, &[10, 20, 30]);
//! assert_eq!(&*p.tags,    &[100, 101, 102, 103]);
//!
//! // Fill with a repeated value — like vec![val; len]
//! let q = Packet::new_box_def(1, (0_u8, 8), (0_u32, 4));
//! assert_eq!(&*q.payload, &[0_u8; 8]);
//! ```
//!
//! ## What the macro generates
//!
//! For each `#[slice] foo: T` field, the macro generates an actual field:
//! - `foo: SliceHandle<T>`
//!
//! This field implements `Deref<Target = [T]>`, allowing `&*p.foo`.
//! For safe disjoint mutable borrowing, a `project()` method is generated:
//!
//! ```rust,ignore
//! let mut proj = p.as_mut().project();
//! proj.payload.as_mut_slice()[0] = 99;
//! proj.tags.as_mut_slice()[0] = 99;
//! ```
//!
//! For the struct as a whole:
//! - [`new_box_iter`] — one `impl ExactSizeIterator` per slice field; no `Clone` needed
//! - [`new_box_def`]  — one `(T, usize)` pair per slice field; requires `T: Clone`
//!
//! ## Limitations
//!
//! - Named fields only (`struct Foo { field: Type }`).
//! - `#[slice]` fields must follow all plain fields in source order.
//! - The struct is `?Sized`; it must live behind a pointer such as `Box`.
//!
//! [`new_box_iter`]: #
//! [`new_box_def`]: #

#[cfg(test)]
extern crate self as slice_struct;

pub use slice_struct_macro::slice_struct;

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

/// A temporary borrow of an inline slice field, produced by [`.project()`].
///
/// Implements both [`Deref`] and [`DerefMut`] to `[T]`, so you can index,
/// iterate, or reborrow as `&[T]` or `&mut [T]` freely — without calling
/// any extra method.
///
/// The key property: two `SliceBorrow`s from the same `.project()` call are
/// **disjoint** — one can be borrowed immutably while another is borrowed
/// mutably, at the same time.
///
/// [`.project()`]: the generated `project` method on the struct
pub struct SliceBorrow<'a, T> {
    ptr: NonNull<T>,
    len: usize,
    _marker: PhantomData<&'a mut [T]>,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[slice_struct]
    struct Str<T: Clone> {
        a: u32,
        #[slice]
        b: T,
        #[slice]
        c: u32,
    }

    // ── new_box_iter ─────────────────────────────────────────────────────────

    #[test]
    fn test_iter_basic() {
        let mut s = Str::<i32>::new_box_iter(42, [1, 2, 3].into_iter(), [4, 5, 6, 7].into_iter());

        assert_eq!(s.a, 42);
        assert_eq!(&*s.b, &[1, 2, 3]);
        assert_eq!(&*s.c, &[4, 5, 6, 7]);

        // project() is the guard — get it once, borrow fields independently
        let mut proj = s.as_mut().project();
        // direct indexing, no .as_mut_slice() needed
        proj.b[0] = 10;
        proj.c[1] = 99;

        assert_eq!(&*s.b, &[10, 2, 3]);
        assert_eq!(&*s.c, &[4, 99, 6, 7]);
    }

    #[test]
    fn test_mixed_borrow() {
        // The whole point: borrow b immutably AND c mutably at the same time
        let mut s = Str::<i32>::new_box_iter(0, [10_i32, 20, 30].into_iter(), [1_u32, 2, 3].into_iter());
        let mut proj = s.as_mut().project();

        let b_ref: &[i32] = &proj.b;              // immutable borrow of b
        proj.c[0] = b_ref[2] as u32 * 10;         // mutable borrow of c — simultaneous!

        assert_eq!(&*s.b, &[10, 20, 30]);         // b is unchanged
        assert_eq!(&*s.c, &[300, 2, 3]);          // c[0] got b[2]*10
    }

    #[test]
    fn test_iter_from_range() {
        let s = Str::<i32>::new_box_iter(0, (0..5).map(|x| x * x), 100_u32..104);

        assert_eq!(&*s.b, &[0, 1, 4, 9, 16]);
        assert_eq!(&*s.c, &[100, 101, 102, 103]);
    }

    #[test]
    fn test_iter_empty_slice() {
        let s = Str::<i32>::new_box_iter(99, [].into_iter(), [].into_iter());
        assert_eq!(s.a, 99);
        assert_eq!(&*s.b, &[]);
        assert_eq!(&*s.c, &[]);
    }

    // ── new_box_def ──────────────────────────────────────────────────────────

    #[test]
    fn test_def_basic() {
        let s = Str::<i32>::new_box_def(1, (0, 4), (7, 3));

        assert_eq!(s.a, 1);
        assert_eq!(&*s.b, &[0, 0, 0, 0]);
        assert_eq!(&*s.c, &[7, 7, 7]);
    }

    #[test]
    fn test_def_zero_len() {
        let s = Str::<i32>::new_box_def(5, (99, 0), (42, 0));
        assert_eq!(&*s.b, &[]);
        assert_eq!(&*s.c, &[]);
    }

    // ── Drop correctness (non-Copy element types) ─────────────────────────────

    #[slice_struct]
    struct SStr {
        a: String,
        #[slice]
        b: String,
    }

    #[test]
    fn test_iter_drop() {
        let words = ["hello".to_string(), "world".to_string()];
        let s = SStr::new_box_iter("first".to_string(), words.into_iter());
        assert_eq!(s.a, "first");
        assert_eq!(&*s.b, &["hello", "world"]);
    }

    #[test]
    fn test_def_drop() {
        let s = SStr::new_box_def("first".to_string(), ("hello".to_string(), 3));
        assert_eq!(&*s.b, &["hello", "hello", "hello"]);
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;
    extern crate self as slice_struct;

    #[slice_struct]
    struct NoSlices {
        pub a: u32,
        pub b: bool,
    }

    #[test]
    fn test_no_slices_iter() {
        let mut s = NoSlices::new_box_iter(42, true);
        assert_eq!(s.a, 42);
        assert_eq!(s.b, true);
        
        let mut proj = s.as_mut().project();
        *proj.a = 99;
        assert_eq!(s.a, 99);
    }

    #[test]
    fn test_no_slices_def() {
        let s = NoSlices::new_box_def(42, true);
        assert_eq!(s.a, 42);
        assert_eq!(s.b, true);
    }

    #[slice_struct]
    struct OneSlice {
        #[slice] pub data: u8,
    }

    #[test]
    fn test_only_one_slice_iter() {
        let mut s = OneSlice::new_box_iter([10, 20].into_iter());
        assert_eq!(&*s.data, &[10, 20]);
        
        let mut proj = s.as_mut().project();
        proj.data[1] = 99;
        assert_eq!(&*s.data, &[10, 99]);
    }

    #[test]
    fn test_only_one_slice_def() {
        let s = OneSlice::new_box_def((5, 3));
        assert_eq!(&*s.data, &[5, 5, 5]);
    }
}
