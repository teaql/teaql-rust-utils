use teaql_tool_core::{Result, TeaQLToolError};
use notify::{Watcher, RecursiveMode};
use std::sync::mpsc::channel;

#[derive(Debug, Clone)]
pub struct WatcherTool;

impl WatcherTool {
    pub fn new() -> Self { Self }

    pub fn watch<F>(&self, path: &str, callback: F) -> Result<()> 
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(tx).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        
        watcher.watch(std::path::Path::new(path), RecursiveMode::Recursive)
            .map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
            
        println!("Watching path: {}", path);
        
        for res in rx {
            match res {
                Ok(event) => {
                    for p in event.paths {
                        callback(&p.to_string_lossy());
                    }
                },
                Err(e) => println!("Watch error: {:?}", e),
            }
        }
        
        Ok(())
    }
}

impl Default for WatcherTool {
    fn default() -> Self { Self::new() }
}
