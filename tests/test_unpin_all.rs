#![allow(dead_code)]
use slice_struct::slice_struct;

#[slice_struct(unpin)]
struct Str<T: Clone> {
    a: u32,
    #[slice]
    b: [T],
    #[slice]
    c: [u32],
}

// ── init_iter ─────────────────────────────────────────────────────────

#[test]
fn test_iter_basic() {
    let mut s = Str::init_iter(42, [1i32, 2, 3].into_iter(), [4, 5, 6, 7].into_iter()).in_box();

    assert_eq!(*s.view().a, 42);
    assert_eq!(s.view().b, &[1i32, 2, 3]);
    assert_eq!(s.view().c, &[4u32, 5, 6, 7]);

    let v = s.as_mut().view_mut();
    v.b[0] = 10;
    v.c[1] = 99;

    assert_eq!(s.view().b, &[10, 2, 3]);
    assert_eq!(s.view().c, &[4, 99, 6, 7]);
}

#[test]
fn test_mixed_borrow() {
    let mut s = Str::init_iter(0, [10_i32, 20, 30].into_iter(), [1_u32, 2, 3].into_iter()).in_box();
    let v = s.as_mut().view_mut();

    let b_ref: &[i32] = &v.b;
    v.c[0] = b_ref[2] as u32 * 10;

    assert_eq!(s.view().b, &[10, 20, 30]);
    assert_eq!(s.view().c, &[300, 2, 3]);
}

#[test]
fn test_iter_from_range() {
    let s = Str::<i32>::init_iter(0, (0..5).map(|x| x * x), 100_u32..104).in_box();
    assert_eq!(s.view().b, &[0, 1, 4, 9, 16]);
    assert_eq!(s.view().c, &[100, 101, 102, 103]);
}

#[test]
fn test_iter_empty_slice() {
    let s = Str::<i32>::init_iter(99, [].into_iter(), [].into_iter()).in_box();
    assert_eq!(*s.view().a, 99);
    assert_eq!(s.view().b, &[]);
    assert_eq!(s.view().c, &[]);
}

// ── init_def ──────────────────────────────────────────────────────────

#[test]
fn test_def_basic() {
    let s = Str::<i32>::init_def(1, (0, 4), (7, 3)).in_box();
    assert_eq!(*s.view().a, 1);
    assert_eq!(s.view().b, &[0, 0, 0, 0]);
    assert_eq!(s.view().c, &[7, 7, 7]);
}

#[test]
fn test_def_zero_len() {
    let s = Str::<i32>::init_def(5, (99, 0), (42, 0)).in_box();
    assert_eq!(s.view().b, &[]);
    assert_eq!(s.view().c, &[]);
}

// ── Drop correctness (non-Copy element types) ─────────────────────────────

#[slice_struct(unpin)]
struct SStr {
    a: String,
    #[slice]
    b: [String],
}

#[test]
fn test_iter_drop() {
    let words = ["hello".to_string(), "world".to_string()];
    let s = SStr::init_iter("first".to_string(), words.into_iter()).in_box();
    assert_eq!(s.view().a, "first");
    assert_eq!(s.view().b, &["hello", "world"]);
}

#[test]
fn test_def_drop() {
    let s = SStr::init_def("first".to_string(), ("hello".to_string(), 3)).in_box();
    assert_eq!(s.view().b, &["hello", "hello", "hello"]);
}

#[slice_struct(unpin)]
struct NoSlices {
    pub a: u32,
    pub b: bool,
}

#[test]
fn test_no_slices_iter() {
    let mut s = NoSlices::init_iter(42, true).in_box();
    assert_eq!(*s.view().a, 42);
    assert_eq!(*s.view().b, true);

    let v = s.as_mut().view_mut();
    *v.a = 99;
    assert_eq!(*s.view().a, 99);
}

#[test]
fn test_no_slices_def() {
    let s = NoSlices::init_def(42, true).in_box();
    assert_eq!(*s.view().a, 42);
    assert_eq!(*s.view().b, true);
}

#[slice_struct(unpin)]
struct OneSlice {
    #[slice]
    pub data: [u8],
}

