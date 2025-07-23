use arc_vec::alloc::arc_vec::ArcVec;
use arc_vec::arc_vec;

#[test]
fn arc_vec_macro_test_int() {
    let my_num_init: ArcVec<i32> = arc_vec!(10);
    my_num_init.push(10);
    my_num_init.push(9);
    my_num_init.push(8);
    assert_eq!(format!("{my_num_init}"), "(10, 10, 9, 8)");

    let my_num_init_new: ArcVec<i32> = arc_vec!(1, 2, 3);
    my_num_init_new.push(10);
    my_num_init_new.push(9);
    my_num_init_new.push(8);
    assert_eq!(format!("{my_num_init_new}"), "(1, 2, 3, 10, 9, 8)");
}
