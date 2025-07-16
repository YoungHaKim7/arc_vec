use std::sync::Arc;

#[derive(Debug)]
struct ArcVeci32 {
    data: Arc<[i32; 16]>,
}

impl ArcVeci32 {
    fn new() -> Self {
        Self {
            data: Arc::new([0; 16]),
        }
    }
}

impl Default for ArcVeci32 {
    fn default() -> Self {
        Self {
            data: Arc::new([0; 16]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_i32_new() {
        let my_num = ArcVeci32::new();
        dbg!(my_num);
    }

    #[test]
    fn arc_i32_default() {
        let my_num = ArcVeci32::default();
        dbg!(my_num);
    }
}
