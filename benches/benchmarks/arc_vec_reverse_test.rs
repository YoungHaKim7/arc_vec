use arc_vec::alloc::arc_vec::ArcVec;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::{rng, seq::SliceRandom};

fn generate_data(size: usize) -> ArcVec<i32> {
    let mut vec: Vec<i32> = (0..size as i32).collect();
    vec.shuffle(&mut rng());
    vec.into_iter().collect()
}

pub fn bench_reverse(c: &mut Criterion) {
    let size = 1_000_000;

    c.bench_function("arcvec::reverse", |b| {
        b.iter(|| {
            let arcvec = generate_data(size);
            arcvec.reverse();
        })
    });

    c.bench_function("arcvec::parallel_reverse", |b| {
        b.iter(|| {
            let arcvec = generate_data(size);
            arcvec.parallel_reverse();
        })
    });
}

criterion_group!(benches, bench_reverse);
criterion_main!(benches);
