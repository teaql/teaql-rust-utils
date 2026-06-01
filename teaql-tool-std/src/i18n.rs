use serde_json::Value;
use std::collections::HashMap;
use teaql_tool_core::{Result, TeaQLToolError};

#[derive(Debug, Clone)]
pub struct I18nTool {
    translations: HashMap<String, HashMap<String, String>>,
    default_locale: String,
}

impl I18nTool {
    pub fn new() -> Self {
        Self {
            translations: HashMap::new(),
            default_locale: "en".to_string(),
        }
    }

    pub fn set_default_locale(&mut self, locale: &str) {
        self.default_locale = locale.to_string();
    }

    pub fn add(&mut self, locale: &str, key: &str, value: &str) {
        self.translations
            .entry(locale.to_string())
            .or_default()
            .insert(key.to_string(), value.to_string());
    }

    /// Loads flat or nested JSON into the specified locale.
    /// Nested objects will have their keys joined by dots (e.g. "greeting.hello").
    pub fn load_json(&mut self, locale: &str, json_str: &str) -> Result<()> {
        let v: Value = serde_json::from_str(json_str)
            .map_err(|e| TeaQLToolError::ParseError(format!("Invalid JSON for i18n: {}", e)))?;
        
        self.flatten_and_add(locale, "", &v);
        Ok(())
    }

    fn flatten_and_add(&mut self, locale: &str, prefix: &str, value: &Value) {
        match value {
            Value::Object(map) => {
                for (k, v) in map {
                    let new_prefix = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    self.flatten_and_add(locale, &new_prefix, v);
                }
            }
            Value::String(s) => {
                self.add(locale, prefix, s);
            }
            Value::Number(n) => {
                self.add(locale, prefix, &n.to_string());
            }
            Value::Bool(b) => {
                self.add(locale, prefix, &b.to_string());
            }
            _ => {}
        }
    }

    /// Translates a key for a given locale.
    /// Falls back to the default locale if not found, or returns the key itself.
    pub fn t(&self, locale: &str, key: &str) -> String {
        if let Some(map) = self.translations.get(locale) {
            if let Some(val) = map.get(key) {
                return val.clone();
            }
        }
        
        if locale != self.default_locale {
            if let Some(map) = self.translations.get(&self.default_locale) {
                if let Some(val) = map.get(key) {
                    return val.clone();
                }
            }
        }
        
        key.to_string()
    }

    /// Translates a key and replaces placeholders like {name} with provided arguments.
    pub fn tf(&self, locale: &str, key: &str, args: &[(&str, &str)]) -> String {
        let mut text = self.t(locale, key);
        for (k, v) in args {
            let placeholder = format!("{{{}}}", k);
            text = text.replace(&placeholder, v);
        }
        text
    }
}

impl Default for I18nTool {
    fn default() -> Self {
        Self::new()
    }
}
