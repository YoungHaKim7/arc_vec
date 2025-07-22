use arc_vec::alloc::arc_vec::ArcVec;

#[test]
fn arc_vec_new_string_test() {
    let my_num_init: ArcVec<String> = ArcVec::new();
    my_num_init.push_str("hello".to_string());
    my_num_init.push_str("world".to_string());
    my_num_init.push_str("test".to_string());
    println!("my_string_init : {} (string fn test)", my_num_init);
}
