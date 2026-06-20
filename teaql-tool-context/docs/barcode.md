# TeaQL Tool API: Barcode Utilities

Use `ctx.barcode()` to generate Code 128 barcodes in PNG or SVG formats.

> [!WARNING]
> Always provide a `.comment()` describing the purpose of the operation.

## Required Dependencies

To use the Barcode utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Generate Barcode

```rust
// Generate as PNG file
ctx.barcode()
    .comment("generate order tracking barcode image")
    .generate_code128_png("TRACKING-123", "/path/to/barcode.png")?;

// Generate as SVG string
let svg_string = ctx.barcode()
    .comment("generate order tracking barcode SVG")
    .generate_code128_svg("TRACKING-123")?;
```

## Key Methods
- `.comment(reason)`: **(Required)** Describe what this operation is doing.
- `.generate_code128_png(data: &str, output_path: &str) -> Result<()>`: Generates a Code 128 barcode and saves it as a PNG file.
- `.generate_code128_svg(data: &str) -> Result<String>`: Generates a Code 128 barcode and returns it as an SVG string.
