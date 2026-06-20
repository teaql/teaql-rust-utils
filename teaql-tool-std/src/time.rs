use chrono::{DateTime, Datelike, Duration, Months, NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;
use std::str::FromStr;
use teaql_tool_core::{ Result, TeaQLToolError};

pub struct TimeTool;

impl TimeTool {
    pub fn new() -> Self {
        Self
    }

    pub fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    pub fn today(&self) -> NaiveDate {
        Utc::now().date_naive()
    }

    pub fn parse_date(&self, s: &str) -> Result<NaiveDate> {
        NaiveDate::from_str(s)
            
            .map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn parse_datetime(&self, s: &str) -> Result<DateTime<Utc>> {
        s.parse::<DateTime<Utc>>()
            
            .map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn add_days(&self, dt: DateTime<Utc>, days: i64) -> DateTime<Utc> {
        dt + Duration::days(days)
    }

    pub fn add_months(&self, dt: DateTime<Utc>, months: u32) -> DateTime<Utc> {
        dt.checked_add_months(Months::new(months)).unwrap_or(dt)
    }

    pub fn start_of_day(&self, dt: DateTime<Utc>) -> DateTime<Utc> {
        
            Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 0, 0, 0)
                .unwrap()
        
    }

    pub fn end_of_day(&self, dt: DateTime<Utc>) -> DateTime<Utc> {
        self.start_of_day(dt) + Duration::days(1) - Duration::nanoseconds(1)
    }

    pub fn days_between(&self, dt1: DateTime<Utc>, dt2: DateTime<Utc>) -> i64 {
        let dur = dt2.signed_duration_since(dt1);
        dur.num_days()
    }

    pub fn to_timezone(&self, dt: DateTime<Utc>, tz_str: &str) -> Result<DateTime<Tz>> {
        let tz: Tz = tz_str
            .parse()
            .map_err(|e| TeaQLToolError::InvalidArgument(format!("{}", e)))?;
        Ok(dt.with_timezone(&tz))
    }
}

impl Default for TimeTool {
    fn default() -> Self {
        Self::new()
    }
}
