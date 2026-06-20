# TeaQL Tool API: Date Range Utilities

Use `ctx.daterange()` to easily calculate common date and time ranges for queries and filtering.

## Required Dependencies

To use the date range utilities, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Basic Usage

```rust
// Get ranges for specific days
let today_range = ctx.daterange().today();
let yesterday_range = ctx.daterange().yesterday();

// Get range for the last 7 days
if let Some(last_week) = ctx.daterange().last_n_days(7) {
    // use the date range
}
```

## Key Methods
- `.today()` / `.yesterday()` / `.tomorrow()`: Get a `DateRange` for a specific day.
- `.offset_day(n)`: Get a `DateRange` offset by `n` days from today.
- `.last_n_days(n)` / `.next_n_days(n)`: Get a `DateRange` spanning multiple days (returns `Option<DateRange>`).
- `.this_hour()` / `.last_hour()` / `.next_hour()`: Get a `DateRange` for a specific hour.
- `.offset_hour(n)`: Get a `DateRange` offset by `n` hours from the current hour.
- `.last_n_hours(n)` / `.next_n_hours(n)`: Get a `DateRange` spanning multiple hours (returns `Option<DateRange>`).
