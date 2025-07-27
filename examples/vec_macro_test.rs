use arc_vec::{alloc::arc_vec::ArcVec, arc_vec};

fn main() {
    let arc_test: ArcVec<i32> = arc_vec![];
    println!("arc_vec : {arc_test}");

    let arc_test_02 = arc_vec![1, 2, 3];
    println!("arc_vec : {arc_test_02}");
}
