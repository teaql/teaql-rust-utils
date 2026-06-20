# TeaQL Tool API: Unit Utilities

Use `ctx.unit()` to perform conversions between different units of measurement, such as digital storage sizes and temperatures. This tool is natively available within the TeaQL runtime.

## Required Dependencies

To use the unit utilities and other standard features, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Example Usage

```rust
let size_in_kb = ctx.unit().bytes_to_kb(2048);
let size_in_mb = ctx.unit().bytes_to_mb(1048576);

let fahrenheit = ctx.unit().c_to_f(25.0);
let celsius = ctx.unit().f_to_c(77.0);
```

## Key Methods
- `.bytes_to_kb(bytes)`: Convert bytes to kilobytes.
- `.bytes_to_mb(bytes)`: Convert bytes to megabytes.
- `.bytes_to_gb(bytes)`: Convert bytes to gigabytes.
- `.c_to_f(c)`: Convert degrees Celsius to Fahrenheit.
- `.f_to_c(f)`: Convert degrees Fahrenheit to Celsius.
