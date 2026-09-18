use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use slice_struct::slice_struct;

#[derive(Clone)]
pub struct StandardPacket {
    pub id: u32,
    pub payload: Vec<u8>,
    pub tags: Vec<u32>,
}

#[slice_struct]
pub struct SlicePacket {
    pub id: u32,
    #[slice] pub payload: [u8],
    #[slice] pub tags: [u32],
}

fn bench_drop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Deallocation (Drop)");

    group.bench_function("Standard (3 Drops)", |b| {
        b.iter_batched(
            || {
                Box::new(StandardPacket {
                    id: 42,
                    payload: vec![0; 1024],
                    tags: vec![0; 128],
                })
            },
            |p| {
                // Drop happens here
                drop(std::hint::black_box(p));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("slice_struct (1 Drop)", |b| {
        b.iter_batched(
            || {
                SlicePacket::init_def(42, (0, 1024), (0, 128)).in_box()
            },
            |p| {
                // Drop happens here
                drop(std::hint::black_box(p));
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_drop);
criterion_main!(benches);
