use std::{
    fmt::{self, Debug, Display},
    mem::MaybeUninit,
    ptr,
    sync::{Arc, Mutex},
};

mod macros;

#[derive(Debug, Clone)]
struct ArcVec<T> {
    data: Arc<Mutex<RawArcVec<T>>>,
}

#[derive(Debug)]
struct RawArcVec<T> {
    buf: Box<[MaybeUninit<T>]>,
    len: usize,
    capacity: usize,
}

impl<T> Default for ArcVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArcVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "capacity must be > 0");

        let buf: Box<[MaybeUninit<T>]> = (0..cap)
            .map(|_| MaybeUninit::uninit())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            data: Arc::new(Mutex::new(RawArcVec {
                buf,
                len: 0,
                capacity: cap,
            })),
        }
    }
    pub fn append(&self, other: &Self) {
        let mut self_raw = self.data.lock().unwrap();
        let mut other_raw = other.data.lock().unwrap();

        let total_len = self_raw.len + other_raw.len;
        if total_len > self_raw.capacity {
            let mut new_capacity = self_raw.capacity.max(1);
            while new_capacity < total_len {
                new_capacity *= 2;
            }

            let mut new_buf: Vec<MaybeUninit<T>> = Vec::with_capacity(new_capacity);
            new_buf.resize_with(new_capacity, MaybeUninit::uninit);

            // Move existing items
            for i in 0..self_raw.len {
                unsafe {
                    let src = self_raw.buf[i].as_ptr();
                    let dst = new_buf[i].as_mut_ptr();
                    ptr::copy_nonoverlapping(src, dst, 1);
                }
            }

            self_raw.buf = new_buf.into_boxed_slice();
            self_raw.capacity = new_capacity;
        }

        // Move items from other to self
        for i in 0..other_raw.len {
            unsafe {
                let current_len = self_raw.len;
                let dst = self_raw.buf[current_len].as_mut_ptr();
                let src = other_raw.buf[i].as_ptr();
                ptr::copy_nonoverlapping(src, dst, 1);
            }
            self_raw.len += 1;
        }

        // Clear other
        other_raw.len = 0;
    }

    pub fn push(&self, val: T) {
        let mut raw = self.data.lock().unwrap();

        if raw.len == raw.capacity {
            let new_capacity = raw.capacity << 1;
            let mut new_buf: Vec<MaybeUninit<T>> = Vec::with_capacity(new_capacity);
            new_buf.resize_with(new_capacity, MaybeUninit::uninit);

            for i in 0..raw.len {
                unsafe {
                    let src = raw.buf[i].as_ptr();
                    let dst = new_buf[i].as_mut_ptr();
                    std::ptr::copy_nonoverlapping(src, dst, 1);
                }
            }

            raw.buf = new_buf.into_boxed_slice();
            raw.capacity = new_capacity;
            // println!("Capacity doubled to {}", new_capacity);
        }

        let idx = raw.len;
        raw.buf[idx].write(val);
        raw.len += 1;
    }

    pub fn push_str(&self, val: T)
    where
        T: ToString,
    {
        let mut raw = self.data.lock().unwrap();

        if raw.len == raw.capacity {
            let new_capacity = raw.capacity << 1;
            let mut new_buf: Vec<MaybeUninit<T>> = Vec::with_capacity(new_capacity);
            new_buf.resize_with(new_capacity, MaybeUninit::uninit);

            for i in 0..raw.len {
                unsafe {
                    let src = raw.buf[i].as_ptr();
                    let dst = new_buf[i].as_mut_ptr();
                    std::ptr::copy_nonoverlapping(src, dst, 1);
                }
            }

            raw.buf = new_buf.into_boxed_slice();
            raw.capacity = new_capacity;
            println!("Capacity doubled to {}", new_capacity);
        }

        let idx = raw.len;
        raw.buf[idx].write(val);
        raw.len += 1;
    }

    pub fn pop(&self) -> Option<T> {
        let mut raw = self.data.lock().unwrap();

        if raw.len == 0 {
            None
        } else {
            raw.len -= 1;
            let value = unsafe { raw.buf[raw.len].assume_init_read() };
            Some(value)
        }
    }

    pub fn is_empty(&self) -> bool {
        let raw = self.data.lock().unwrap();

        if raw.len == 0 { true } else { false }
    }
}

