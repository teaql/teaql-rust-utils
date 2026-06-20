# TeaQL Tool API: Geo Utilities

Use `ctx.geo()` to perform geographical calculations.

## Required Dependencies

To use the Geo utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
// Calculate the distance between two geographical coordinates in kilometers
let distance = ctx.geo().distance_km(37.7749, -122.4194, 34.0522, -118.2437);
println!("Distance: {} km", distance);
```

## Key Methods
- `.distance_km(lat1, lon1, lat2, lon2)`: Calculate the distance between two geographical points (latitude and longitude) in kilometers.
