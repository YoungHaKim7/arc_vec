use arc_vec::alloc::arc_vec::ArcVec;
use rand::{rng, seq::SliceRandom};

fn generate_data(size: usize) -> ArcVec<i32> {
    let mut vec: Vec<i32> = (0..size as i32).collect();
    vec.shuffle(&mut rng());
    vec.into_iter().collect()
}

fn main() {
    let my_arc_vec = generate_data(1_000);
    my_arc_vec.sort();
    println!("sort test(basic) : my_arc_vec: {my_arc_vec}");
    let my_arc_vec = generate_data(1_000);
    my_arc_vec.reverse();
    println!("reverse test(basic) : my_arc_vec: {my_arc_vec}");
}
