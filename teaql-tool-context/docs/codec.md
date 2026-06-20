# TeaQL Tool API: Codec Utilities

Use `ctx.codec()` to perform various encoding and decoding operations like Base64, Hex, URL, and HTML escaping.

## Required Dependencies

To use the codec utilities, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Basic Usage

```rust
// Base64 encoding/decoding
let encoded = ctx.codec().base64_encode(b"hello world");
let decoded = ctx.codec().base64_decode(&encoded)?;

// URL encoding/decoding
let url_safe = ctx.codec().url_encode("hello world!");
let original = ctx.codec().url_decode(&url_safe)?;
```

## Key Methods
- `.base64_encode(data)` / `.base64_decode(data)`: Encode to or decode from Base64.
- `.hex_encode(data)` / `.hex_decode(data)`: Encode to or decode from hexadecimal.
- `.url_encode(data)` / `.url_decode(data)`: Encode or decode URL components.
- `.html_escape(data)` / `.html_unescape(data)`: Escape or unescape HTML entities.
