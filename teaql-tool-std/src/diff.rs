use similar::{ChangeTag, TextDiff};

#[derive(Debug, Clone)]
pub struct DiffTool;

impl DiffTool {
    pub fn new() -> Self { Self }

    /// Returns a textual diff showing differences between old and new text
    pub fn text_diff(&self, old: &str, new: &str) -> String {
        let diff = TextDiff::from_lines(old, new);
        let mut result = String::new();
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            result.push_str(&format!("{}{}", sign, change));
        }
        result
    }
}

impl Default for DiffTool {
    fn default() -> Self { Self::new() }
}
