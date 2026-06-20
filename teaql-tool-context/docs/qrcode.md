# TeaQL Tool API: QRCode Utilities

Use `ctx.qrcode()` to generate QR codes in PNG or SVG formats.

## Required Dependencies

To use the QRCode utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
let data = "https://example.com";

// Generate a QR code and save it as a PNG file
ctx.qrcode().generate_png(data, "qrcode.png")?;

// Generate a QR code as an SVG string
let svg_string = ctx.qrcode().generate_svg(data)?;
println!("SVG: {}", svg_string);
```

## Key Methods
- `.generate_png(data, output_path)`: Generate a QR code for the given data and save it as a PNG file.
- `.generate_svg(data)`: Generate a QR code for the given data and return it as an SVG string.
