use std::sync::Arc;

#[derive(Debug)]
struct ArcVeci32 {
    data: Arc<Vec<i32>>,
}

impl ArcVeci32 {
    fn new() -> Self {
        Self {
            data: Arc::new(vec![0]),
        }
    }

    fn push(&self, val: i32) -> Self {
        let mut vec_clone = (*self.data).clone();
        vec_clone.push(val);
        Self {
            data: Arc::new(vec_clone),
        }
    }
}

impl Default for ArcVeci32 {
    fn default() -> Self {
        Self::new()
    }
}

fn arc_check_init(data: &Arc<Vec<i32>>) -> bool {
    data.len() == 1
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
    fn arc_i32_push() {
        let my_num = ArcVeci32::new();
        let pushed = my_num.push(42);
        dbg!(pushed);
    }

    #[test]
    fn arc_i32_default() {
        let my_num = ArcVeci32::default();
        dbg!(my_num);
    }

    #[test]
    fn arc_check() {
        let my_num = ArcVeci32::new();
        assert!(arc_check_init(&my_num.data));
    }
}
