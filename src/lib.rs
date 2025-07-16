use std::{
    fmt::{self, Display},
    sync::Arc,
};

struct ArcVeci32 {
    data: Arc<[i32; 1]>,
}

impl ArcVeci32 {
    fn new() -> Self {
        Self {
            data: Arc::new([0]),
        }
    }

    fn push(&self, val: i32) -> Self {
        Self {
            data: Arc::new([val]),
        }
    }
}

impl Default for ArcVeci32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ArcVeci32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.data[0])
    }
}

fn arc_check_init(data: &Arc<[i32; 1]>) -> bool {
    data[0] == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_i32_new() {
        let my_num = ArcVeci32::new();
        println!("default : {}", my_num);
    }

    #[test]
    fn arc_i32_push() {
        let my_num = ArcVeci32::new();
        let pushed = my_num.push(42);
        println!("pushed : {}", pushed);
        assert_eq!(*pushed.data, [42]);
    }

    #[test]
    fn arc_i32_default() {
        let my_num = ArcVeci32::default();
        println!("default : {}", my_num);
    }

    #[test]
    fn arc_check() {
        let my_num = ArcVeci32::new();
        println!("default : {}", my_num);
        assert!(arc_check_init(&my_num.data));
    }
}
