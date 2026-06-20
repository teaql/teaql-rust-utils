# TeaQL Tool API: Time Utilities

Use `ctx.time()` to perform time and date calculations, timezone conversions, and formatting. This tool is built into the TeaQL runtime for convenient time manipulation.

## Required Dependencies

To use the time utilities and other standard features, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Example Usage

```rust
let current_time = ctx.time().now();
let today = ctx.time().today();

let next_week = ctx.time().add_days(current_time, 7);
let end_of_today = ctx.time().end_of_day(current_time);

let parsed_date = ctx.time().parse_date("2023-10-01")?;
```

## Key Methods
- `.now()`: Get the current UTC date and time.
- `.today()`: Get the current date.
- `.add_days(dt, days)`: Add or subtract days from a given date and time.
- `.add_months(dt, months)`: Add months to a given date and time.
- `.start_of_day(dt)` / `.end_of_day(dt)`: Get the beginning or end of the day for a given date and time.
- `.days_between(dt1, dt2)`: Calculate the number of days between two dates.
- `.parse_date(s)`: Parse a string into a date.
- `.parse_datetime(s)`: Parse a string into a UTC date and time.
- `.to_timezone(dt, tz_str)`: Convert a UTC date and time to a specific timezone.
