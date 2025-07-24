use std::{
    fmt::{self, Display},
    mem::MaybeUninit,
    ptr,
    sync::{Arc, Mutex},
};

use rayon::prelude::ParallelSliceMut;

#[derive(Debug, Clone)]
pub struct ArcVec<T> {
    pub data: Arc<Mutex<RawArcVec<T>>>,
}

#[derive(Debug)]
pub struct RawArcVec<T> {
    pub buf: Box<[MaybeUninit<T>]>,
    pub len: usize,
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
            for (i, item) in self_raw.buf.iter().enumerate().take(self_raw.len) {
                unsafe {
                    let src = item.as_ptr();
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

            for (i, item) in raw.buf.iter().enumerate().take(raw.len) {
                unsafe {
                    let src = item.as_ptr();
                    let dst = new_buf[i].as_mut_ptr();
                    std::ptr::copy_nonoverlapping(src, dst, 1);
                }
            }

            raw.buf = new_buf.into_boxed_slice();
            raw.capacity = new_capacity;
            // println!("Capacity doubled to {new_capacity}");
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

            for (i, item) in raw.buf.iter().enumerate().take(raw.len) {
                unsafe {
                    let src = item.as_ptr();
                    let dst = new_buf[i].as_mut_ptr();
                    std::ptr::copy_nonoverlapping(src, dst, 1);
                }
            }

            raw.buf = new_buf.into_boxed_slice();
            raw.capacity = new_capacity;
            println!("Capacity doubled to {new_capacity}");
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

        raw.len == 0
    }

    pub fn sort_by<F>(&self, mut compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        let mut raw = self.data.lock().unwrap();

        // Step 1: move items out into Vec<T>
        let mut temp: Vec<T> = Vec::with_capacity(raw.len);
        for i in 0..raw.len {
            unsafe {
                temp.push(raw.buf[i].assume_init_read());
            }
        }

        // Step 2: sort temp Vec
        temp.sort_by(&mut compare);

        // Step 3: move sorted items back
        for (i, val) in temp.into_iter().enumerate() {
            raw.buf[i].write(val);
        }
    }
    pub fn reverse(&self) {
        let mut raw = self.data.lock().unwrap();
        let len = raw.len;

        for i in 0..len / 2 {
            unsafe {
                let a = raw.buf[i].as_mut_ptr();
                let b = raw.buf[len - 1 - i].as_mut_ptr();
                ptr::swap(a, b);
            }
        }
    }
}

impl<T: Ord + Send + Sync> ArcVec<T> {
    pub fn sort(&self) {
        self.sort_by(|a, b| a.cmp(b));
    }

    pub fn parallel_sort(&self) {
        let mut raw = self.data.lock().unwrap();
        let slice =
            unsafe { std::slice::from_raw_parts_mut(raw.buf.as_mut_ptr() as *mut T, raw.len) };
        slice.par_sort();
    }
}

impl<T: Display> Display for ArcVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let raw = self.data.lock().unwrap();
        write!(f, "(")?;
        for (i, item) in raw.buf.iter().enumerate().take(raw.len) {
            unsafe {
                write!(f, "{}", item.assume_init_ref())?;
            }
            if i + 1 < raw.len {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}
