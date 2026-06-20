use crate::macros::define_context_facade;

use teaql_tool_std::emoji::EmojiTool;

define_context_facade!("std", emoji, ContextEmojiExt, ContextEmojiFacade);

#[cfg(feature = "std")]
impl<'a> ContextEmojiFacade<'a> {
    /// Check if a string contains any emoji characters
    pub fn contains_emoji(&self, text: impl AsRef<str>) -> bool {
        EmojiTool::new().contains_emoji(text)
    }

    /// Remove all emoji characters from a string
    pub fn remove_all(&self, text: impl AsRef<str>) -> String {
        EmojiTool::new().remove_all(text)
    }

    /// Replace all emoji characters in a string with a replacement string
    pub fn replace_all(&self, text: impl AsRef<str>, replacement: impl AsRef<str>) -> String {
        EmojiTool::new().replace_all(text, replacement)
    }
}
