pub mod backend;
pub mod drawing;
pub mod figure;
pub mod preview;
pub mod styles;

pub use crate::styles::color;
pub use crate::styles::color::{BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW};
pub use figure::Saver;
pub use preview::Preview;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
