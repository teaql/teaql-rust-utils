use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct MapTool;

impl MapTool {
    pub fn new() -> Self { Self }

    pub fn merge<K: Eq + Hash + Clone, V: Clone>(&self, a: &HashMap<K, V>, b: &HashMap<K, V>) -> HashMap<K, V> {
        let mut res = a.clone();
        for (k, v) in b {
            res.insert(k.clone(), v.clone());
        }
        res
    }

    pub fn flip<K: Clone, V: Eq + Hash + Clone>(&self, map: &HashMap<K, V>) -> HashMap<V, K> {
        let mut res = HashMap::new();
        for (k, v) in map {
            res.insert(v.clone(), k.clone());
        }
        res
    }
}

impl Default for MapTool {
    fn default() -> Self { Self::new() }
}
