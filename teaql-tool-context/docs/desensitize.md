# TeaQL Tool API: Desensitize Utilities

Use `ctx.desensitize()` to mask sensitive personal information (PII) before logging, storing, or returning it in API responses.

## Required Dependencies

To use the desensitize utilities, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Basic Usage

```rust
let email = "user@example.com";
let masked_email = ctx.desensitize().email(email);
// Result: u***@example.com

let phone = "+1-555-123-4567";
let masked_phone = ctx.desensitize().us_phone(phone);
// Result: +1-***-***-4567
```

## Key Methods
- `.email(email)`: Mask an email address.
- `.password(pwd)`: Replace a password with fixed asterisks.
- `.credit_card(card)`: Mask all but the last 4 digits of a credit card.
- `.us_phone(phone)` / `.us_ssn(ssn)`: Mask US phone numbers and Social Security Numbers.
- `.chinese_phone(phone)` / `.chinese_id_card(id)` / `.chinese_name(name)`: Mask Chinese personal information.
