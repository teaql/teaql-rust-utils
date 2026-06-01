pub struct EmojiTool;

impl EmojiTool {
    pub fn new() -> Self {
        Self
    }

    /// Check if a string contains any emoji characters
    pub fn contains_emoji(&self, text: impl AsRef<str>) -> bool {
        let text = text.as_ref();
        text.chars().any(|c| self.is_emoji_char(c))
    }

    /// Remove all emoji characters from a string
    pub fn remove_all(&self, text: impl AsRef<str>) -> String {
        let text = text.as_ref();
        text.chars().filter(|c| !self.is_emoji_char(*c)).collect()
    }

    /// Replace all emoji characters in a string with a replacement string
    pub fn replace_all(&self, text: impl AsRef<str>, replacement: impl AsRef<str>) -> String {
        let text = text.as_ref();
        let rep = replacement.as_ref();
        let mut result = String::with_capacity(text.len());
        for c in text.chars() {
            if self.is_emoji_char(c) {
                result.push_str(rep);
            } else {
                result.push(c);
            }
        }
        result
    }

    /// Internal logic to detect common Emoji Unicode blocks
    fn is_emoji_char(&self, c: char) -> bool {
        let u = c as u32;
        // Common emoji blocks
        (0x1F300..=0x1F5FF).contains(&u) || // Miscellaneous Symbols and Pictographs
        (0x1F900..=0x1F9FF).contains(&u) || // Supplemental Symbols and Pictographs
        (0x1F600..=0x1F64F).contains(&u) || // Emoticons
        (0x1F680..=0x1F6FF).contains(&u) || // Transport and Map
        (0x2600..=0x26FF).contains(&u)   || // Miscellaneous Symbols
        (0x2700..=0x27BF).contains(&u)   || // Dingbats
        (0xFE00..=0xFE0F).contains(&u)   || // Variation Selectors
        (0x1F1E6..=0x1F1FF).contains(&u) || // Regional indicator symbol
        (0x1F018..=0x1F270).contains(&u) || // Various enclosed alphanumerics
        (0x1F700..=0x1F77F).contains(&u) || // Alchemical Symbols
        (0x1F780..=0x1F7FF).contains(&u) || // Geometric Shapes Extended
        (0x1F800..=0x1F8FF).contains(&u) || // Supplemental Arrows-C
        (0x1FA70..=0x1FAFF).contains(&u)    // Symbols and Pictographs Extended-A
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_tool() {
        let tool = EmojiTool::new();
        assert!(tool.contains_emoji("Hello 😀!"));
        assert!(!tool.contains_emoji("Hello World!"));
        
        let cleaned = tool.remove_all("Bad 💩 Data 🤡!");
        assert_eq!(cleaned, "Bad  Data !");
    }
}
