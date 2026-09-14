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
//! let p = Packet::new_box_iter(42, [10, 20, 30].into_iter(), 100..104);
//! assert_eq!(*p.view().id, 42);
//! assert_eq!(p.view().payload, &[10, 20, 30]);
//! assert_eq!(p.view().tags,    &[100, 101, 102, 103]);
//! 
//! // Or zero-initialize them:
//! let q = Packet::new_box_def(99, (0, 8), (0, 0));
//! assert_eq!(q.view().payload, &[0_u8; 8]);
//! ```
//!
//! ## The View API
//!
//! To ensure memory safety and perfectly handle the complex borrow-checking required 
//! for inline slices, the macro encapsulates the struct's layout.
//!
//! You interact with your data exclusively through two macro-generated structs:
//! - `{Struct}View` — obtained via `.view()` or `.as_ref().view()`, containing immutable `&'a` references to all fields.
//! - `{Struct}ViewMut` — obtained via `.as_mut().view_mut()`, providing `&'a mut` references to plain fields, and a special guard for slice fields that acts as a `&mut [T]`.
//!
//! ```rust
//! # use slice_struct::slice_struct;
//! # #[slice_struct]
//! # pub struct Packet {
//! #     pub id: u32,
//! #     #[slice] pub payload: u8,
//! #     #[slice] pub tags: u32,
//! # }
//! let mut p = Packet::new_box_iter(1, [10, 20, 30].into_iter(), 100..104);
//! let mut v = p.as_mut().view_mut();
//! v.payload[0] = 99; // direct array access!
//! *v.id = 42;
//!
//! let v = p.view();
//! assert_eq!(*v.id, 42);
//! assert_eq!(v.payload, &[99, 20, 30])
//! ```
//!
//! For the struct as a whole, two constructors are generated. For a struct named `Packet`, 
//! they will look roughly like this:
//!
//! ```rust,ignore
//! impl Packet {
//!     /// Construct a Box<Packet> by draining one ExactSizeIterator per slice field.
//!     pub fn new_box_iter(
//!         // .. sized field args ..
//!         // .. slice field iterators ..
//!     ) -> Pin<Box<Packet>>
//!
//!     /// Construct a Box<Packet> from (value, length) pairs (requires Clone).
//!     pub fn new_box_def(
//!         // .. sized field args ..
//!         // .. slice field (T, usize) pairs ..
//!     ) -> Pin<Box<Packet>>
//! }
//! ```
//!
//! Full documentation for these constructors will automatically appear on your generated 
//! struct when you run `cargo doc` in your own crate.
//!
//! ## Limitations
//!
//! - Named fields only (`struct Foo { field: Type }`).
//! - `#[slice]` fields must follow all plain fields in source order.
//! - The struct is `!Sized` and `!Unpin`; it must live behind a pinned pointer (`Pin<Box<Self>>`).
//!


#[cfg(test)]
extern crate self as slice_struct;

pub use slice_struct_macro::slice_struct;

use core::marker::{PhantomData, PhantomPinned};
use core::pin::Pin;
use core::ptr::NonNull;

/// A RAII guard that drops `written` already-initialized elements of a raw
/// slice pointer if the guarded scope panics before `forget()` is called.
///
/// Used internally by the `new_box_iter` constructor to prevent memory leaks
/// when an iterator panics mid-write.
#[doc(hidden)]
pub struct __DropGuard<T> {
    ptr: *mut T,
    written: *mut usize,
    _marker: PhantomData<T>,
}

impl<T> __DropGuard<T> {
    #[inline]
    pub fn new(ptr: *mut T, written: *mut usize) -> Self {
        Self { ptr, written, _marker: PhantomData }
    }
}

impl<T> Drop for __DropGuard<T> {
    fn drop(&mut self) {
        // SAFETY: `ptr..ptr+written` were all initialized before the panic.
        unsafe {
            let n = *self.written;
            core::ptr::drop_in_place(core::ptr::slice_from_raw_parts_mut(self.ptr, n));
        }
    }
}

unsafe impl<T: Send> Send for __DropGuard<T> {}
unsafe impl<T: Sync> Sync for __DropGuard<T> {}

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
/// iterate, or reborrow as `&[T]` or `&mut [T]` freely — without calling
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

