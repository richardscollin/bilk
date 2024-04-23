use std::{
    collections::HashMap,
    hash::Hash,
};

#[derive(Clone, Debug)]
pub struct Counter<K, V = usize>(HashMap<K, V>);

impl<T> FromIterator<T> for Counter<T>
where
    T: Eq + Ord + Hash,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().fold(Default::default(), |mut acc, item| {
            *acc.entry(item).or_default() += 1;
            acc
        }))
    }
}

type IntoIter<K, V> = std::collections::hash_map::IntoIter<K, V>;
impl<K, V> IntoIterator for Counter<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K, V> Counter<K, V>
where
    K: Eq + Hash + Ord,
    V: Ord + Default,
{
    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.len() <= other.0.len()
            && self
                .0
                .keys()
                .all(|key| &self.0[key] <= other.0.get(key).unwrap_or(&Default::default()))
    }

    pub fn into_hashmap(self) -> HashMap<K, V> {
        self.into_iter().collect::<HashMap<K, V>>()
    }
}

pub trait IntoCounter<T> {
    fn counter(self) -> Counter<T>;
}
impl<T: Eq + Hash + Ord, I: Iterator<Item = T>> IntoCounter<T> for I {
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

    let _x = counter.into_iter().collect::<HashMap<char, usize>>();
}
