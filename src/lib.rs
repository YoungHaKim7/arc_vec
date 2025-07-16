use std::{
    alloc::{Layout, alloc},
    fmt::{self, Display},
    mem::MaybeUninit,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
struct ArcVeci32 {
    data: Arc<Mutex<RawArcVec>>,
}

#[derive(Debug)]
struct RawArcVec {
    buf: Box<[MaybeUninit<i32>]>,
    len: usize,
    capacity: usize,
}

impl ArcVeci32 {
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "capacity must be > 0");

        // Allocate uninitialized memory for the array
        let mut vec: Vec<MaybeUninit<i32>> = Vec::new();

        unsafe {
            let layout = Layout::array::<MaybeUninit<i32>>(cap).unwrap();
            let ptr = alloc(layout) as *mut MaybeUninit<i32>;

            // Build a slice from the raw pointer
            let slice = std::slice::from_raw_parts_mut(ptr, cap);

            // Take ownership of the allocation
            let buf = Box::from_raw(slice);

            Self {
                data: Arc::new(Mutex::new(RawArcVec {
                    buf,
                    len: 0,
                    capacity: cap,
                })),
            }
        }
    }

    pub fn push(&self, val: i32) {
        let mut raw = self.data.lock().unwrap();

        if raw.len < raw.capacity {
            let idx = raw.len;
            raw.buf[idx].write(val);
            raw.len += 1;
        } else {
            println!("Cannot push: capacity reached.");
        }
    }

    // pub fn push(&self, val: i32) {
    //     let mut raw = self.data.lock().unwrap();
    //     if raw.len < raw.capacity {
    //         raw.buf[raw.len].write(val);
    //         raw.len += 1;
    //     } else {
    //         println!("Cannot push: capacity reached.");
    //     }
    // }
}

impl Display for ArcVeci32 {
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
        let my_num = ArcVeci32::with_capacity(4);
        my_num.push(1);
        my_num.push(2);
        my_num.push(3);
        println!("with capacity : {}", my_num);
        assert_eq!(format!("{}", my_num), "(1, 2, 3)");
        println!("my_num  val: with capacity : {}", size_of_val(&my_num));
    }

    #[test]
    fn arc_vec_size_of_val() {
        let arc_vec = Arc::new([3]);

        println!("arc_vec  val: with capacity : {}", size_of_val(&arc_vec));
    }
}
