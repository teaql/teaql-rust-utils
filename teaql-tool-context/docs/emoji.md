# TeaQL Tool API: Emoji Utilities

Use `ctx.emoji()` to detect, remove, or replace emoji characters in strings.

## Required Dependencies

To use the emoji utilities, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Basic Usage

```rust
let text = "Hello World! 👋🌍";

let has_emoji = ctx.emoji().contains_emoji(text); // true

let clean_text = ctx.emoji().remove_all(text); // "Hello World! "

let replaced = ctx.emoji().replace_all(text, "[EMOJI]"); // "Hello World! [EMOJI][EMOJI]"
```

## Key Methods
- `.contains_emoji(text)`: Check if a string contains any emoji characters. Returns a boolean.
- `.remove_all(text)`: Remove all emoji characters from a string.
- `.replace_all(text, replacement)`: Replace all emoji characters in a string with a given replacement string.
