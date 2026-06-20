use convert_case::{Case, Casing};

pub struct TextTool;

impl TextTool {
    pub fn new() -> Self {
        Self
    }

    pub fn trim(&self, s: &str) -> String {
        s.trim().to_string()
    }

    pub fn lowercase(&self, s: &str) -> String {
        s.to_lowercase()
    }

    pub fn uppercase(&self, s: &str) -> String {
        s.to_uppercase()
    }

    pub fn capitalize(&self, s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    pub fn to_snake_case(&self, s: &str) -> String {
        s.to_case(Case::Snake)
    }

    pub fn to_camel_case(&self, s: &str) -> String {
        s.to_case(Case::Camel)
    }

    pub fn to_pascal_case(&self, s: &str) -> String {
        s.to_case(Case::Pascal)
    }

    pub fn to_kebab_case(&self, s: &str) -> String {
        s.to_case(Case::Kebab)
    }

    pub fn normalize_whitespace(&self, s: &str) -> String {
        s.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    pub fn contains(&self, s: &str, substring: &str) -> bool {
        s.contains(substring)
    }

    pub fn starts_with(&self, s: &str, prefix: &str) -> bool {
        s.starts_with(prefix)
    }

    pub fn ends_with(&self, s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }
}

impl Default for TextTool {
    fn default() -> Self {
        Self::new()
    }
}
