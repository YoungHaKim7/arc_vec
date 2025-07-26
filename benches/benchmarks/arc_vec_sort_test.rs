use arc_vec::alloc::arc_vec::ArcVec;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rng, seq::SliceRandom, Rng};

fn generate_data(size: usize) -> ArcVec<i32> {
    let mut vec: Vec<i32> = (0..size as i32).collect();
    vec.shuffle(&mut rng());
    vec.into_iter().collect()
}

pub fn bench_sort(c: &mut Criterion) {
    let size = 1_000_000;

    c.bench_function("arcvec::sort", |b| {
        b.iter(|| {
            let arcvec = generate_data(size);
            arcvec.sort();
        })
    });

    c.bench_function("arcvec::parallel_sort", |b| {
        b.iter(|| {
            let arcvec = generate_data(size);
            arcvec.parallel_sort();
        })
    });

    c.bench_function("std_vec::sort", |b| {
        b.iter(|| {
            let mut rng = rand::rng();
            let mut std_vec: Vec<i32> = (0..size).map(|_| rng.random_range(0..1_000_000)).collect();
            std_vec.sort();
        })
    });
}

criterion_group!(benches, bench_sort);
criterion_main!(benches);
