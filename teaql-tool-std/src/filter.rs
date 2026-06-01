use aho_corasick::AhoCorasick;

pub struct FilterTool;

impl FilterTool {
    pub fn new() -> Self {
        Self
    }

    pub fn build_trie<I, P>(&self, patterns: I) -> AhoCorasick
    where
        I: IntoIterator<Item = P>,
        P: AsRef<[u8]>,
    {
        AhoCorasick::new(patterns).unwrap()
    }

    pub fn contains_sensitive(&self, text: &str, trie: &AhoCorasick) -> bool {
        trie.is_match(text)
    }

    pub fn replace_sensitive(&self, text: &str, trie: &AhoCorasick, replacement: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut last_match = 0;
        for mat in trie.find_iter(text) {
            result.push_str(&text[last_match..mat.start()]);
            result.push_str(replacement);
            last_match = mat.end();
        }
        result.push_str(&text[last_match..]);
        result
    }
}

impl Default for FilterTool {
    fn default() -> Self {
        Self::new()
    }
}
