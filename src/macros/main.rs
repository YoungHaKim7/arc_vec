#[macro_export]
macro_rules! arc_vec {
    () => {
        $crate::alloc::arc_vec::ArcVec::new()
    };
    ($elem:expr; $n:expr) => {{
        let arc_vec = $crate::alloc::arc_vec::ArcVec::with_capacity($n);
        for _ in 0..$n {
            arc_vec.push($elem.clone());
        }
        arc_vec
    }};
    ($($x:expr),+ $(,)?) => {{
        let count = $crate::count_exprs!($($x),*);
        let arc_vec = $crate::alloc::arc_vec::ArcVec::with_capacity(count);
        $(
            arc_vec.push($x);
        )*
        arc_vec
    }};
}

#[macro_export]
macro_rules! count_exprs {
    () => { 0 };
    ($head:expr $(, $tail:expr)*) => { 1 + $crate::count_exprs!($($tail),*) };
}
