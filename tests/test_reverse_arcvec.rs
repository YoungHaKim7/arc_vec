use arc_vec::alloc::arc_vec::ArcVec;

#[test]
fn test_reverse_arcvec() {
    let v = ArcVec::with_capacity(4);
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    v.reverse();
    assert_eq!(format!("{v}"), "(4, 3, 2, 1)");
}
