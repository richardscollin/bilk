/// returns an iterator of adjacent tuples (row, column)
#[rustfmt::skip]
pub fn neighbors(
    r: usize,
    c: usize,
    height: usize,
    width: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ]
    .into_iter()
    .map(move |(dr, dc)| (r as isize + dr, c as isize + dc))
    .filter_map(move |(nr, nc)| {
        if (0..height as isize).contains(&nr) && (0..width as isize).contains(&nc) {
            Some((nr as usize, nc as usize))
        } else {
            None
        }
    })
}
