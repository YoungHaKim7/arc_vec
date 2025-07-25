use arc_vec::alloc::arc_vec::ArcVec;
use rand::{rng, seq::SliceRandom};

fn generate_data(size: usize) -> ArcVec<i32> {
    let mut vec: Vec<i32> = (0..size as i32).collect();
    vec.shuffle(&mut rng());
    vec.into_iter().collect()
}

fn main() {
    let my_arc_vec = generate_data(1_000);
    println!("before ~~~~sort test(basic) : my_arc_vec: {my_arc_vec}");
    println!("~~~~~start");
    my_arc_vec.sort();
    println!("\nsort ~~~ result:\n");
    println!("after ~~~~ sort test(basic) : my_arc_vec: {my_arc_vec}");
    println!("end ~~~~~~~~~~");
    println!();
    println!();
    let my_arc_vec = generate_data(1_000);
    println!("before ~~~~reverse test(basic) : my_arc_vec: {my_arc_vec}");
    println!("~~~~~start");
    my_arc_vec.reverse();
    println!("\nreverse ~~~ result:\n");
    println!("reverse test(basic) : my_arc_vec: {my_arc_vec}");
    println!("end ~~~~~~~~~~");

    println!();
    println!();
    println!();
    println!();

    let v: ArcVec<i32> = (1..=1000).collect();

    println!("Before reverse: {v}");
    v.reverse();
    println!("After reverse:  {v}");
}
