use std::collections::HashMap;
use std::hash::Hash;
use std::marker;

use ironsea_index::Indexed;
use ironsea_index::Record;
use ironsea_table::Table;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Index<T, R, K>
where
    T: Table<R>,
    R: Record<K>,
    K: Hash + Eq + PartialEq + Ord,
{
    table: HashMap<K, R>,
    keys: Vec<K>,
    _marker: marker::PhantomData<(T)>,
}

impl<T, R, K> Index<T, R, K>
where
    T: Table<R>,
    R: Clone + Record<K>,
    K: Hash + Eq + PartialEq + Ord,
{
    pub fn new(table: T) -> Self {
        let size = table.get_table().len();
        let mut ht = HashMap::with_capacity(size);
        let mut keys = Vec::with_capacity(size);

        for r in table.get_table() {
            keys.push(r.key());
            ht.insert(r.key(), r.clone());
        }

        keys.sort_unstable();
        keys.dedup();

        Index {
            table: ht,
            keys,
            _marker: marker::PhantomData,
        }
    }

    pub fn keys(&self) -> &Vec<K> {
        &self.keys
    }
}

impl<T, R, K> Indexed<T, R, K> for Index<T, R, K>
where
    T: Table<R>,
    R: Record<K>,
    K: Hash + Eq + PartialEq + Ord,
{
    fn find(&self, key: &K) -> Vec<&R> {
        let mut values = vec![];

        if let Some(record) = self.table.get(key) {
            values.push(record);
        }

        values
    }

    fn find_range(&self, start: &K, end: &K) -> Vec<&R> {
        let start = match self.keys.binary_search(start) {
            Ok(i) => i,
            Err(i) => {
                if i >= self.keys.len() {
                    self.keys.len() - 1
                } else {
                    i
                }
            }
        };

        let end = match self.keys.binary_search(end) {
            Ok(i) => i,
            Err(i) => {
                if i >= self.keys.len() {
                    self.keys.len() - 1
                } else {
                    i
                }
            }
        };

        let mut values = vec![];

        for i in start..end {
            let key = &self.keys[i];
            if let Some(record) = self.table.get(key) {
                values.push(record);
            }
        }

        values
    }
}