#[test]
fn test_only_one_slice_iter() {
    let mut s = OneSlice::init_iter([10, 20].into_iter()).in_box();
    assert_eq!(s.view().data, &[10, 20]);

    let v = s.as_mut().view_mut();
    v.data[1] = 99;
    assert_eq!(s.view().data, &[10, 99]);
}

#[test]
fn test_only_one_slice_def() {
    let s = OneSlice::init_def((5, 3)).in_box();
    assert_eq!(s.view().data, &[5, 5, 5]);
}

// ── New edge cases ──────────────────────────────────────────────────────────

#[slice_struct(unpin)]
struct MixedEmpty {
    #[slice]
    a: [u8],
    #[slice]
    b: [u16],
    #[slice]
    c: [u32],
}

#[test]
fn test_mixed_empty_and_full() {
    let s = MixedEmpty::init_def((1, 0), (2, 5), (3, 0)).in_box();
    assert_eq!(s.view().a, &[]);
    assert_eq!(s.view().b, &[2, 2, 2, 2, 2]);
    assert_eq!(s.view().c, &[]);

    let s2 =
        MixedEmpty::init_iter([1, 2].into_iter(), [].into_iter(), [3, 4, 5].into_iter()).in_box();
    assert_eq!(s2.view().a, &[1, 2]);
    assert_eq!(s2.view().b, &[]);
    assert_eq!(s2.view().c, &[3, 4, 5]);
}

#[slice_struct(unpin)]
struct ZstStruct {
    #[slice]
    zst: [()],
}

#[test]
fn test_zst() {
    let s = ZstStruct::init_def(((), 100)).in_box();
    assert_eq!(s.view().zst.len(), 100);
}

#[repr(align(64))]
#[derive(Clone, PartialEq, Debug)]
struct Aligned64(u8);

#[slice_struct(unpin)]
struct HighlyAligned {
    a: u8,
    #[slice]
    data: [Aligned64],
}

#[test]
fn test_highly_aligned() {
    let s = HighlyAligned::init_def(42, (Aligned64(99), 3)).in_box();
    assert_eq!(*s.view().a, 42);
    assert_eq!(
        s.view().data,
        &[Aligned64(99), Aligned64(99), Aligned64(99)]
    );

    // Check alignment
    let data_ptr = s.view().data.as_ptr() as usize;
    assert_eq!(data_ptr % 64, 0, "Slice data must be 64-byte aligned");
}

#[test]
fn test_send_sync() {
    // This will fail to compile if `SliceHandle` is not Send/Sync
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<std::pin::Pin<Box<Str<i32>>>>();
    assert_sync::<std::pin::Pin<Box<Str<i32>>>>();
}

// ── Drop Tracking ───────────────────────────────────────────────────────────

use std::sync::atomic::{AtomicUsize, Ordering};

static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
struct DropCounter(usize);
impl Drop for DropCounter {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

#[slice_struct(unpin)]
struct DropTest {
    #[slice]
    items: [DropCounter],
}

#[test]
fn test_exact_drop_count() {
    DROP_COUNT.store(0, Ordering::SeqCst);
    {
        let s = DropTest::init_def((DropCounter(1), 5)).in_box();
        assert_eq!(s.view().items.len(), 5);
        assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 0); // None dropped yet
    }
    // All 5 elements should be dropped when the Box drops
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 5);
}

// ── Lying Iterators ─────────────────────────────────────────────────────────

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

#[slice_struct(unpin)]
struct IterTest {
    #[slice]
    data: [u32],
}

#[test]
fn test_lying_iterator_long() {
    // Claims 5, but yields 10. Macro should only consume 5 and stop.
    let iter = BadIter {
        yields: 10,
        claims: 5,
    };
    let s = IterTest::init_iter(iter).in_box();
    assert_eq!(s.view().data.len(), 5);
}

#[test]
#[should_panic(expected = "yielded fewer elements")]
fn test_lying_iterator_short() {
    // Claims 10, but yields 5. Macro MUST panic to prevent uninitialized memory UB.
    let iter = BadIter {
        yields: 5,
        claims: 10,
    };
    let _s = IterTest::init_iter(iter).in_box();
}

