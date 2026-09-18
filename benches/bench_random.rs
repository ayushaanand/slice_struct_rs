use criterion::{criterion_group, criterion_main, Criterion};
use slice_struct::slice_struct;
use std::hint::black_box;

// The Standard Vec-based Packet
#[derive(Clone)]
pub struct StandardPacket {
    pub id: u32,
    pub payload: Vec<u8>,
    pub tags: Vec<u32>,
}

// Our Optimized Slice-based Packet
#[slice_struct]
pub struct SlicePacket {
    pub id: u32,
    #[slice] pub payload: [u8],
    #[slice] pub tags: [u32],
}

fn bench_random_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("Random Access");
    
    let standard = Box::new(StandardPacket {
        id: 42,
        payload: vec![1; 10000],
        tags: vec![1; 10000],
    });
    
    let slice_p = SlicePacket::init_def(42, (1, 10000), (1, 10000)).in_box();
    
    // Pseudo-random deterministic scatter using primes
    let indices: Vec<usize> = (0..1000).map(|i| (i * 997) % 10000).collect();

    group.bench_function("Standard Random Access", |b| {
        b.iter(|| {
            let mut sum: u32 = 0;
            for &idx in &indices {
                sum = sum.wrapping_add(standard.payload[idx] as u32);
                sum = sum.wrapping_add(standard.tags[idx]);
            }
            black_box(sum);
        })
    });

    group.bench_function("slice_struct Random Access", |b| {
        b.iter(|| {
            let mut sum: u32 = 0;
            let v = slice_p.view();
            for &idx in &indices {
                sum = sum.wrapping_add(v.payload[idx] as u32);
                sum = sum.wrapping_add(v.tags[idx]);
            }
            black_box(sum);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_random_access);
criterion_main!(benches);
