use arc_vec::arc_vec;

fn main() {
    let arc_test = arc_vec!(2);
    println!("arc_vec : {}", arc_test);

    let int_macro_append = arc_vec!(2);
    int_macro_append.push(10);
    int_macro_append.push(9);
    println!("int macro append test: {}", int_macro_append);

    let int_append2 = arc_vec!(5);
    int_macro_append.push(10);
    int_macro_append.push(9);

    int_macro_append.append(&int_append2);
    println!("int macro append test 02: {}", int_macro_append);

    // FIX ME : string error
    // let mut arc_string_macro = arc_vec!("string test");
    // println!("arc_vec : {}", arc_string_macro);

    // let arc_string2 = arc_vec!("good");
    // arc_string_macro.append(&arc_string2);
    // println!("arc_vec : {}", arc_string2);
}
