# arc_vec
Arc Vector

# 오리지날 영상
- [(유튜브Youtube외부영상)230614_Use Arc Instead of Vec | Logan Smith](https://youtu.be/A4cKi7PTJSs?si=H4r7BYRrw6rTGp4a)

```rs
// Consider Arc<[T]> over Vec <T>
Arc<[T]>
```


# Result

- `cargo nextest run --nocapture`

```bash
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.85s
    Starting 5 tests across 1 binary
       START             arc_vec tests::arc_vec_macro_test_int

running 1 test
Capacity doubled to 2
Capacity doubled to 4
my_num_init : (10, 10, 9, 8) (new fn test)
Capacity doubled to 6
my_num_init(default fn test) : (1, 2, 3, 10, 9, 8)
test tests::arc_vec_macro_test_int ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

        PASS [   0.631s] arc_vec tests::arc_vec_macro_test_int
       START             arc_vec tests::arc_vec_macro_test_string

running 1 test
Capacity doubled to 2
Capacity doubled to 4
my_string_init : (hello, world, test) (string fn test)
test tests::arc_vec_macro_test_string ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

        PASS [   0.683s] arc_vec tests::arc_vec_macro_test_string
       START             arc_vec tests::arc_vec_new_default_test

running 1 test
Capacity doubled to 2
Capacity doubled to 4
my_num_init : (10, 9, 8) (new fn test)
Capacity doubled to 2
Capacity doubled to 4
my_num_init(default fn test) : (10, 9, 8)
test tests::arc_vec_new_default_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

        PASS [   0.686s] arc_vec tests::arc_vec_new_default_test
       START             arc_vec tests::arc_vec_new_string_test

running 1 test
Capacity doubled to 2
Capacity doubled to 4
my_string_init : (hello, world, test) (string fn test)
test tests::arc_vec_new_string_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

        PASS [   0.692s] arc_vec tests::arc_vec_new_string_test
       START             arc_vec tests::test_with_capacity_and_grow

running 1 test
Capacity doubled to 4
Capacity doubled to 8
with growth: (1, 2, 3, 4, 5)
test tests::test_with_capacity_and_grow ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

        PASS [   0.683s] arc_vec tests::test_with_capacity_and_grow
------------
     Summary [   3.375s] 5 tests run: 5 passed, 0 skipped

```

- `cargo t -- --nocapture`

```bash

running 5 tests
Capacity doubled to 2
Capacity doubled to 4
my_num_init : (10, 10, 9, 8) (new fn test)
Capacity doubled to 6
my_num_init(default fn test) : (1, 2, 3, 10, 9, 8)
Capacity doubled to 4
Capacity doubled to 8
with growth: (1, 2, 3, 4, 5)
Capacity doubled to 2
Capacity doubled to 4
my_string_init : (hello, world, test) (string fn test)
Capacity doubled to 2
test tests::arc_vec_macro_test_int ... Capacity doubled to 4
my_string_init : (hello, world, test) (string fn test)
Capacity doubled to 2
Capacity doubled to 4
my_num_init : (10, 9, 8) (new fn test)
Capacity doubled to 2
Capacity doubled to 4
my_num_init(default fn test) : (10, 9, 8)
ok
test tests::test_with_capacity_and_grow ... ok
test tests::arc_vec_macro_test_string ... ok
test tests::arc_vec_new_string_test ... ok
test tests::arc_vec_new_default_test ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests arc_vec

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```


# `Arc` Vector의 용량이 저렴해 최적화에 유리한 가능성이 보인다. 잘 만들어보자.

```rs

use std::sync::Arc;

fn main() {
    let my_vec: Vec<i32> = vec![];
    let my_vec_capacity = vec![1, 2, 3, 4, 5];
    println!("vec : capacity");
    println!(
        "vec : empty capacity : {} bytes",
        std::mem::size_of_val(&my_vec)
    );
    println!(
        "vec [1, 2, 3, 4, 5] : capacity : {} bytes",
        std::mem::size_of_val(&my_vec_capacity)
    );
    let mut my_vec_new_capacity = Vec::new();
    my_vec_new_capacity.push(99);
    println!(
        "vec [99] : capacity : {} bytes",
        std::mem::size_of_val(&my_vec_new_capacity)
    );

    println!();
    println!("Arc Vector~~~~");

    let arc_vec = Arc::new([1, 2, 3]);
    println!(
        "Arc [1, 2, 3, 4, 5] : capacity : {} bytes",
        std::mem::size_of_val(&arc_vec)
    );
}
```

- Result

```bash
vec : capacity
vec : empty capacity : 24 bytes
vec [1, 2, 3, 4, 5] : capacity : 24 bytes
vec [99] : capacity : 24 bytes

Arc Vector~~~~
Arc [1, 2, 3, 4, 5] : capacity : 8 bytes
```


# Arc 소유권이해
- https://github.com/YoungHaKim7/arc_vec/commit/1f1e54895d8ed76a49c8fb35a83cd26cef6f4697

- 해결책

```rs
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
```


<hr />

- 문제의 코드

```rs
pub fn push(&self, val: T) {
    let mut raw = self.data.lock().unwrap();

    if raw.len == raw.capacity {
        // Double the capacity using shift operator
        let new_capacity = raw.capacity << 1;

        let mut new_buf: Vec<MaybeUninit<T>> = Vec::with_capacity(new_capacity);
        // Pre-fill with uninit
        new_buf.resize_with(new_capacity, MaybeUninit::uninit);

        // Move initialized items
        for i in 0..raw.len {
            unsafe {
                let src = raw.buf[i].as_ptr();
                let dst = new_buf[i].as_mut_ptr();
                ptr::copy_nonoverlapping(src, dst, 1);
            }
        }

        raw.buf = new_buf.into_boxed_slice();
        raw.capacity = new_capacity;
        println!("Capacity doubled to {}", new_capacity);
    }

    raw.buf[raw.len].write(val);
    raw.len += 1;
}

```

```bash
error[E0502]: cannot borrow `raw` as immutable because it is also borrowed as mutable
   --> src/lib.rs:187:17
    |
187 |         raw.buf[raw.len].write(val);
    |         --------^^^-----
    |         |       |
    |         |       immutable borrow occurs here
    |         mutable borrow occurs here
    |         mutable borrow later used here
    |
help: try adding a local storing this...
   --> src/lib.rs:187:17
    |
187 |         raw.buf[raw.len].write(val);
    |                 ^^^^^^^
help: ...and then using that local here
   --> src/lib.rs:187:9
    |
187 |         raw.buf[raw.len].write(val);
    |         ^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0502`.
warning: `arc_vec` (lib) generated 1 warning
```

# Union MaybeUninit
- 1.36.0 · Source
  - https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html

# vector보면서 더 깊이 들어가기
- https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
- vec macro
  - https://github.com/rust-lang/rust/blob/master/library/alloc/src/macros.rs
- raw_vec
  - https://github.com/rust-lang/rust/blob/master/library/alloc/src/raw_vec/mod.rs
