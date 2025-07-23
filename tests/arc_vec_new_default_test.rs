use arc_vec::alloc::arc_vec::ArcVec;

#[test]
fn arc_vec_new_default_test() {
    let my_num_init: ArcVec<i32> = ArcVec::new();
    my_num_init.push(10);
    my_num_init.push(9);
    my_num_init.push(8);
    println!("my_num_init : {my_num_init} (new fn test)");

    let my_num_init_new: ArcVec<i32> = ArcVec::default();
    my_num_init_new.push(10);
    my_num_init_new.push(9);
    my_num_init_new.push(8);
    println!("my_num_init(default fn test) : {my_num_init_new}");
}
