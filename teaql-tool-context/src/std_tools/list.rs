use crate::macros::define_context_facade;

use teaql_tool_std::list::ListTool;
use std::hash::Hash;

define_context_facade!("std", list, ContextListExt, ContextListFacade);

// All ListTool methods have generic type parameters, so they must be manually implemented.

#[cfg(feature = "std")]
impl<'a> ContextListFacade<'a> {
    pub fn chunk<T: Clone>(&self, list: &[T], size: usize) -> Vec<Vec<T>> {
        ListTool::new().chunk(list, size)
    }

    pub fn unique<T: Clone + Eq + Hash>(&self, list: &[T]) -> Vec<T> {
        ListTool::new().unique(list)
    }

    pub fn intersect<T: Clone + Eq + Hash>(&self, a: &[T], b: &[T]) -> Vec<T> {
        ListTool::new().intersect(a, b)
    }

    pub fn union<T: Clone + Eq + Hash>(&self, a: &[T], b: &[T]) -> Vec<T> {
        ListTool::new().union(a, b)
    }

    pub fn difference<T: Clone + Eq + Hash>(&self, a: &[T], b: &[T]) -> Vec<T> {
        ListTool::new().difference(a, b)
    }
}
