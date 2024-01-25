#![doc = include_str!("../README.md")]
mod auto;
mod counter;
mod cumsum;
mod neighbors;

pub use auto::auto;
pub use counter::{Counter, IntoCounter};
pub use cumsum::CumSum;
pub use neighbors::neighbors;
