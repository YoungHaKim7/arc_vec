use arc_vec::alloc::arc_vec::ArcVec;

#[test]
fn test_parallel_sort_arcvec() {
    let v = ArcVec::with_capacity(4);
    v.push(10);
    v.push(2);
    v.push(33);
    v.push(5);

    println!("Before sort: {v}");
    v.parallel_sort();
    println!("After sort: {v}");
}
