use std::fmt::Debug;

pub fn counting_sort<const MAX_ELEMENT: usize>(
    data: &mut [impl Copy + TryInto<usize, Error: Debug> + TryFrom<usize, Error: Debug>],
) {
    let mut counts = [0usize; MAX_ELEMENT];

    for e in data.iter() {
        counts[(*e).try_into().unwrap()] += 1;
    }

    let mut i = 0;
    for (idx, count) in counts.into_iter().enumerate() {
        if count != 0 {
            data[i..(i + count)].fill(idx.try_into().unwrap());
            i += count;
        }
    }
}

#[test]
fn test_counting_sort() {
    let mut x: Vec<i32> = vec![5, 4, 3, 2, 1];
    counting_sort::<8>(&mut x);

    let mut y = x.clone();
    y.sort();

    assert_eq!(x, y);
}
