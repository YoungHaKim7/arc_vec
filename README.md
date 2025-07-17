# arc_vec
Arc Vector

# 오리지날 영상
- [(유튜브Youtube외부영상)230614_Use Arc Instead of Vec | Logan Smith](https://youtu.be/A4cKi7PTJSs?si=H4r7BYRrw6rTGp4a)

```rs
// Consider Arc<[T]> over Vec <T>
Arc<[T]>
```


# Result

```bash
running 2 tests
arc_vec  val: with capacity : 8
with capacity : (1, 2, 3)
my_num  val: with capacity : 8
test tests::arc_vec_size_of_val ... ok
test tests::test_with_capacity ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

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
