use arc_vec::alloc::arc_vec::ArcVec;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::{rng, seq::SliceRandom};

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
}

criterion_group!(benches, bench_sort);
criterion_main!(benches);
