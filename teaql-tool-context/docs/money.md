# TeaQL Tool API: Money Operations

Use `ctx.money()` for safe, precise currency and financial calculations using `rust_decimal::Decimal`.

## Required Dependencies

To use the money utilities, you must add `teaql-tool` to your project with the `std` feature enabled.

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Financial Calculations

```rust
use rust_decimal_macros::dec;

// Create Money instances
let price = ctx.money().of("19.99", "USD")?;
let tax_rate = dec!(0.08);

// Arithmetic
let tax = ctx.money().mul(&price, tax_rate)?;
let total = ctx.money().add(&price, &tax)?;

let discount = ctx.money().of("5.00", "USD")?;
let final_price = ctx.money().sub(&total, &discount)?;

// Allocation without losing pennies
let split = ctx.money().allocate(&final_price, vec![1, 1, 1])?;

// Formatting
let display = ctx.money().format(&final_price);
```

## Key Methods
- `.of(amount_str, currency_code)`: Creates a `Money` instance from a string amount and currency code.
- `.zero(currency_code)`: Creates a zero `Money` instance for a specific currency.
- `.same_currency(a, b)`: Checks if two `Money` instances have the same currency.
- `.format(a)`: Formats the `Money` value as a string.
- `.add(a, b)` / `.sub(a, b)`: Safely adds or subtracts two `Money` instances (requires same currency).
- `.mul(a, multiplier)` / `.div(a, divisor)`: Multiplies or divides a `Money` instance by a `Decimal`.
- `.round(a)`: Rounds the money amount to its currency's standard minor unit.
- `.allocate(a, ratios)`: Distributes a money amount evenly across the given ratios, safely allocating remainder pennies to avoid losing value.
