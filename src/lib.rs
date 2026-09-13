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
//! assert_eq!(p.payload(), &[10, 20, 30]);
//! assert_eq!(p.tags(),    &[100, 101, 102, 103]);
//!
//! // Fill with a repeated value — like vec![val; len]
//! let q = Packet::new_box_def(1, (0_u8, 8), (0_u32, 4));
//! assert_eq!(q.payload(), &[0_u8; 8]);
//! ```
//!
//! ## What the macro generates
//!
//! For each `#[slice] foo: T` field:
//! - `fn foo(&self) -> &[T]`
//! - `fn foo_mut(&mut self) -> &mut [T]`
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

pub use slice_struct_macro::slice_struct;

#[cfg(test)]
mod tests {
    use super::*;

    #[slice_struct]
    struct S<T: Clone> {
        a: u32,
        #[slice]
        b: T,
        #[slice]
        c: u32,
    }

    // ── new_box_iter ─────────────────────────────────────────────────────────

    #[test]
    fn test_iter_basic() {
        let mut s = S::<i32>::new_box_iter(42, [1, 2, 3].into_iter(), [4, 5, 6, 7].into_iter());

        assert_eq!(s.a, 42);
        assert_eq!(s.b(), &[1, 2, 3]);
        assert_eq!(s.c(), &[4, 5, 6, 7]);

        s.b_mut()[0] = 10;
        assert_eq!(s.b(), &[10, 2, 3]);
    }

    #[test]
    fn test_iter_from_range() {
        // Any ExactSizeIterator works — no pre-existing slice needed.
        // (Range<u32> impls ExactSizeIterator, RangeInclusive<u32> does not.)
        let s = S::<i32>::new_box_iter(0, (0..5).map(|x| x * x), 100_u32..104);

        assert_eq!(s.b(), &[0, 1, 4, 9, 16]);
        assert_eq!(s.c(), &[100, 101, 102, 103]);
    }

    #[test]
    fn test_iter_empty_slice() {
        let s = S::<i32>::new_box_iter(99, [].into_iter(), [].into_iter());
        assert_eq!(s.a, 99);
        assert_eq!(s.b(), &[]);
        assert_eq!(s.c(), &[]);
    }

    // ── new_box_def ──────────────────────────────────────────────────────────

    #[test]
    fn test_def_basic() {
        let s = S::<i32>::new_box_def(1, (0, 4), (7, 3));

        assert_eq!(s.a, 1);
        assert_eq!(s.b(), &[0, 0, 0, 0]);
        assert_eq!(s.c(), &[7, 7, 7]);
    }

    #[test]
    fn test_def_zero_len() {
        let s = S::<i32>::new_box_def(5, (99, 0), (42, 0));
        assert_eq!(s.b(), &[]);
        assert_eq!(s.c(), &[]);
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
        assert_eq!(s.b(), &["hello", "world"]);
    }

    #[test]
    fn test_def_drop() {
        let s = SStr::new_box_def("first".to_string(), ("hello".to_string(), 3));
        assert_eq!(s.b(), &["hello", "hello", "hello"]);
    }
}
