pub mod alloc;
pub mod macros;

// #[cfg(test)]
// mod tests {
//     use crate::alloc::arc_vec::ArcVec;

//     #[test]
//     fn test_with_capacity_and_grow() {
//         let my_num = ArcVec::with_capacity(2);
//         for i in 1..=5 {
//             my_num.push(i);
//         }
//         println!("with growth: {}", my_num);
//         assert_eq!(format!("{}", my_num), "(1, 2, 3, 4, 5)");
//     }

//     #[test]
//     fn test_pop() {
//         let my_num = ArcVec::with_capacity(2);
//         my_num.push(1);
//         my_num.push(2);
//         my_num.push(3);

//         assert_eq!(format!("{}", my_num), "(1, 2, 3)");
//         assert_eq!(my_num.pop(), Some(3));
//         assert_eq!(format!("{}", my_num), "(1, 2)");
//         assert_eq!(my_num.pop(), Some(2));
//         assert_eq!(format!("{}", my_num), "(1)");
//         assert_eq!(my_num.pop(), Some(1));
//         assert_eq!(format!("{}", my_num), "()");
//         assert_eq!(my_num.pop(), None);
//     }

//     #[test]
//     fn arc_vec_new_default_test() {
//         let my_num_init: ArcVec<i32> = ArcVec::new();
//         my_num_init.push(10);
//         my_num_init.push(9);
//         my_num_init.push(8);
//         println!("my_num_init : {} (new fn test)", my_num_init);

//         let my_num_init_new: ArcVec<i32> = ArcVec::default();
//         my_num_init_new.push(10);
//         my_num_init_new.push(9);
//         my_num_init_new.push(8);
//         println!("my_num_init(default fn test) : {}", my_num_init_new);
//     }

//     #[test]
//     fn arc_vec_new_string_test() {
//         let my_num_init: ArcVec<String> = ArcVec::new();
//         my_num_init.push_str("hello".to_string());
//         my_num_init.push_str("world".to_string());
//         my_num_init.push_str("test".to_string());
//         println!("my_string_init : {} (string fn test)", my_num_init);
//     }

//     #[test]
//     fn arc_vec_macro_test_string() {
//         let my_num_init: ArcVec<String> = crate::arc_vec!();
//         my_num_init.push_str("hello".to_string());
//         my_num_init.push_str("world".to_string());
//         my_num_init.push_str("test".to_string());
//         println!("my_string_init : {} (string fn test)", my_num_init);
//         // (hello, world, test)
//     }

//     #[test]
//     fn arc_vec_macro_test_int() {
//         let my_num_init: ArcVec<i32> = crate::arc_vec!(10);
//         my_num_init.push(10);
//         my_num_init.push(9);
//         my_num_init.push(8);
//         assert_eq!(format!("{}", my_num_init), "(10, 10, 9, 8)");

//         let my_num_init_new: ArcVec<i32> = crate::arc_vec!(1, 2, 3);
//         my_num_init_new.push(10);
//         my_num_init_new.push(9);
//         my_num_init_new.push(8);
//         assert_eq!(format!("{}", my_num_init_new), "(1, 2, 3, 10, 9, 8)");
//     }

//     #[test]
//     fn test_append() {
//         let vec1: ArcVec<i32> = ArcVec::with_capacity(2);
//         vec1.push(1);
//         vec1.push(2);

//         let vec2: ArcVec<i32> = ArcVec::with_capacity(2);
//         vec2.push(3);
//         vec2.push(4);

//         vec1.append(&vec2);

//         assert_eq!(format!("{}", vec1), "(1, 2, 3, 4)");
//         assert_eq!(format!("{}", vec2), "()");
//         assert!(vec2.is_empty());
//     }
// }
