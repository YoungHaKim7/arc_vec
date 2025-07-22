//! * Vector to use for parallel running. Experimental project.
//! * I made the code after watching a [YouTube video.(230614_Use Arc Instead of Vec | Logan Smith)][youtube]
//!
//!
//! ```rs
//! // Consider Arc<[T]> over Vec <T>
//! Arc<[T]>
//! ```
//!
//! # example
//! ```rs
//! use arc_vec::alloc::arc_vec::ArcVec;
//!
//! fn main() {
//!     let my_arc_vec = ArcVec::new();
//!     my_arc_vec.push(10);
//!     println!("arc_vec int push : {}", my_arc_vec);
//! }
//! ```
//!
//! # documents referenced in the making
//! * [Union MaybeUninit(rust ver. 1.36.0)][union_maybeuninit]
//!
//! # vector deep dive
//!
//! * [rust vector][rustvector]
//!
//! * [vector macro][vectormacro]
//!
//! * [raw vec][rawvec-link]
//!
//! * [trait ToString][tostring-link]
//!
//! * [fn.copy_nonoverlapping][copynonoverlapping-link]
//!
//! [youtube]: https://youtu.be/A4cKi7PTJSs?si=H4r7BYRrw6rTGp4a
//! [union_maybeuninit]: https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html
//! [rustvector]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
//! [vectormacro]: https://github.com/rust-lang/rust/blob/master/library/alloc/src/macros.rs
//!
//! [rawvec-link]: <https://github.com/rust-lang/rust/blob/master/library/alloc/src/raw_vec/mod.rs>
//! [tostring-link]: <https://doc.rust-lang.org/src/alloc/string.rs.html#2728-2804>
//! [copynonoverlapping-link]: <https://doc.rust-lang.org/stable/std/ptr/fn.copy_nonoverlapping.html>

pub mod alloc;
pub mod macros;
