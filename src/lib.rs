#![allow(clippy::missing_safety_doc)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
//! # slice_struct
//!
//! A Rust procedural macro for packing **multiple variable-length slices** and **complex stateful wrappers** into a single contiguous heap allocation.
//!
//! Normally, putting multiple dynamic arrays in a struct requires multiple `Vec`s, which spreads your data across multiple separate heap allocations. `slice_struct` packs everything inline into one contiguous block of memory to guarantee data locality and reduce memory overhead.
//!
//! ## Quick Start
//!
//! Add the dependency to your `Cargo.toml`. To use advanced features like nested arenas or binary parsing, enable the corresponding features:
//!
//! ```toml
//! [dependencies]
//! slice_struct = { version = "0.4.0", features = ["arena", "zerocopy"] }
//! ```
//!
//! Mark your dynamic fields with `#[slice]`.
//!
//! ```rust
//! use slice_struct::slice_struct;
//!
//! #[slice_struct]
//! pub struct Packet {
//!     pub id: u32,
//!     #[slice] pub payload: [u8],
//! }
//!
//! // 1. The macro generates `init_def`.
//! // Arguments strictly follow the struct fields: (id_value, (payload_default_val, payload_len))
//! let mut packet = Packet::init_def(42, (0, 10)).in_box();
//!
//! // 2. The macro generates `.view()` and `.view_mut()` for safe data access
//! let mut v = packet.as_mut().view_mut();
//! *v.id = 1;
//! v.payload[0] = 255;
//! ```
//!
//! ## Guides
//!
//! Because `slice_struct` changes how your structs are allocated, the documentation split into dedicated guides focused on how to use the generated APIs.
//!
//! - **[Initialization & Allocation](guide::initialization)**: Deep dive into `init_def`, `init_iter`, and composable allocations via `SliceBuilder` (`.in_box()`, `.in_arc()`, `.with_mutex()`).
//! - **[Implementing Methods](guide::methods)**: Learn how to encapsulate your logic by writing `impl` blocks on your generated structs, and how to use disjoint mutable borrowing.
//! - **[Shared Layouts & Unpin](guide::shared_layouts)**: Optimize memory for thousands of identical structs (ECS / Node graphs) using `#[slice_struct(shared_layout)]`, which natively unlocks `Unpin` standard methods.
//! - **[Nested Arenas](guide::nested_arenas)**: Embed dynamically sized arrays *inside* other dynamic arrays seamlessly using `ArenaSlice<T>`. *(Requires `arena` feature)*
//! - **[Zerocopy Parsing](guide::zerocopy)**: Safely cast binary byte buffers directly into viewable structs. *(Requires `zerocopy` feature)*
//!
//! ---
//!
//! ## Limitations
//!
//! * **Named fields only:** Tuple and unit structs are not supported.
//! * **Ordering:** `#[slice]` fields must follow plain fields in the struct definition.
//!
//! ## License
//!
//! MIT OR Apache-2.0
extern crate self as slice_struct;

pub use slice_struct_macro::slice_struct;

#[cfg(feature = "arena")]
mod arena;
mod drop_guard;
mod handle;
mod init;
mod inline;
mod wrappers;


pub mod __private {
    
#[cfg(feature = "arena")]
    pub use crate::arena::{ArenaDescriptor, ArenaElement};
    pub use crate::drop_guard::__DropGuard;
    pub use crate::handle::{
        __SyncUnsafeCell, AbsoluteMode, AddressingMode, RelativeMode, SliceHandle,
    };
    pub use crate::init::{OwnedDst, SliceInit};
    pub use crate::inline::InlineSlice;
    pub use crate::wrappers::{WithMutex, WithRefCell};

    #[cfg(feature = "zerocopy")]
    pub use ::zerocopy;
}

#[cfg(feature = "arena")]
pub use arena::{ArenaDescriptor, ArenaElement, ArenaSlice, ArenaSliceView, ArenaSliceViewMut};
pub use handle::SliceBorrow;
pub use init::SliceBuilder;
pub use inline::{SliceMutexGuard, SliceRefGuard};
pub use wrappers::{DstMutex, DstRefCell};

use core::pin::Pin;

pub trait AsView {
    type View<'a>
    where
        Self: 'a;
    fn as_view(&self) -> Self::View<'_>;
}

pub trait AsViewMut {
    type ViewMut<'a>
    where
        Self: 'a;
    fn as_view_mut(self: Pin<&mut Self>) -> Self::ViewMut<'_>;
}

pub trait UnpinViewMutExt: AsViewMut + Unpin {
    fn view_mut(&mut self) -> Self::ViewMut<'_>;
}

impl<T: AsViewMut + Unpin> UnpinViewMutExt for T {
    #[inline(always)]
    fn view_mut(&mut self) -> Self::ViewMut<'_> {
        Pin::new(self).as_view_mut()
    }
}

mod shared;
pub use shared::*;



/// Detailed user guides detailing API usage and layout capabilities.
pub mod guide {
    #[doc = include_str!("../docs/guide/01_initialization.md")]
    pub mod initialization {}

    #[doc = include_str!("../docs/guide/02_methods.md")]
    pub mod methods {}

    #[doc = include_str!("../docs/guide/03_shared_layouts.md")]
    pub mod shared_layouts {}

    #[doc = include_str!("../docs/guide/04_nested_arenas.md")]
    pub mod nested_arenas {}

    #[doc = include_str!("../docs/guide/05_zerocopy.md")]
    pub mod zerocopy {}
}