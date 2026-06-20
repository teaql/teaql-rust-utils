# TeaQL Tool API: Diff Utilities

Use `ctx.diff()` to generate differences between two text strings.

## Required Dependencies

To use the diff utilities, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Basic Usage

```rust
let old_text = "The quick brown fox jumps over the lazy dog.";
let new_text = "The quick red fox jumps over the sleeping dog.";

let changes = ctx.diff().text_diff(old_text, new_text);
println!("Changes:\n{}", changes);
```

## Key Methods
- `.text_diff(old, new)`: Compare two strings and return a diff representation.
