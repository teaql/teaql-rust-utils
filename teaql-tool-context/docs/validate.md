# TeaQL Tool API: Validate Utilities

Use `ctx.validate()` to perform common validations, such as checking if strings are valid emails, URLs, or if they meet specific length constraints.

## Required Dependencies

To use the validation utilities and other standard features, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Example Usage

```rust
let is_valid_email = ctx.validate().email("user@example.com");
let is_valid_url = ctx.validate().url("https://example.com");

let meets_min_length = ctx.validate().min_length("password123", 8);
let meets_max_length = ctx.validate().max_length("short", 10);

if !is_valid_email {
    // Handle invalid email
}
```

## Key Methods
- `.email(val)`: Check if a string is a valid email address.
- `.url(val)`: Check if a string is a valid URL.
- `.min_length(val, min)`: Check if a string's length is at least the specified minimum.
- `.max_length(val, max)`: Check if a string's length is no more than the specified maximum.
