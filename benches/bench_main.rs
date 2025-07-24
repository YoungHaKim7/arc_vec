mod benchmarks;

use criterion::criterion_main;

criterion_main!(benchmarks::sort_compare::benches);
