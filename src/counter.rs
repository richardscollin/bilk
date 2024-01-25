#[derive(Clone, Debug)]
pub struct Counter<T>(std::collections::HashMap<T, usize>);

impl<T: Eq + Ord + std::hash::Hash> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().fold(Default::default(), |mut acc, item| {
            *acc.entry(item).or_default() += 1;
            acc
        }))
    }
}
impl<T: Eq + std::hash::Hash + Ord> Counter<T> {
    pub fn is_subset(&self, other: &Self) -> bool {
        self.len() <= other.len()
            && self.keys().all(|key| {
                if let Some(&o) = other.get(key) {
                    self[key] <= o
                } else {
                    false
                }
            })
    }
}

pub trait IntoCounter<T> {
    fn counter(self) -> Counter<T>;
}
impl<T: Eq + std::hash::Hash + Ord, I: Iterator<Item = T>> IntoCounter<T> for I {
    fn counter(self) -> Counter<T> {
        self.collect()
    }
}

impl<T> std::ops::Deref for Counter<T> {
    type Target = std::collections::HashMap<T, usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> std::ops::DerefMut for Counter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test_counter() {
    let text = "abcdabcde";
    let counter = text.chars().counter();
    assert_eq!(counter[&'a'], 2);
    assert_eq!(counter[&'e'], 1);

    let text2 = "abcdabcdea";
    let counter2 = text2.chars().counter();
    assert!(counter.is_subset(&counter2));
}