// ── Lifetimes and Generics ──────────────────────────────────────────────────

#[slice_struct(unpin)]
struct WithLifetimes<'a, T: Clone> {
    prefix: &'a str,
    #[slice]
    data: [T],
}

#[test]
fn test_lifetimes_and_generics() {
    let local_str = String::from("hello");
    let s = WithLifetimes::init_def(local_str.as_str(), (42_u32, 3)).in_box();
    assert_eq!(*s.view().prefix, "hello");
    assert_eq!(s.view().data, &[42, 42, 42]);
}

// ── Complex Layout Padding ──────────────────────────────────────────────────

#[slice_struct(unpin)]
struct KitchenSink {
    a: u8,
    #[slice]
    b: [u64],
    #[slice]
    c: [u8],
    #[slice]
    d: [()],
    #[slice]
    e: [u32],
}

#[test]
fn test_kitchen_sink_layout() {
    // This violently stresses the layout allocator padding.
    // u8 -> u64 requires 7 bytes of padding.
    // u8 -> () requires 0 padding.
    // () -> u32 requires padding to reach 4-byte alignment.
    let s = KitchenSink::init_def(1, (2_u64, 3), (3_u8, 5), ((), 10), (5_u32, 2)).in_box();

    let v = s.view();
    assert_eq!(*v.a, 1);
    assert_eq!(v.b, &[2, 2, 2]);
    assert_eq!(v.c, &[3, 3, 3, 3, 3]);
    assert_eq!(v.d.len(), 10);
    assert_eq!(v.e, &[5, 5]);
}

// ── view() does not require unique ownership ──────────────────────────────

#[test]
fn test_multiple_simultaneous_views() {
    let s = Str::<i32>::init_iter(10, [1, 2, 3].into_iter(), [4, 5].into_iter()).in_box();

    // Two independent view() borrows can coexist — no reborrowing conflict.
    let v1 = s.view();
    let v2 = s.view();
    assert_eq!(v1.b, v2.b);
    assert_eq!(v1.c, v2.c);
}

// ── view_mut() releases borrow, allowing a subsequent view() ─────────────

#[test]
fn test_view_mut_then_view() {
    let mut s = Str::<i32>::init_iter(0, [1, 2, 3].into_iter(), [10].into_iter()).in_box();
    {
        let v = s.as_mut().view_mut();
        v.b[0] = 99;
    } // mutable borrow ends here
    assert_eq!(s.view().b[0], 99); // immutable borrow is now legal
}

// ── Slices are independent pointers: mutate one, read the other ──────────

#[test]
fn test_slice_independence() {
    let mut s = Str::<i32>::init_iter(0, [1, 2, 3].into_iter(), [10, 20, 30].into_iter()).in_box();
    let v = s.as_mut().view_mut();

    // Read c, write b — these are physically separate memory regions.
    let sum_c: u32 = v.c.iter().sum();
    v.b.iter_mut().for_each(|x| *x = sum_c as i32);

    drop(v);
    assert_eq!(s.view().b, &[60, 60, 60]);
    assert_eq!(s.view().c, &[10, 20, 30]); // c untouched
}

// ── Large allocations ────────────────────────────────────────────────────

