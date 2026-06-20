# TeaQL Tool API: Decimal Utilities

Use `ctx.decimal()` to perform precise financial and mathematical operations without floating-point errors.

## Required Dependencies

To use the decimal utilities, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Basic Usage

```rust
// Parse strings into Decimals
let price = ctx.decimal().of("19.99")?;
let tax_rate = ctx.decimal().of("0.05")?;

// Perform exact calculations
let tax = ctx.decimal().mul(price, tax_rate);
let total = ctx.decimal().add(price, tax);

// Rounding
let rounded_total = ctx.decimal().round(total, 2);
```

## Key Methods
- `.of(string)`: Parse a string into a `Decimal`. Returns a `Result`.
- `.zero()` / `.one()`: Get standard values.
- `.add(a, b)` / `.sub(a, b)` / `.mul(a, b)` / `.div(a, b)`: Perform basic arithmetic. Note that `.div` returns a `Result`.
- `.round(a, dp)` / `.ceil(a)` / `.floor(a)`: Rounding operations.
- `.abs(a)` / `.min(a, b)` / `.max(a, b)`: Standard mathematical functions.
- `.percent(amount, pct)`: Calculate a percentage of an amount.
- `.ratio(part, total)`: Calculate the ratio between two values. Returns a `Result`.
