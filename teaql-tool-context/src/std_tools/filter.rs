use crate::macros::*;

use teaql_tool_std::filter::FilterTool;
use aho_corasick::AhoCorasick;

define_context_facade!("std", filter, ContextFilterExt, ContextFilterFacade);

#[cfg(feature = "std")]
impl<'a> ContextFilterFacade<'a> {
    delegate_comment! { FilterTool::new(),
        fn contains_sensitive(&self, text: &str, trie: &AhoCorasick) -> bool;
        fn replace_sensitive(&self, text: &str, trie: &AhoCorasick, replacement: &str) -> String
    }

    pub fn build_trie<I, P>(&self, patterns: I) -> AhoCorasick
    where
        I: IntoIterator<Item = P>,
        P: AsRef<[u8]>,
    {
        FilterTool::new().build_trie(patterns)
    }
}
