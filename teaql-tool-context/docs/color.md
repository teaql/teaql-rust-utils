# TeaQL Tool API: Color Utilities

Use `ctx.color()` to access standard web color names as string references. This is useful for generating UI properties or styling output.

## Required Dependencies

To use the color utilities, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Basic Usage

```rust
let primary_color = ctx.color().blue();
let background_color = ctx.color().alice_blue();
let error_color = ctx.color().red();
```

## Key Methods
- `.alice_blue()`, `.red()`, `.green()`, `.blue()`, `.white()`, `.black()`, etc.: Returns the corresponding standard color name as a `&'static str`.
