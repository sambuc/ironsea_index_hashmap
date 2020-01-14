#![allow(clippy::type_repetition_in_bounds)]

use std::collections::HashMap;
use std::hash::Hash;

use ironsea_index::IndexedDestructured;
use ironsea_index::Record;
use ironsea_index::RecordFields;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Index<F, K>
where
    K: Clone + Eq + Hash + PartialEq + Ord,
{
    hashmap: HashMap<K, F>,
    keys: Vec<K>,
}

impl<F, K> Index<F, K>
where
    K: Clone + Eq + Hash + PartialEq + Ord,
{
    pub fn new<I, R>(iter: I) -> Self
    where
        I: Iterator<Item = R>,
        R: Record<K> + RecordFields<F>,
    {
        let (size, _) = iter.size_hint();
        let mut hashmap = HashMap::with_capacity(size);

        for r in iter {
            hashmap.insert(r.key(), r.fields());
        }

        let mut keys = hashmap.keys().cloned().collect::<Vec<_>>();
        keys.sort_unstable();

        Index { hashmap, keys }
    }

    pub fn keys(&self) -> &Vec<K> {
        &self.keys
    }

    pub fn index(&self, key: &K) -> usize {
        match self.keys.binary_search(&key) {
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

impl<F, K> IndexedDestructured<F, K> for Index<F, K>
where
    K: Clone + Eq + Hash + PartialEq + Ord,
{
    fn find(&self, key: &K) -> Vec<&F> {
        let mut values = vec![];

        if let Some(fields) = self.hashmap.get(key) {
            values.push(fields);
        }

        values
    }

    fn find_range(&self, start: &K, end: &K) -> Vec<(K, &F)> {
        let start = self.index(start);
        let end = self.index(end);

        (start..end)
            .filter_map(|i| {
                let key = &self.keys[i];
                if let Some(fields) = self.hashmap.get(key) {
                    Some((key.clone(), fields))
                } else {
                    None
                }
            })
            .collect()
    }
}
