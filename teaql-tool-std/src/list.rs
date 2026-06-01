use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct ListTool;

impl ListTool {
    pub fn new() -> Self { Self }

    pub fn chunk<T: Clone>(&self, list: &[T], size: usize) -> Vec<Vec<T>> {
        if size == 0 { return vec![list.to_vec()]; }
        list.chunks(size).map(|c| c.to_vec()).collect()
    }

    pub fn unique<T: Clone + Eq + Hash>(&self, list: &[T]) -> Vec<T> {
        let mut seen = HashSet::new();
        list.iter().filter(|x| seen.insert((*x).clone())).cloned().collect()
    }

    pub fn intersect<T: Clone + Eq + Hash>(&self, a: &[T], b: &[T]) -> Vec<T> {
        let set_b: HashSet<_> = b.iter().collect();
        self.unique(a).into_iter().filter(|x| set_b.contains(x)).collect()
    }

    pub fn union<T: Clone + Eq + Hash>(&self, a: &[T], b: &[T]) -> Vec<T> {
        let mut res = a.to_vec();
        res.extend(b.iter().cloned());
        self.unique(&res)
    }

    pub fn difference<T: Clone + Eq + Hash>(&self, a: &[T], b: &[T]) -> Vec<T> {
        let set_b: HashSet<_> = b.iter().collect();
        self.unique(a).into_iter().filter(|x| !set_b.contains(x)).collect()
    }
}

impl Default for ListTool {
    fn default() -> Self { Self::new() }
}
