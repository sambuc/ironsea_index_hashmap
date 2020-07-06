#![allow(clippy::type_repetition_in_bounds)]

use std::collections::HashMap;
use std::hash::Hash;

use ironsea_index::IndexedDestructured;
use ironsea_index::Record;
use ironsea_index::RecordFields;
use serde::Deserialize;
use serde::Serialize;

/// Implementation of [`ironsea_index`]::[`IndexedDestructured`].
///
/// The index is backed by a [`std`]`::`[`collections`]`::`[`HashMap`].
///
/// An ordered [`std`]`::`[`vec`]`::`[`Vec`] of keys is maintained, in
/// order to satisfy range queries.
///
/// [`ironsea_index`]: https://epfl-dias.github.io/ironsea_index/ironsea_index/index.html
/// [`IndexedDestructured`]: https://epfl-dias.github.io/ironsea_index/ironsea_index/trait.IndexedDestructured.html
///
/// [`std`]: https://doc.rust-lang.org/std/index.html
/// [`collections`]: https://doc.rust-lang.org/std/collections/index.html
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
/// [`vec`]: https://doc.rust-lang.org/std/vec/index.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html

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
    /// Creates a new Index from the provided iterator.
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

    /// Return an ordered list of all keys contained in the index.
    pub fn keys(&self) -> &Vec<K> {
        &self.keys
    }

    /// Returns the position within the index of the key.
    ///
    /// If the key is not found, return the index where it should be
    /// inserted.
    fn index(&self, key: &K) -> usize {
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
    fn find<'i>(&'i self, key: &K) -> Box<dyn Iterator<Item = &F> + 'i> {
        let mut values = vec![];

        if let Some(fields) = self.hashmap.get(key) {
            values.push(fields);
        }

        Box::new(values.into_iter())
    }

    fn find_range<'i>(&'i self, start: &K, end: &K) -> Box<dyn Iterator<Item = (K, &F)> + 'i> {
        let start = self.index(start);
        let end = self.index(end);

        Box::new((start..=end).filter_map(move |i| {
            let key = &self.keys[i];
            if let Some(fields) = self.hashmap.get(key) {
                Some((key.clone(), fields))
            } else {
                None
            }
        }))
    }
}