#[slice_struct(unpin)]
struct BigBuf {
    tag: u64,
    #[slice]
    data: [u8],
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_large_allocation() {
    const N: usize = 1 << 20; // 1 MiB of u8
    let s = BigBuf::init_iter(0xdeadbeef, (0..N).map(|i| (i % 256) as u8)).in_box();

    assert_eq!(s.view().data.len(), N);
    assert_eq!(s.view().data[0], 0);
    assert_eq!(s.view().data[255], 255);
    assert_eq!(s.view().data[256], 0); // wraps back around
    assert_eq!(s.view().data[N - 1], ((N - 1) % 256) as u8);
}

// ── Arc<T> elements: ref-counted types in slices ──────────────────────────

use std::sync::Arc;

#[slice_struct(unpin)]
struct ArcSlice {
    #[slice]
    items: [Arc<u32>],
}

#[test]
fn test_arc_element_drop() {
    let shared = Arc::new(42_u32);
    assert_eq!(Arc::strong_count(&shared), 1);

    {
        let s = ArcSlice::init_iter([shared.clone(), shared.clone(), shared.clone()].into_iter())
            .in_box();
        assert_eq!(Arc::strong_count(&shared), 4); // 1 original + 3 in slice
        assert_eq!(*s.view().items[1], 42);
    }
    // Box dropped: all 3 Arcs cloned into the slice should be dropped.
    assert_eq!(Arc::strong_count(&shared), 1);
}

// ── Partial write rollback (panic mid-iter) ───────────────────────────────

struct PanicsAt {
    current: usize,
    panics_at: usize,
}
impl Iterator for PanicsAt {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.current == self.panics_at {
            panic!("Deliberate panic at element {}", self.current);
        }
        let v = self.current;
        self.current += 1;
        Some(format!("item_{}", v))
    }
}
impl ExactSizeIterator for PanicsAt {
    fn len(&self) -> usize {
        self.panics_at + 1
    }
}

#[slice_struct(unpin)]
struct StringSlice {
    #[slice]
    words: [String],
}

