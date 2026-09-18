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

fn bench_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Allocation");

    group.bench_function("Standard (3 Allocations)", |b| {
        b.iter(|| {
            let p = Box::new(StandardPacket {
                id: black_box(42),
                payload: vec![0; black_box(1024)],
                tags: vec![0; black_box(128)],
            });
            black_box(p);
        })
    });

    group.bench_function("slice_struct (1 Allocation)", |b| {
        b.iter(|| {
            let p = SlicePacket::init_def(
                black_box(42),
                (0, black_box(1024)),
                (0, black_box(128)),
            )
            .in_box();
            black_box(p);
        })
    });

    group.finish();
}

fn bench_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("Iteration (Cache Locality)");

    let num_packets = 10_000;
    
    // We MUST Box the standard packet so it perfectly mimics the heap layout
    // of a slice_struct which is !Unpin and lives behind a Box.
    let standard_packets: Vec<Box<StandardPacket>> = (0..num_packets)
        .map(|i| Box::new(StandardPacket {
            id: i,
            payload: vec![1; 256],
            tags: vec![1; 32],
        }))
        .collect();

    group.bench_function("Standard Iteration", |b| {
        b.iter(|| {
            let mut sum: u32 = 0;
            for p in &standard_packets {
                sum = sum.wrapping_add(p.id);
                for &v in &p.payload {
                    sum = sum.wrapping_add(v as u32);
                }
                for &v in &p.tags {
                    sum = sum.wrapping_add(v);
                }
            }
            black_box(sum);
        })
    });

    let slice_packets: Vec<std::pin::Pin<Box<SlicePacket>>> = (0..num_packets)
        .map(|i| SlicePacket::init_def(i, (1, 256), (1, 32)).in_box())
        .collect();

    group.bench_function("slice_struct Iteration", |b| {
        b.iter(|| {
            let mut sum: u32 = 0;
            for p in &slice_packets {
                let v = p.view();
                sum = sum.wrapping_add(*v.id);
                for &val in v.payload {
                    sum = sum.wrapping_add(val as u32);
                }
                for &val in v.tags {
                    sum = sum.wrapping_add(val);
                }
            }
            black_box(sum);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_allocation, bench_iteration);
criterion_main!(benches);
