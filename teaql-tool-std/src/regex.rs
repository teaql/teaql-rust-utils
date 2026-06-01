use ::regex::Regex;
use teaql_tool_core::{Result, TeaQLToolError};

pub struct RegexTool;

impl RegexTool {
    pub fn new() -> Self {
        Self
    }

    fn compile(&self, pattern: &str) -> Result<Regex> {
        Regex::new(pattern)
            .map_err(|e| TeaQLToolError::InvalidArgument(format!("Invalid regex: {}", e)))
    }

    pub fn is_match(&self, pattern: &str, text: &str) -> Result<bool> {
        let re = self.compile(pattern)?;
        Ok(re.is_match(text))
    }

    pub fn find(&self, pattern: &str, text: &str) -> Result<Option<String>> {
        let re = self.compile(pattern)?;
        Ok(re.find(text).map(|m| m.as_str().to_string()))
    }

    pub fn find_all(&self, pattern: &str, text: &str) -> Result<Vec<String>> {
        let re = self.compile(pattern)?;
        Ok(re.find_iter(text).map(|m| m.as_str().to_string()).collect())
    }

    pub fn replace(&self, pattern: &str, text: &str, rep: &str) -> Result<String> {
        let re = self.compile(pattern)?;
        Ok(re.replace(text, rep).into_owned())
    }

    pub fn replace_all(&self, pattern: &str, text: &str, rep: &str) -> Result<String> {
        let re = self.compile(pattern)?;
        Ok(re.replace_all(text, rep).into_owned())
    }

    pub fn split(&self, pattern: &str, text: &str) -> Result<Vec<String>> {
        let re = self.compile(pattern)?;
        Ok(re.split(text).map(|s| s.to_string()).collect())
    }

    pub fn escape(&self, text: &str) -> String {
        ::regex::escape(text)
    }

    pub fn validate(&self, pattern: &str) -> bool {
        Regex::new(pattern).is_ok()
    }
}

impl Default for RegexTool {
    fn default() -> Self {
        Self::new()
    }
}
