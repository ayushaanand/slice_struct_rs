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
//! let p = Packet::init_iter(42, [10, 20, 30].into_iter(), 100..104).in_box();
//! assert_eq!(*p.view().id, 42);
//! assert_eq!(p.view().payload, &[10, 20, 30]);
//! assert_eq!(p.view().tags,    &[100, 101, 102, 103]);
//! 
//! // Or zero-initialize them:
//! let q = Packet::init_def(99, (0, 8), (0, 0)).in_box();
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
//! let mut p = Packet::init_iter(1, [10, 20, 30].into_iter(), 100..104).in_box();
//! let mut v = p.as_mut().view_mut();
//! v.payload[0] = 99; // direct array access!
//! *v.id = 42;
//!
//! let v = p.view();
//! assert_eq!(*v.id, 42);
//! assert_eq!(v.payload, &[99, 20, 30])
//! ```
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

mod drop_guard;
mod handle;
mod init;
mod wrappers;

#[doc(hidden)]
pub use drop_guard::__DropGuard;
pub use handle::{SliceBorrow, SliceHandle};
pub use init::{OwnedDst, SliceBuilder, SliceInit};
pub use wrappers::{DstMutex, DstRefCell, WithMutex, WithRefCell};
