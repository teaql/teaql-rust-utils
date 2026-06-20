# TeaQL Tool API: URL Utilities

Use `ctx.url()` to encode, decode, and parse URLs. This client is fully integrated with the TeaQL runtime for robust URL manipulation.

## Required Dependencies

To use the URL utilities and other standard features, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Example Usage

```rust
let encoded = ctx.url().encode("hello world & others");
let decoded = ctx.url().decode(&encoded)?;

let parsed_url = ctx.url().parse("https://example.com/path?query=1")?;
```

## Key Methods
- `.encode(text)`: Safely encode text to be used in a URL.
- `.decode(text)`: Decode a URL-encoded string.
- `.parse(url)`: Parse a string into a structured `Url` object.
