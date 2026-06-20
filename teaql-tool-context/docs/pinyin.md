# TeaQL Tool API: Pinyin Utilities

Use `ctx.pinyin()` to convert Chinese text to Pinyin.

## Required Dependencies

To use the Pinyin utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
// Convert Chinese text to Pinyin
let pinyin_str = ctx.pinyin().get_pinyin("你好世界");
println!("Pinyin: {}", pinyin_str);
```

## Key Methods
- `.get_pinyin(text)`: Convert the provided Chinese text into a Pinyin string.
