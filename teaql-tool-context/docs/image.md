# TeaQL Tool API: Image Utilities

Use `ctx.image()` to perform image processing operations, such as resizing.

## Required Dependencies

To use the Image utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
// Resize an image to 800x600
ctx.image().resize("input.jpg", "output.jpg", 800, 600)?;
```

## Key Methods
- `.resize(input_path, output_path, width, height)`: Resize an image from the input path and save it to the output path with the specified dimensions.
