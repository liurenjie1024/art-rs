use std::collections::btree_map::BTreeMap;

mod entry;
mod node;
mod navigate;
pub mod tree;
mod util;
pub mod marker;


pub(crate) use util::*;

//
// #[cfg(test)]
// mod tests {
//     use std::mem::size_of;
//
//     enum Test {
//         A(i8),
//         B(i16)
//     }
//
//     #[test]
//     fn it_works() {
//         println!("Size of {}", size_of::<Test>());
//     }
// }
