/// Tally occurrences using `.collect::<Counter>()`
#[derive(Clone, Debug)]
pub struct Counter<K, V = usize>(std::collections::HashMap<K, V>);

impl<T> FromIterator<T> for Counter<T>
where
    T: Eq + Ord + std::hash::Hash,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().fold(Default::default(), |mut acc, item| {
            *acc.entry(item).or_default() += 1;
            acc
        }))
    }
}

impl<T> Counter<T>
where
    T: Eq + Ord + std::hash::Hash,
{
    pub fn intersect(mut self, other: Self) -> Self {
        self.0.retain(|k, _| other.0.contains_key(k));
        for (k, v) in other.0 {
            let x = self.0.entry(k).or_default();
            *x = usize::min(*x, v);
        }
        self
    }
}

impl<T> Counter<T>
where
    T: Clone,
{
    pub fn into_iter_multi(self) -> impl Iterator<Item = T> {
        // use repeat_n when: <https://github.com/rust-lang/rust/issues/104434> is merged
        self.0
            .into_iter()
            .flat_map(|(k, v)| std::iter::repeat(k).take(v))
    }
}

/*
impl<T> Extend<T> for Counter<T>
where
    T: IntoIterator,
    T::Item: Eq + PartialEq + std::hash::Hash,
{
    fn extend(&mut self, iter: T) {
        for item in iter {
            self.0.entry(item)
        }
    }
}
*/

impl<K, V> IntoIterator for Counter<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K, V> Counter<K, V>
where
    K: Eq + std::hash::Hash + Ord,
    V: Ord + Default,
{
    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.len() <= other.0.len()
            && self
                .0
                .keys()
                .all(|key| &self.0[key] <= other.0.get(key).unwrap_or(&Default::default()))
    }

    pub fn into_hashmap(self) -> std::collections::HashMap<K, V> {
        self.into_iter().collect()
    }
}

/// Tally the occurrences of elements in an iterator
pub trait IntoCounter<T> {
    fn counter(self) -> Counter<T>;
}
impl<T: Eq + std::hash::Hash + Ord, I: Iterator<Item = T>> IntoCounter<T> for I {
    fn counter(self) -> Counter<T> {
        self.collect()
    }
}

#[test]
fn test_counter() {
    let text = "abcdabcde";
    let counter = text.chars().counter();
    assert_eq!(counter.0[&'a'], 2);
    assert_eq!(counter.0[&'e'], 1);

    let text2 = "abcdabcdea";
    let counter2 = text2.chars().counter();
    assert!(counter.is_subset(&counter2));

    let _x: std::collections::HashMap<char, usize> = counter.into_iter().collect();
}
