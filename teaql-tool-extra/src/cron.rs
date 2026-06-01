use teaql_tool_core::{Result, TeaQLToolError};
use cron::Schedule;
use std::str::FromStr;
use chrono::Utc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct CronTool;

impl CronTool {
    pub fn new() -> Self { Self }

    /// Blocks and runs the callback indefinitely
    pub fn schedule<F>(&self, cron_expr: &str, mut callback: F) -> Result<()> 
    where
        F: FnMut() + Send + 'static,
    {
        let schedule = Schedule::from_str(cron_expr).map_err(|e| TeaQLToolError::ParseError(e.to_string()))?;
        for datetime in schedule.upcoming(Utc) {
            let now = Utc::now();
            if let Ok(duration) = (datetime - now).to_std() {
                std::thread::sleep(duration);
                callback();
            }
        }
        Ok(())
    }
}

impl Default for CronTool {
    fn default() -> Self { Self::new() }
}
