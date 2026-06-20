use crate::macros::*;

use teaql_tool_std::time::TimeTool;
use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;
use teaql_tool_core::{MustPurpose, Result};

define_context_facade!("std", time, ContextTimeExt, ContextTimeFacade);

#[cfg(feature = "std")]
impl<'a> ContextTimeFacade<'a> {
    delegate_comment! { TimeTool::new(),
        fn now(&self) -> DateTime<Utc>;
        fn today(&self) -> NaiveDate;
        fn add_days(&self, dt: DateTime<Utc>, days: i64) -> DateTime<Utc>;
        fn add_months(&self, dt: DateTime<Utc>, months: u32) -> DateTime<Utc>;
        fn start_of_day(&self, dt: DateTime<Utc>) -> DateTime<Utc>;
        fn end_of_day(&self, dt: DateTime<Utc>) -> DateTime<Utc>;
        fn days_between(&self, dt1: DateTime<Utc>, dt2: DateTime<Utc>) -> i64
    }
    delegate_res_comment! { TimeTool::new(),
        fn parse_date(&self, s: &str) -> NaiveDate;
        fn parse_datetime(&self, s: &str) -> DateTime<Utc>;
        fn to_timezone(&self, dt: DateTime<Utc>, tz_str: &str) -> DateTime<Tz>
    }
}
