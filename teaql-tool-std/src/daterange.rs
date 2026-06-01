use chrono::{DateTime, Duration, Local, NaiveTime, Timelike};

#[derive(Debug, Clone, PartialEq)]
pub struct DateRange {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

impl DateRange {
    pub fn new(start: DateTime<Local>, end: DateTime<Local>) -> Self {
        Self { start, end }
    }
}

pub struct DateRangeTool;

impl DateRangeTool {
    pub fn new() -> Self {
        Self
    }

    /// Helper to get the start of the day
    fn begin_of_day(&self, dt: DateTime<Local>) -> DateTime<Local> {
        dt.with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()).unwrap()
    }

    /// Helper to get the end of the day
    fn end_of_day(&self, dt: DateTime<Local>) -> DateTime<Local> {
        dt.with_time(NaiveTime::from_hms_nano_opt(23, 59, 59, 999_999_999).unwrap()).unwrap()
    }

    pub fn today(&self) -> DateRange {
        self.offset_day(0)
    }

    pub fn yesterday(&self) -> DateRange {
        self.offset_day(-1)
    }

    pub fn tomorrow(&self) -> DateRange {
        self.offset_day(1)
    }

    pub fn offset_day(&self, n: i64) -> DateRange {
        let the_day = Local::now() + Duration::days(n);
        DateRange::new(self.begin_of_day(the_day), self.end_of_day(the_day))
    }

    pub fn last_n_days(&self, n: i64) -> Option<DateRange> {
        if n <= 0 {
            return None;
        }
        let yesterday = Local::now() - Duration::days(1);
        let start_day = yesterday - Duration::days(n - 1);
        Some(DateRange::new(self.begin_of_day(start_day), self.end_of_day(yesterday)))
    }

    pub fn next_n_days(&self, n: i64) -> Option<DateRange> {
        if n <= 0 {
            return None;
        }
        let tomorrow = Local::now() + Duration::days(1);
        let end_day = tomorrow + Duration::days(n - 1);
        Some(DateRange::new(self.begin_of_day(tomorrow), self.end_of_day(end_day)))
    }

    /// Helper to get the start of the hour
    fn begin_of_hour(&self, dt: DateTime<Local>) -> DateTime<Local> {
        dt.with_time(NaiveTime::from_hms_opt(dt.time().hour(), 0, 0).unwrap()).unwrap()
    }

    /// Helper to get the end of the hour
    fn end_of_hour(&self, dt: DateTime<Local>) -> DateTime<Local> {
        dt.with_time(NaiveTime::from_hms_nano_opt(dt.time().hour(), 59, 59, 999_999_999).unwrap()).unwrap()
    }

    pub fn this_hour(&self) -> DateRange {
        self.offset_hour(0)
    }

    pub fn last_hour(&self) -> DateRange {
        self.offset_hour(-1)
    }

    pub fn next_hour(&self) -> DateRange {
        self.offset_hour(1)
    }

    pub fn offset_hour(&self, n: i64) -> DateRange {
        let the_hour = Local::now() + Duration::hours(n);
        DateRange::new(self.begin_of_hour(the_hour), self.end_of_hour(the_hour))
    }

    pub fn last_n_hours(&self, n: i64) -> Option<DateRange> {
        if n <= 0 {
            return None;
        }
        let last_hr = Local::now() - Duration::hours(1);
        let start_hr = last_hr - Duration::hours(n - 1);
        Some(DateRange::new(self.begin_of_hour(start_hr), self.end_of_hour(last_hr)))
    }

    pub fn next_n_hours(&self, n: i64) -> Option<DateRange> {
        if n <= 0 {
            return None;
        }
        let next_hr = Local::now() + Duration::hours(1);
        let end_hr = next_hr + Duration::hours(n - 1);
        Some(DateRange::new(self.begin_of_hour(next_hr), self.end_of_hour(end_hr)))
    }
}

impl Default for DateRangeTool {
    fn default() -> Self {
        Self::new()
    }
}
