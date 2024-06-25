/// counting sort using stack allocated array
///
/// # Panics
///
/// If an element is equal to or larger then `MAX_ELEMENT`.
///
/// # Example
///
/// ```
/// # use bilk::counting_sort;
/// let mut x: Vec<i32> = vec![5, 4, 3, 2, 1];
///
/// counting_sort::<8>(&mut x);
///
/// assert_eq!(x, vec![1, 2, 3, 4, 5]);
/// ```
pub fn counting_sort<const MAX_ELEMENT: usize>(
    data: &mut [impl Copy
              + TryInto<usize, Error: std::fmt::Debug>
              + TryFrom<usize, Error: std::fmt::Debug>],
) {
    let mut counts = [0usize; MAX_ELEMENT];

    for &e in data.iter() {
        counts[e.try_into().unwrap()] += 1;
    }

    let mut i = 0;
    for (idx, count) in counts.into_iter().enumerate() {
        if count != 0 {
            data[i..(i + count)].fill(idx.try_into().unwrap());
            i += count;
        }
    }
}
