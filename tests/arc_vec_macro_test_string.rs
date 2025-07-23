use arc_vec::alloc::arc_vec::ArcVec;
use arc_vec::arc_vec;

#[test]
fn arc_vec_macro_test_string() {
    let my_num_init: ArcVec<String> = arc_vec!();
    my_num_init.push_str("hello".to_string());
    my_num_init.push_str("world".to_string());
    my_num_init.push_str("test".to_string());
    println!("my_string_init : {my_num_init} (string fn test)");
    // (hello, world, test)
}
