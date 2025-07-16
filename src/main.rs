#[derive(Debug)]
struct ArcVeci32 {
    data: [i32; 16],
}

impl ArcVeci32 {
    fn new() -> Self {
        Self { data: [0; 16] }
    }
}

fn main() {
    let my_num = ArcVeci32::new();
    dbg!(my_num);
}
