//! # slice_struct
//!
//! A Rust procedural macro for packing **multiple variable-length slices** and **complex stateful wrappers** into a single contiguous heap allocation.
//!
//! Normally, putting multiple dynamic arrays in a struct requires multiple `Vec`s, which spreads your data across multiple separate heap allocations. `slice_struct` packs everything inline into one contiguous block of memory, offering zero-cost access and perfect memory safety.
//!
//! With version `0.3.0`, `slice_struct` now features:
//! - **Universal Inline Slice Projection**: Wrap your slices in `Mutex`, `RefCell`, or `str` effortlessly with safe, disjoint borrow checking.
//! - **Composable Allocation (SliceBuilder)**: Seamlessly instantiate structs into `Box`, `Arc`, `Rc`, or wrap them in DST locks directly without macro bloat.
//!
//! ## Usage
//!
//! Add the dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! slice_struct = "0.3.1"
//! ```
//!
//! ### 1. Defining and Reading
//!
//! Mark your dynamic fields with `#[slice]`. The macro securely encapsulates the internal layout to guarantee memory safety. Instead of manual heap management, you interact with your data safely through `.view()` and `.view_mut()`.
//!
//! ```rust
//! use slice_struct::slice_struct;
//! use std::sync::Mutex;
//!
//! #[slice_struct]
//! pub struct Packet {
//!     pub id: u32,
//!     #[slice] pub payload: [u8],            // Standard dynamic array
//!     #[slice] pub flags: Mutex<[u32]>,      // Stateful wrapper!
//!     #[slice] pub string_data: str,         // UTF-8 validated slice
//! }
//!
//! // Create a new packet, populating the slices from iterators (zero-clone)
//! let p = Packet::init_iter(
//!     42,                                // id
//!     [10_u8, 20, 30].into_iter(),       // payload
//!     [100_u32, 200_u32].into_iter(),    // flags (Mutex)
//!     "hello".bytes(),                   // string_data (str)
//! ).in_box();
//!
//! // Read fields via `.view()`
//! let mut v = p.view();
//! assert_eq!(*v.id, 42);
//! assert_eq!(v.payload, &[10, 20, 30]);
//! assert_eq!(v.string_data, "hello");
//!
//! // Mutex fields provide safe inner interior mutability from immutable views!
//! v.flags[0] = 999;
//! assert_eq!(&*v.flags, &[999, 200]);
//! ```
//!
//! ### 2. Mutating (Disjoint Borrowing)
//!
//! To modify multiple distinct slices simultaneously, use `.as_mut().view_mut()`. This provides safe, disjoint mutable borrowing, meaning you can safely mutate one slice while interacting with another.
//!
//! ```rust
//! # use slice_struct::slice_struct;
//! # use std::sync::Mutex;
//! # #[slice_struct]
//! # pub struct Packet {
//! #     pub id: u32,
//! #     #[slice] pub payload: [u8],            
//! #     #[slice] pub flags: Mutex<[u32]>,      
//! #     #[slice] pub string_data: str,         
//! # }
//! // Create a packet using default values and lengths (like vec![val; len])
//! let mut p = Packet::init_def(1, (0_u8, 8), (0_u32, 4), (0_u8, 5)).in_box();
//!
//! let mut v = p.as_mut().view_mut();
//!
//! // Mutate data safely!
//! v.payload[0] = 100;
//! v.string_data.make_ascii_uppercase();
//!
//! // Rust guarantees safe disjoint borrowing natively!
//! let p_ref: &mut [u8] = v.payload;
//! let s_ref: &mut str = v.string_data;
//!
//! // Wait, wait! flags are accessed via interior mutability!
//!
//! v.flags[0] = 999;
//! ```
//!
//! *(Note: `.as_mut()` is required because the macro returns a pinned pointer `Pin<Box<Self>>`, and standard Rust rules require you to explicitly borrow pinned boxes).*
//!
//! ### 3. Composable Allocation (`SliceBuilder`)
//!
//! You are not restricted to just `Box`. The `SliceBuilder` pattern allows you to route your allocations to `Arc` or `Rc`, and even lock the *entire struct* safely behind a DST `Mutex`.
//!
//! ```rust
//! # use slice_struct::slice_struct;
//! # use std::sync::Mutex;
//! # #[slice_struct]
//! # pub struct Packet {
//! #     pub id: u32,
//! #     #[slice] pub payload: [u8],            
//! #     #[slice] pub flags: Mutex<[u32]>,      
//! #     #[slice] pub string_data: str,         
//! # }
//! // Allocate directly into an Arc
//! let arc = Packet::init_def(1, (0, 8), (0, 4), (0, 5)).in_arc();
//! let cloned = arc.clone();
//!
//! // Lock the ENTIRE DST dynamically behind a Mutex inside an Arc
//! let locked_arc = Packet::init_def(1, (0, 8), (0, 4), (0, 5))
//!     .with_mutex()
//!     .in_arc();
//!
//! // Multithreaded mutation!
//! std::thread::spawn(move || {
//!     let mut guard = locked_arc.lock();
//!     guard.as_mut().view_mut().payload[0] = 42;
//! });
//! ```
//!
//! ## Features
//!
//! The `slice_struct` crate provides implementations for `InlineSlice` natively:
//! * **`[T]`**: Raw contiguous dynamic arrays.
//! * **`str`**: Dynamic string slices. Automatically validates UTF-8 upon initialization, guaranteeing zero-cost `&str` access during runtime.
//! * **`Mutex<[T]>`**: Interior mutability over slices. Synchronizes parallel mutations over individual slices without needing to lock the entire struct.
//! * **`RefCell<[T]>`**: Interior mutability for single-threaded usage.
//!
//! ## Limitations
//!
//! * **Named fields only:** Tuple and unit structs are not supported.
//! * **Ordering:** `#[slice]` fields must follow plain fields in the struct definition.
//! * **Pinning:** The resulting struct is `!Sized` and `!Unpin`, meaning it must live exclusively behind a pointer (like `Pin<Box<Self>>`) and cannot be moved in memory.
//! * **Derives:** Standard `#[derive(Clone)]` is not safe. If cloning is needed, implement it manually by allocating a new pointer.
//!
//! ## License
//!
//! MIT OR Apache-2.0
#[cfg(test)]
extern crate self as slice_struct;

pub use slice_struct_macro::slice_struct;

#[cfg(feature = "arena")]
mod arena;
mod drop_guard;
mod handle;
mod init;
mod inline;
mod wrappers;

#[doc(hidden)]
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
