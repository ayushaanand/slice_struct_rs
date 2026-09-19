#![allow(unsafe_op_in_unsafe_fn)]
use crate::init::{OwnedDst, SliceInit};
use core::pin::Pin;
use core::ptr::NonNull;
use std::alloc::Layout;
use std::cell::{Ref, RefCell, RefMut, UnsafeCell};
use std::sync::{Mutex, MutexGuard};

/// A custom DST-compatible Mutex that locks the entire underlying `slice_struct`.
///
/// Because a `slice_struct` is a Dynamically Sized Type (DST) whose size isn't
/// known at compile-time, it cannot be safely wrapped inside a standard `std::sync::Mutex<T>`.
/// This wrapper guarantees the same thread-safety guarantees, allowing the struct to be
/// mutated concurrently.
///
/// ```rust
/// # use slice_struct::slice_struct;
/// # #[slice_struct]
/// # pub struct Packet { pub id: u32, #[slice] pub payload: [u8] }
/// let arc_mutex = Packet::init_def(1, (0, 10))
///     .with_mutex()
///     .in_arc();
///
/// let mut guard = arc_mutex.lock();
/// guard.as_mut().view_mut().payload[0] = 99;
/// ```
#[repr(C)]
pub struct DstMutex<T: ?Sized> {
    lock: Mutex<()>,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for DstMutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for DstMutex<T> {}

pub struct DstMutexGuard<'a, T: ?Sized> {
    _guard: MutexGuard<'a, ()>,
    data: Pin<&'a mut T>,
}

impl<'a, T: ?Sized> DstMutexGuard<'a, T> {
    pub fn as_mut(&mut self) -> Pin<&mut T> {
        self.data.as_mut()
    }
}

impl<'a, T: ?Sized> core::ops::Deref for DstMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T: ?Sized> DstMutex<T> {
    /// Acquires the lock, blocking the current thread until it is able to do so.
    /// Returns a guard that grants `Pin<&mut T>` access to the inner struct.
    pub fn lock(&self) -> DstMutexGuard<'_, T> {
        let guard = self.lock.lock().unwrap();
        let data = unsafe { Pin::new_unchecked(&mut *self.data.get()) };
        DstMutexGuard {
            _guard: guard,
            data,
        }
    }
}

pub struct WithMutex<I>(pub I);

unsafe impl<T: ?Sized, I: SliceInit<T>> SliceInit<DstMutex<T>> for WithMutex<I> {
    fn layout(&self) -> Layout {
        let (layout, _) = Layout::new::<Mutex<()>>().extend(self.0.layout()).unwrap();
        layout.pad_to_align()
    }

    fn make_fat_ptr(&self, base: *mut u8) -> *mut DstMutex<T> {
        self.0.make_fat_ptr(base) as *mut DstMutex<T>
    }

    unsafe fn write_data(self, ptr: *mut u8) -> OwnedDst<DstMutex<T>> {
        let layout = self.layout();
        let (_, offset) = Layout::new::<Mutex<()>>().extend(self.0.layout()).unwrap();

        std::ptr::write(ptr as *mut Mutex<()>, Mutex::new(()));

        let data_ptr = ptr.add(offset);
        let fat_ptr = self.make_fat_ptr(ptr);
        let owned_inner = unsafe { self.0.write_data(data_ptr) };
        std::mem::forget(owned_inner);

        OwnedDst {
            ptr: unsafe { NonNull::new_unchecked(fat_ptr) },
            layout,
        }
    }

    unsafe fn fixup(ptr: *mut DstMutex<T>) {
        let data_ptr = core::ptr::addr_of_mut!((*ptr).data) as *mut T;
        I::fixup(data_ptr);
    }
}

/// A custom DST-compatible RefCell that dynamically checks borrows for the entire underlying `slice_struct`.
///
/// Because a `slice_struct` is a Dynamically Sized Type (DST) whose size isn't
/// known at compile-time, it cannot be safely wrapped inside a standard `std::cell::RefCell<T>`.
/// This wrapper guarantees the same borrow-checking mechanics for dynamic interior mutability.
///
/// ```rust
/// # use slice_struct::slice_struct;
/// # #[slice_struct]
/// # pub struct Packet { pub id: u32, #[slice] pub payload: [u8] }
/// let rc_refcell = Packet::init_def(1, (0, 10))
///     .with_refcell()
///     .in_rc();
///
/// let mut guard = rc_refcell.borrow_mut();
/// guard.as_mut().view_mut().payload[0] = 99;
/// ```
#[repr(C)]
pub struct DstRefCell<T: ?Sized> {
    lock: RefCell<()>,
    data: UnsafeCell<T>,
}

pub struct DstRefMut<'a, T: ?Sized> {
    _guard: RefMut<'a, ()>,
    data: Pin<&'a mut T>,
}

impl<'a, T: ?Sized> DstRefMut<'a, T> {
    pub fn as_mut(&mut self) -> Pin<&mut T> {
        self.data.as_mut()
    }
}

impl<'a, T: ?Sized> core::ops::Deref for DstRefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T: ?Sized> DstRefCell<T> {
    /// Immutably borrows the wrapped value, panicking if the value is currently mutably borrowed.
    pub fn borrow(&self) -> Ref<'_, T> {
        Ref::map(self.lock.borrow(), |_| unsafe { &*self.data.get() })
    }

    /// Mutably borrows the wrapped value, panicking if the value is currently borrowed.
    /// Returns a guard that grants `Pin<&mut T>` access to the inner struct.
    pub fn borrow_mut(&self) -> DstRefMut<'_, T> {
        let guard = self.lock.borrow_mut();
        let data = unsafe { Pin::new_unchecked(&mut *self.data.get()) };
        DstRefMut {
            _guard: guard,
            data,
        }
    }
}

pub struct WithRefCell<I>(pub I);

unsafe impl<T: ?Sized, I: SliceInit<T>> SliceInit<DstRefCell<T>> for WithRefCell<I> {
    fn layout(&self) -> Layout {
        let (layout, _) = Layout::new::<RefCell<()>>()
            .extend(self.0.layout())
            .unwrap();
        layout.pad_to_align()
    }

    fn make_fat_ptr(&self, base: *mut u8) -> *mut DstRefCell<T> {
        self.0.make_fat_ptr(base) as *mut DstRefCell<T>
    }

    unsafe fn write_data(self, ptr: *mut u8) -> OwnedDst<DstRefCell<T>> {
        let layout = self.layout();
        let (_, offset) = Layout::new::<RefCell<()>>()
            .extend(self.0.layout())
            .unwrap();

        std::ptr::write(ptr as *mut RefCell<()>, RefCell::new(()));

        let data_ptr = ptr.add(offset);
        let fat_ptr = self.make_fat_ptr(ptr);
        let owned_inner = unsafe { self.0.write_data(data_ptr) };
        std::mem::forget(owned_inner);

        OwnedDst {
            ptr: unsafe { NonNull::new_unchecked(fat_ptr) },
            layout,
        }
    }

    unsafe fn fixup(ptr: *mut DstRefCell<T>) {
        let data_ptr = core::ptr::addr_of_mut!((*ptr).data) as *mut T;
        I::fixup(data_ptr);
    }
}
