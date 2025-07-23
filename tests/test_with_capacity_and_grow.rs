use arc_vec::alloc::arc_vec::ArcVec;

#[test]
fn test_with_capacity_and_grow() {
    let my_num = ArcVec::with_capacity(2);
    for i in 1..=5 {
        my_num.push(i);
    }
    println!("with growth: {my_num}");
    assert_eq!(format!("{my_num}"), "(1, 2, 3, 4, 5)");
}
