use arc_vec::alloc::arc_vec::ArcVec;
use arc_vec::sync::parallel_sort;
use criterion::{BenchmarkId, Criterion, criterion_group};
use rand::seq::SliceRandom;
use std::hint::black_box;

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_compare");

    for size in [1024, 16_384, 1_048_576].iter() {
        let mut data: Vec<i32> = (0..*size).collect();
        data.shuffle(&mut rand::rng());
        let arc_vec: ArcVec<i32> = data.into_iter().collect();

        group.bench_with_input(BenchmarkId::new("basic_sort", size), &arc_vec, |b, data| {
            b.iter_batched(
                || data.clone(),
                |d| {
                    d.sort();
                    black_box(d);
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_with_input(
            BenchmarkId::new("rayon_parallel_sort", size),
            &arc_vec,
            |b, data| {
                b.iter_batched(
                    || data.clone(),
                    |d| {
                        parallel_sort(&d);
                        black_box(d);
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_sorts);
