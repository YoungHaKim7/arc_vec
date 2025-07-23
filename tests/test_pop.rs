use arc_vec::alloc::arc_vec::ArcVec;

#[test]
fn test_pop() {
    let my_num = ArcVec::with_capacity(2);
    my_num.push(1);
    my_num.push(2);
    my_num.push(3);

    assert_eq!(format!("{my_num}"), "(1, 2, 3)");
    assert_eq!(my_num.pop(), Some(3));
    assert_eq!(format!("{my_num}"), "(1, 2)");
    assert_eq!(my_num.pop(), Some(2));
    assert_eq!(format!("{my_num}"), "(1)");
    assert_eq!(my_num.pop(), Some(1));
    assert_eq!(format!("{my_num}"), "()");
    assert_eq!(my_num.pop(), None);
}
