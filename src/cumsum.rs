/// Calculates the cumulative sum of an iterator
pub trait CumSum<T> {
    fn cumsum(self) -> impl Iterator<Item = T>;
}

/// Calculates the cumulative sum of an iterator
///
/// This implementation initializes the accumulator with
/// the default value and AddAssign's each additional value
impl<T, U> CumSum<T> for U
where
    T: Copy + Default + std::ops::AddAssign,
    U: Iterator<Item = T>,
{
    fn cumsum(self) -> impl Iterator<Item = T> {
        self.scan(Default::default(), |acc, x| {
            *acc += x;
            Some(*acc)
        })
    }
}

#[test]
fn test_cumsum() {
    let x = vec![0i32, 1, -1];
    let it = x.into_iter().cumsum().collect::<Vec<_>>();
    assert_eq!(it, vec![0i32, 1, 0]);
}
