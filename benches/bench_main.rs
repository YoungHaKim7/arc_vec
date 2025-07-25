use criterion::criterion_main;

mod benchmarks;

criterion_main!(
    benchmarks::sort_compare::benches,
    benchmarks::arc_vec_sort_test::benches,
    benchmarks::arc_vec_reverse_test::benches
);
