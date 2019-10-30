#![allow(clippy::type_repetition_in_bounds)]

use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Iterator;

use ironsea_index::Indexed;
use ironsea_index::Record;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Index<R, K>
where
    K: Clone + Eq + Hash + PartialEq + Ord,
{
    hashmap: HashMap<K, R>,
    keys: Vec<K>,
}

impl<K, R> Index<R, K>
where
    R: Record<K>,
    K: Clone + Eq + Hash + PartialEq + Ord,
{
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = R>,
    {
        let (size, _) = iter.size_hint();
        let mut hashmap = HashMap::with_capacity(size);

        for r in iter {
            hashmap.insert(r.key(), r);
        }

        let mut keys = hashmap.keys().cloned().collect::<Vec<_>>();
        keys.sort_unstable();

        Index { hashmap, keys }
    }

    pub fn keys(&self) -> &Vec<K> {
        &self.keys
    }

    pub fn index(&self, key: &K) -> usize {
        match self.keys.binary_search(key) {
            Ok(i) => i,
            Err(i) => {
                if i >= self.keys.len() {
                    self.keys.len() - 1
                } else {
                    i
                }
            }
        }
    }
}

impl<K, R> Indexed<R, K> for Index<R, K>
where
    R: Record<K>,
    K: Clone + Eq + Hash + PartialEq + Ord,
{
    fn find(&self, key: &K) -> Vec<&R> {
        let mut values = vec![];

        if let Some(record) = self.hashmap.get(key) {
            values.push(record);
        }

        values
    }

    fn find_range(&self, start: &K, end: &K) -> Vec<&R> {
        let start = self.index(start);
        let end = self.index(end);

        (start..end)
            .filter_map(|i| self.hashmap.get(&self.keys[i]))
            .collect()
    }
}
