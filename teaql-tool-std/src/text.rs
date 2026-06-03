use convert_case::{Case, Casing};
use teaql_tool_core::MustComment;

pub struct TextTool;

impl TextTool {
    pub fn new() -> Self {
        Self
    }

    pub fn trim(&self, s: &str) -> MustComment<String> {
        MustComment::new(s.trim().to_string())
    }

    pub fn lowercase(&self, s: &str) -> MustComment<String> {
        MustComment::new(s.to_lowercase())
    }

    pub fn uppercase(&self, s: &str) -> MustComment<String> {
        MustComment::new(s.to_uppercase())
    }

    pub fn capitalize(&self, s: &str) -> MustComment<String> {
        let mut c = s.chars();
        MustComment::new(match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        })
    }

    pub fn to_snake_case(&self, s: &str) -> MustComment<String> {
        MustComment::new(s.to_case(Case::Snake))
    }

    pub fn to_camel_case(&self, s: &str) -> MustComment<String> {
        MustComment::new(s.to_case(Case::Camel))
    }

    pub fn to_pascal_case(&self, s: &str) -> MustComment<String> {
        MustComment::new(s.to_case(Case::Pascal))
    }

    pub fn to_kebab_case(&self, s: &str) -> MustComment<String> {
        MustComment::new(s.to_case(Case::Kebab))
    }

    pub fn normalize_whitespace(&self, s: &str) -> MustComment<String> {
        MustComment::new(s.split_whitespace().collect::<Vec<&str>>().join(" "))
    }

    pub fn contains(&self, s: &str, substring: &str) -> MustComment<bool> {
        MustComment::new(s.contains(substring))
    }

    pub fn starts_with(&self, s: &str, prefix: &str) -> MustComment<bool> {
        MustComment::new(s.starts_with(prefix))
    }

    pub fn ends_with(&self, s: &str, suffix: &str) -> MustComment<bool> {
        MustComment::new(s.ends_with(suffix))
    }
}

impl Default for TextTool {
    fn default() -> Self {
        Self::new()
    }
}
