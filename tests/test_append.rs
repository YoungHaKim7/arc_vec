use arc_vec::alloc::arc_vec::ArcVec;

#[test]
fn test_append() {
    let vec1: ArcVec<i32> = ArcVec::with_capacity(2);
    vec1.push(1);
    vec1.push(2);

    let vec2: ArcVec<i32> = ArcVec::with_capacity(2);
    vec2.push(3);
    vec2.push(4);

    vec1.append(&vec2);

    assert_eq!(format!("{vec1}"), "(1, 2, 3, 4)");
    assert_eq!(format!("{vec2}"), "()");
    assert!(vec2.is_empty());
}
