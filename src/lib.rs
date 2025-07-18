use std::{
    fmt::{self, Debug, Display},
    mem::MaybeUninit,
    sync::{Arc, Mutex},
};

mod macro_rs;

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
            println!("Capacity doubled to {}", new_capacity);
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
    }

    #[test]
    fn arc_vec_macro_test_int() {
        let my_num_init: ArcVec<i32> = arc_vec!(10);
        my_num_init.push(10);
        my_num_init.push(9);
        my_num_init.push(8);
        println!("my_num_init : {} (new fn test)", my_num_init);

        let my_num_init_new: ArcVec<i32> = arc_vec!();
        my_num_init_new.push(10);
        my_num_init_new.push(9);
        my_num_init_new.push(8);
        println!("my_num_init(default fn test) : {}", my_num_init_new);
    }
}