impl<T: Display> Display for ArcVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let raw = self.data.lock().unwrap();
        write!(f, "(")?;
        for i in 0..raw.len {
            unsafe {
                write!(f, "{}", raw.buf[i].assume_init_ref())?;
            }
            if i + 1 < raw.len {
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
    fn test_with_capacity_and_grow() {
        let my_num = ArcVec::with_capacity(2);
        for i in 1..=5 {
            my_num.push(i);
        }
        println!("with growth: {}", my_num);
        assert_eq!(format!("{}", my_num), "(1, 2, 3, 4, 5)");
    }

    #[test]
    fn test_pop() {
        let my_num = ArcVec::with_capacity(2);
        my_num.push(1);
        my_num.push(2);
        my_num.push(3);

        assert_eq!(format!("{}", my_num), "(1, 2, 3)");
        assert_eq!(my_num.pop(), Some(3));
        assert_eq!(format!("{}", my_num), "(1, 2)");
        assert_eq!(my_num.pop(), Some(2));
        assert_eq!(format!("{}", my_num), "(1)");
        assert_eq!(my_num.pop(), Some(1));
        assert_eq!(format!("{}", my_num), "()");
        assert_eq!(my_num.pop(), None);
    }

    #[test]
    fn arc_vec_new_default_test() {
        let my_num_init: ArcVec<i32> = ArcVec::new();
        my_num_init.push(10);
        my_num_init.push(9);
        my_num_init.push(8);
        println!("my_num_init : {} (new fn test)", my_num_init);

        let my_num_init_new: ArcVec<i32> = ArcVec::default();
        my_num_init_new.push(10);
        my_num_init_new.push(9);
        my_num_init_new.push(8);
        println!("my_num_init(default fn test) : {}", my_num_init_new);
    }

    #[test]
    fn arc_vec_new_string_test() {
        let my_num_init: ArcVec<String> = ArcVec::new();
        my_num_init.push_str("hello".to_string());
        my_num_init.push_str("world".to_string());
        my_num_init.push_str("test".to_string());
        println!("my_string_init : {} (string fn test)", my_num_init);
    }

    #[test]
    fn arc_vec_macro_test_string() {
        let my_num_init: ArcVec<String> = arc_vec!();
        my_num_init.push_str("hello".to_string());
        my_num_init.push_str("world".to_string());
        my_num_init.push_str("test".to_string());
        println!("my_string_init : {} (string fn test)", my_num_init);
        // (hello, world, test)
    }

    #[test]
    fn arc_vec_macro_test_int() {
        let my_num_init: ArcVec<i32> = arc_vec!(10);
        my_num_init.push(10);
        my_num_init.push(9);
        my_num_init.push(8);
        assert_eq!(format!("{}", my_num_init), "(10, 10, 9, 8)");

        let my_num_init_new: ArcVec<i32> = arc_vec!(1, 2, 3);
        my_num_init_new.push(10);
        my_num_init_new.push(9);
        my_num_init_new.push(8);
        assert_eq!(format!("{}", my_num_init_new), "(1, 2, 3, 10, 9, 8)");
    }

    #[test]
    fn test_append() {
        let vec1: ArcVec<i32> = ArcVec::with_capacity(2);
        vec1.push(1);
        vec1.push(2);

        let vec2: ArcVec<i32> = ArcVec::with_capacity(2);
        vec2.push(3);
        vec2.push(4);

        vec1.append(&vec2);

        assert_eq!(format!("{}", vec1), "(1, 2, 3, 4)");
        assert_eq!(format!("{}", vec2), "()");
        assert!(vec2.is_empty());
    }
}