#[test]
fn test_panic_mid_iter_no_leak() {
    DROP_COUNT.store(0, Ordering::SeqCst);

    // Each String that gets written before the panic should be dropped
    // when the stack unwinds. We use a wrapper to count.
    #[derive(Clone)]
    struct Tracked(String);
    impl Drop for Tracked {
        fn drop(&mut self) {
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[slice_struct]
    struct TrackedSlice {
        #[slice]
        items: [Tracked],
    }

    struct TrackedPanicsAt {
        current: usize,
        panics_at: usize,
    }
    impl Iterator for TrackedPanicsAt {
        type Item = Tracked;
        fn next(&mut self) -> Option<Tracked> {
            if self.current == self.panics_at {
                panic!("deliberate");
            }
            self.current += 1;
            Some(Tracked(format!("item")))
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

    assert!(result.is_err(), "should have panicked");
    // The 3 successfully written items must have been dropped during unwind.
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 3);
}

// ── Pointer alignment for all slice fields ────────────────────────────────

#[repr(align(16))]
#[derive(Clone)]
struct A16(u8);
#[repr(align(32))]
#[derive(Clone)]
struct A32(u8);

#[slice_struct(unpin)]
struct TwoAligned {
    tag: u8,
    #[slice]
    a: [A16],
    #[slice]
    b: [A32],
}

#[test]
fn test_two_aligned_slices() {
    let s = TwoAligned::init_def(7, (A16(1), 4), (A32(2), 2)).in_box();
    let ptr_a = s.view().a.as_ptr() as usize;
    let ptr_b = s.view().b.as_ptr() as usize;
    assert_eq!(ptr_a % 16, 0, "A16 slice not 16-byte aligned");
    assert_eq!(ptr_b % 32, 0, "A32 slice not 32-byte aligned");
}

// ── Mutating all elements via view_mut ────────────────────────────────────

#[test]
fn test_iter_mut_all_elements() {
    let mut s = Str::<i32>::init_def(0, (1, 100), (2, 100)).in_box();
    {
        let v = s.as_mut().view_mut();
        v.b.iter_mut().enumerate().for_each(|(i, x)| *x = i as i32);
        v.c.iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x = (i * 2) as u32);
    }

    let v = s.view();
    assert_eq!(v.b[0], 0);
    assert_eq!(v.b[99], 99);
    assert_eq!(v.c[0], 0);
    assert_eq!(v.c[99], 198);
}

// ── Struct with only sized fields (no slices) still works end-to-end ──────

#[slice_struct(unpin)]
struct AllSized {
    pub x: i64,
    pub y: f64,
    pub z: bool,
}

#[test]
fn test_all_sized_fields_read_write() {
    let mut s = AllSized::init_def(1_i64, 3.14_f64, true).in_box();
    assert_eq!(*s.view().x, 1);
    assert!((s.view().y - 3.14).abs() < 1e-10);
    assert_eq!(*s.view().z, true);

    {
        let v = s.as_mut().view_mut();
        *v.x = -99;
        *v.y = 2.718;
        *v.z = false;
    }

    assert_eq!(*s.view().x, -99);
    assert!((s.view().y - 2.718).abs() < 1e-10);
    assert_eq!(*s.view().z, false);
}

// ── SliceBorrow deref: &[T] and &mut [T] coercions ───────────────────────

#[test]
fn test_sliceborrow_coercions() {
    fn takes_slice(s: &[i32]) -> i32 {
        s.iter().sum()
    }
    fn takes_mut_slice(s: &mut [i32]) {
        s.iter_mut().for_each(|x| *x *= 2);
    }

    let mut s = Str::<i32>::init_iter(0, [1, 2, 3, 4].into_iter(), [0].into_iter()).in_box();
    {
        let mut v = s.as_mut().view_mut();
        // SliceBorrow<i32> must coerce to &[i32] and &mut [i32]
        let sum = takes_slice(&v.b);
        assert_eq!(sum, 10);
        takes_mut_slice(&mut v.b);
    }
    assert_eq!(s.view().b, &[2, 4, 6, 8]);
}

// ── Tests for Arc, Rc, Mutex, RefCell Wrappers ───────────────────────────

#[slice_struct(unpin)]
struct WrapTest {
    pub counter: u32,
    #[slice]
    pub data: [u8],
}

#[test]
fn test_in_arc() {
    let arc = WrapTest::init_def(10, (5, 4)).in_arc();
    assert_eq!(*arc.view().counter, 10);
    assert_eq!(arc.view().data, &[5, 5, 5, 5]);
    let clone1 = arc.clone();

    assert_eq!(clone1.view().data.len(), 4);

    let raw_arc = unsafe { std::pin::Pin::into_inner_unchecked(arc) };
    assert_eq!(Arc::strong_count(&raw_arc), 2);
}

#[test]
fn test_in_rc() {
    let rc = WrapTest::init_def(99, (1, 2)).in_rc();
    let clone1 = rc.clone();

    assert_eq!(clone1.view().data, &[1, 1]);

    let raw_rc = unsafe { std::pin::Pin::into_inner_unchecked(rc) };
    assert_eq!(std::rc::Rc::strong_count(&raw_rc), 2);
}

#[test]
fn test_arc_mutex_concurrency() {
    use std::thread;
    let s = WrapTest::init_def(0, (0, 100)).with_mutex().in_arc();
    let mut handles = vec![];
    for _ in 0..10 {
        let s_clone = s.clone();
        handles.push(thread::spawn(move || {
            let mut guard = s_clone.lock();
            let view = guard.as_mut().view_mut();
            *view.counter += 1;
            for i in 0..100 {
                view.data[i] += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let guard = s.lock();
    let view = guard.view();
    assert_eq!(*view.counter, 10);
    for i in 0..100 {
        assert_eq!(view.data[i], 10);
    }
}

#[test]
fn test_rc_refcell() {
    let rc = WrapTest::init_def(0, (0, 10)).with_refcell().in_rc();
    {
        let mut guard = rc.borrow_mut();
        let view = guard.as_mut().view_mut();
        *view.counter = 42;
        view.data[0] = 99;
    }
    let view = rc.borrow();
    assert_eq!(*view.view().counter, 42);
    assert_eq!(view.view().data[0], 99);
}

use std::cell::RefCell;
use std::sync::Mutex;

#[slice_struct(unpin)]
struct Advanced {
    #[slice]
    string: str,
    #[slice]
    locked: Mutex<[u8]>,
    #[slice]
    cell: RefCell<[u32]>,
}

#[test]
fn test_advanced() {
    let a = Advanced::init_iter(
        "hello".bytes(),
        [1, 2, 3].into_iter(),
        [100, 200].into_iter(),
    )
    .in_box();
    assert_eq!(a.view().string, "hello");
    assert_eq!(*a.view().locked, [1, 2, 3]);
    assert_eq!(*a.view().cell, [100, 200]);

    a.view().locked[0] = 99;
    a.view().cell[1] = 999;

    assert_eq!(*a.view().locked, [99, 2, 3]);
    assert_eq!(*a.view().cell, [100, 999]);
}
