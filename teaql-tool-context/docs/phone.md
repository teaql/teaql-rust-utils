# TeaQL Tool API: Phone Utilities

Use `ctx.phone()` to validate, parse, and format phone numbers.

## Required Dependencies

To use the Phone utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
let number = "+14155552671";

// Check if a phone number is valid
let is_valid = ctx.phone().is_valid(number);
println!("Is valid: {}", is_valid);

// Parse a phone number
let parsed = ctx.phone().parse(number)?;
println!("Country code: {}", parsed.code().value());

// Format a phone number internationally
let formatted = ctx.phone().format_international(number)?;
println!("Formatted: {}", formatted);
```

## Key Methods
- `.is_valid(number)`: Check if the provided phone number is valid.
- `.parse(number)`: Parse a phone number string into a `phonenumber::PhoneNumber` object.
- `.format_international(number)`: Format a phone number into its international representation.
