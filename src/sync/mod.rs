use rayon::prelude::*;

use crate::alloc::arc_vec::ArcVec;

pub fn parallel_sort<T>(data: &ArcVec<T>)
where
    T: Ord + Send + Clone,
{
    let (mut chunks, len) = {
        let raw = data.data.lock().unwrap();
        let len = raw.len;
        let _chunks: Vec<T> = Vec::new();

        // Extract all values
        let values: Vec<T> = (0..raw.len)
            .map(|i| unsafe { raw.buf[i].assume_init_ref().clone() })
            .collect();

        // Drop lock
        (values, len)
    };

    // Sort in parallel
    chunks.par_sort();

    // Put sorted data back into ArcVec
    let mut raw = data.data.lock().unwrap();
    for (i, item) in chunks.into_iter().enumerate() {
        raw.buf[i].write(item);
    }
    raw.len = len;
}
