use crate::macros::*;

use teaql_tool_std::daterange::{DateRangeTool, DateRange};

define_context_facade!("std", daterange, ContextDateRangeExt, ContextDateRangeFacade);

#[cfg(feature = "std")]
impl<'a> ContextDateRangeFacade<'a> {
    delegate_comment! { DateRangeTool::new(),
        fn today(&self) -> DateRange;
        fn yesterday(&self) -> DateRange;
        fn tomorrow(&self) -> DateRange;
        fn offset_day(&self, n: i64) -> DateRange;
        fn last_n_days(&self, n: i64) -> Option<DateRange>;
        fn next_n_days(&self, n: i64) -> Option<DateRange>;
        fn this_hour(&self) -> DateRange;
        fn last_hour(&self) -> DateRange;
        fn next_hour(&self) -> DateRange;
        fn offset_hour(&self, n: i64) -> DateRange;
        fn last_n_hours(&self, n: i64) -> Option<DateRange>;
        fn next_n_hours(&self, n: i64) -> Option<DateRange>
    }
}
