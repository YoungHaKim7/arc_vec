use arc_vec::alloc::arc_vec::ArcVec;

fn main() {
    let my_arc_vec = ArcVec::new();
    my_arc_vec.push(10);
    println!("arc_vec int push : {}", my_arc_vec);
}
