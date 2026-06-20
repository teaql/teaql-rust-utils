use crate::macros::define_context_facade;

use teaql_tool_std::map::MapTool;
use std::collections::HashMap;
use std::hash::Hash;

define_context_facade!("std", map, ContextMapExt, ContextMapFacade);

// All MapTool methods have generic type parameters, so they must be manually implemented.

#[cfg(feature = "std")]
impl<'a> ContextMapFacade<'a> {
    pub fn merge<K: Eq + Hash + Clone, V: Clone>(&self, a: &HashMap<K, V>, b: &HashMap<K, V>) -> HashMap<K, V> {
        MapTool::new().merge(a, b)
    }

    pub fn flip<K: Clone, V: Eq + Hash + Clone>(&self, map: &HashMap<K, V>) -> HashMap<V, K> {
        MapTool::new().flip(map)
    }
}
