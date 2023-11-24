/// A shorter way of writing Default::default
///
/// ```rust,ignore
/// // instead of writing:
/// let x: Vec<i32> = Default::default();
/// // you can write:
/// let y: Vec<i32> = auto();
///
/// assert_eq!(x, y);
/// ```
#[inline]
pub fn auto<T: Default>() -> T {
    Default::default()
}
