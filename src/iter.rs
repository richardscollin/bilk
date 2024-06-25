use std::collections::{BTreeMap, HashMap};

use std::cmp::Ord;
use std::hash::Hash;

///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use bilk::ExtendOrd as _;
///
/// let mut x = HashMap::from([("a", 5), ("b", 3)]);
/// let mut y = HashMap::from([("a", 4), ("b", 7)]);
///
/// x.extend_min(y.into_iter());
///
/// assert_eq!(x, HashMap::from([("a", 4), ("b", 3)]));
/// ```
///
pub trait ExtendOrd<K, V> {
    fn extend_min<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
        V: std::cmp::Ord;

    fn extend_max<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
        V: std::cmp::Ord;
}

impl<K: Eq + Hash, V: Copy> ExtendOrd<K, V> for HashMap<K, V> {
    fn extend_min<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
        V: Ord,
    {
        for (k, v) in iter {
            use std::collections::hash_map::Entry;
            match self.entry(k) {
                Entry::Occupied(mut occupied) => {
                    *occupied.get_mut() = (*occupied.get()).min(v);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(v);
                }
            }
        }
    }

    fn extend_max<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
        V: Ord,
    {
        for (k, v) in iter {
            use std::collections::hash_map::Entry;
            match self.entry(k) {
                Entry::Occupied(mut occupied) => {
                    *occupied.get_mut() = (*occupied.get()).max(v);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(v);
                }
            }
        }
    }
}

impl<K: Ord, V: Copy> ExtendOrd<K, V> for BTreeMap<K, V> {
    fn extend_min<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
        V: std::cmp::Ord,
    {
        for (k, v) in iter {
            use std::collections::btree_map::Entry;
            match self.entry(k) {
                Entry::Occupied(mut occupied) => {
                    *occupied.get_mut() = (*occupied.get()).min(v);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(v);
                }
            }
        }
    }

    fn extend_max<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
        V: std::cmp::Ord,
    {
        for (k, v) in iter {
            use std::collections::btree_map::Entry;
            match self.entry(k) {
                Entry::Occupied(mut occupied) => {
                    *occupied.get_mut() = (*occupied.get()).max(v);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(v);
                }
            }
        }
    }
}
