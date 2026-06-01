use moka::sync::Cache;
use std::time::Duration;

pub struct CacheTool {
    string_cache: Cache<String, String>,
}

impl CacheTool {
    pub fn new() -> Self {
        let string_cache = Cache::builder()
            .time_to_live(Duration::from_secs(60 * 60)) // 1 hour default
            .build();
        Self { string_cache }
    }

    pub fn put(&self, key: &str, value: &str) {
        self.string_cache.insert(key.to_string(), value.to_string());
    }
    
    pub fn get(&self, key: &str) -> Option<String> {
        self.string_cache.get(key)
    }
}

impl Default for CacheTool {
    fn default() -> Self {
        Self::new()
    }
}
