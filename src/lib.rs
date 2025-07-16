use std::{
    fmt::{self, Display},
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
struct ArcVeci32 {
    data: Arc<Mutex<Vec<i32>>>,
}

impl ArcVeci32 {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn push(&self, val: i32) {
        let mut guard = self.data.lock().unwrap();
        guard.push(val);
    }
}

impl Default for ArcVeci32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ArcVeci32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.data.lock().unwrap();
        write!(f, "(")?;
        for (i, val) in guard.iter().enumerate() {
            write!(f, "{}", val)?;
            if i + 1 < guard.len() {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_push_push() {
        let my_num = ArcVeci32::new();
        my_num.push(42);
        my_num.push(24);
        println!("arc Vec push push : {}", my_num);
        // Expected Output: (42, 24)
    }

    #[test]
    fn arc_push_3x() {
        let my_num = ArcVeci32::new();
        my_num.push(42);
        my_num.push(24);
        my_num.push(99);
        println!("arc Vec push push : {}", my_num);
        // Expected Output: (42, 24, 99)
    }
}
