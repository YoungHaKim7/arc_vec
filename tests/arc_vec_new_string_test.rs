use arc_vec::alloc::arc_vec::ArcVec;

#[test]
fn arc_vec_new_string_test() {
    let my_num_init: ArcVec<String> = ArcVec::new();
    my_num_init.push_str("hello".to_string());
    my_num_init.push_str("world".to_string());
    my_num_init.push_str("test".to_string());
    assert_eq!(format!("{my_num_init}"), "(hello, world, test)");
}
