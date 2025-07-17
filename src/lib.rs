use std::{
    alloc::{alloc, Layout},
    fmt::{self, Display, Debug},
    mem::{MaybeUninit, size_of_val},
    sync::{Arc, Mutex},
};

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

impl<T> ArcVec<T> {
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "capacity must be > 0");

        // Safely create uninitialized buffer
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

        if raw.len < raw.capacity {
            let idx = raw.len;
            raw.buf[idx].write(val);
            raw.len += 1;
        } else {
            println!("Cannot push: capacity reached.");
        }
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
    fn test_with_capacity() {
        let my_num = ArcVec::with_capacity(4);
        my_num.push(1);
        my_num.push(2);
        my_num.push(3);
        println!("with capacity: {}", my_num);
        assert_eq!(format!("{}", my_num), "(1, 2, 3)");
        println!("my_num size: {}", size_of_val(&my_num));
    }

    #[test]
    fn arc_vec_with_string() {
        let my_strs = ArcVec::with_capacity(3);
        my_strs.push("hello".to_string());
        my_strs.push("world".to_string());
        println!("my_strs: {}", my_strs);
    }
}
