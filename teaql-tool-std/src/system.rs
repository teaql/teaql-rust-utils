use std::env;
use std::env::consts;

pub struct SystemTool;

impl SystemTool {
    pub fn new() -> Self {
        Self
    }

    pub fn env(&self, key: &str) -> Option<String> {
        env::var(key).ok()
    }

    pub fn env_or(&self, key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    pub fn os(&self) -> &'static str {
        consts::OS
    }
    
    pub fn arch(&self) -> &'static str {
        consts::ARCH
    }
    
    pub fn current_dir(&self) -> Option<String> {
        env::current_dir().ok().map(|p| p.to_string_lossy().into_owned())
    }
}

impl Default for SystemTool {
    fn default() -> Self {
        Self::new()
    }
}
