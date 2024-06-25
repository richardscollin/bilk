#![doc = include_str!("../README.md")]
mod counter;
mod counting_sort;
mod cumsum;
mod lift;
mod neighbors;

pub use counter::{
    Counter,
    IntoCounter,
};
pub use counting_sort::counting_sort;
pub use cumsum::CumSum;
pub use lift::lift;
pub use neighbors::neighbors;

/// Help with the trait bound problem described [here (video)](https://youtu.be/CWiz_RtA1Hw).
pub trait Captures<U> {}
impl<T: ?Sized, U> Captures<U> for T {}
