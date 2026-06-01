use email_address::EmailAddress;
use url::Url;
use std::str::FromStr;

pub struct ValidateTool;

impl ValidateTool {
    pub fn new() -> Self {
        Self
    }

    pub fn email(&self, val: &str) -> bool {
        EmailAddress::is_valid(val)
    }

    pub fn url(&self, val: &str) -> bool {
        Url::from_str(val).is_ok()
    }

    pub fn min_length(&self, val: &str, min: usize) -> bool {
        val.chars().count() >= min
    }

    pub fn max_length(&self, val: &str, max: usize) -> bool {
        val.chars().count() <= max
    }
}

impl Default for ValidateTool {
    fn default() -> Self {
        Self::new()
    }
}
