#![doc = include_str!("../README.md")]
mod auto;
mod counter;
mod cumsum;
mod lift;
mod neighbors;

pub use auto::auto;
pub use counter::{
    Counter,
    IntoCounter,
};
pub use cumsum::CumSum;
pub use lift::lift;
pub use neighbors::neighbors;

/// A trait to help with the trait bound problem described
/// in this video: <https://youtu.be/CWiz_RtA1Hw>
pub trait Captures<U> {}
impl<T: ?Sized, U> Captures<U> for T {}
