use arc_vec::alloc::arc_vec::ArcVec;
use rand::seq::SliceRandom;
use std::time::Instant;

fn main() {
    let size = 1_000_000;
    let mut data: Vec<i32> = (0..size).collect();
    data.shuffle(&mut rand::rng());

    let arc_vec: ArcVec<i32> = data.into_iter().collect();

    // Time the standard sort
    let sort_arc_vec = arc_vec.clone();
    let start_sort = Instant::now();
    sort_arc_vec.sort();
    let duration_sort = start_sort.elapsed();

    // Time the parallel sort
    let parallel_sort_arc_vec = arc_vec.clone();
    let start_parallel_sort = Instant::now();
    parallel_sort_arc_vec.parallel_sort();
    let duration_parallel_sort = start_parallel_sort.elapsed();

    println!("Array size: {size}");
    println!("Standard sort took: {duration_sort:?}");
    println!("Parallel sort took: {duration_parallel_sort:?}");
}
