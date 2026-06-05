use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
use teaql_tool_std::time::TimeTool;

#[test]
fn test_time_operations() {
    let tool = TimeTool::new();

    // current time
    let now = tool.now().purpose("test");
    assert!(now.timestamp() > 0);

    // today
    let today = tool.today().purpose("test");
    assert!(today.year() > 2000);

    // parse date
    let parsed_date = tool.parse_date("2023-10-15").unwrap().purpose("test");
    assert_eq!(parsed_date, NaiveDate::from_ymd_opt(2023, 10, 15).unwrap());

    // parse datetime
    let parsed_dt = tool.parse_datetime("2023-10-15T10:30:00Z").unwrap().purpose("test");
    assert_eq!(
        parsed_dt,
        Utc.with_ymd_and_hms(2023, 10, 15, 10, 30, 0).unwrap()
    );

    // add days
    let dt = Utc.with_ymd_and_hms(2023, 10, 15, 10, 30, 0).unwrap();
    let dt_plus_2 = tool.add_days(dt, 2).purpose("test");
    assert_eq!(
        dt_plus_2,
        Utc.with_ymd_and_hms(2023, 10, 17, 10, 30, 0).unwrap()
    );

    // add months
    let dt_plus_1_month = tool.add_months(dt, 1).purpose("test");
    assert_eq!(
        dt_plus_1_month,
        Utc.with_ymd_and_hms(2023, 11, 15, 10, 30, 0).unwrap()
    );

    // boundaries
    let start = tool.start_of_day(dt).purpose("test");
    assert_eq!(start, Utc.with_ymd_and_hms(2023, 10, 15, 0, 0, 0).unwrap());

    let end = tool.end_of_day(dt).purpose("test");
    assert_eq!(
        end,
        Utc.with_ymd_and_hms(2023, 10, 15, 23, 59, 59).unwrap()
            + chrono::Duration::nanoseconds(999_999_999)
    );

    // days between
    let dt2 = Utc.with_ymd_and_hms(2023, 10, 20, 10, 30, 0).unwrap();
    assert_eq!(tool.days_between(dt, dt2).purpose("test"), 5);

    // timezone
    let dt_tz = tool.to_timezone(dt, "Asia/Shanghai").unwrap().purpose("test");
    assert_eq!(dt_tz.to_string(), "2023-10-15 18:30:00 CST");
}
