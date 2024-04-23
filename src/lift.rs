/// lift a function up to be applied on options
///
/// this is only for functions with two arguments of the same
/// type returning one of the same type
pub fn lift<T, F: Fn(T, T) -> T>(f: F, t1: Option<T>, t2: Option<T>) -> Option<T> {
    match (t1, t2) {
        (None, s) | (s, None) => s,
        (Some(a), Some(b)) => Some(f(a, b)),
    }
}

#[test]
fn test_simple_lift() {
    let x = Some(10);
    let y = Some(11);
    let z = None;

    assert_eq!(lift(u64::max, x, y), Some(11));
    assert_eq!(lift(u64::max, x, z), Some(10));
}

#[test]
fn test_reference_lift() {
    let s1 = "asdfa";
    let s2 = "asdf";

    let x = Some(s1);
    let y = Some(s2);
    let z = None;

    assert_eq!(
        lift(
            |a, b| {
                if a.len() > b.len() {
                    a
                } else {
                    b
                }
            },
            x,
            y
        ),
        Some(s1)
    );
    assert_eq!(
        lift(
            |a, b| {
                if a.len() > b.len() {
                    a
                } else {
                    b
                }
            },
            x,
            z
        ),
        Some(s1)
    );
}
