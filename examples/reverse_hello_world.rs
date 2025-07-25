use arc_vec::alloc::arc_vec::ArcVec;

fn main() {
    // Time the standard sort
    let sort_arc_vec = ArcVec::new();
    sort_arc_vec.push(1);
    sort_arc_vec.push(10);
    sort_arc_vec.push(20);
    sort_arc_vec.push(40);
    sort_arc_vec.push(100);
    sort_arc_vec.reverse();
    println!("Standard reverse: {sort_arc_vec}");

    // the parallel sort
    let parallel_sort_arc_vec = ArcVec::new();
    parallel_sort_arc_vec.push(1);
    parallel_sort_arc_vec.push(10);
    parallel_sort_arc_vec.push(20);
    parallel_sort_arc_vec.push(40);
    parallel_sort_arc_vec.push(100);
    parallel_sort_arc_vec.parallel_reverse();

    println!("Parallel reverse test: {parallel_sort_arc_vec}");
}
