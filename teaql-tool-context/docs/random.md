# TeaQL Tool API: Random Utilities

Use `ctx.random()` to generate random numbers and booleans.

## Required Dependencies

To use the Random utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
// Generate a random integer between min and max (inclusive)
let rand_int = ctx.random().int(1, 100);
println!("Random Integer: {}", rand_int);

// Generate a random float between min and max
let rand_float = ctx.random().float(0.0, 1.0);
println!("Random Float: {}", rand_float);

// Generate a random boolean
let rand_bool = ctx.random().boolean();
println!("Random Boolean: {}", rand_bool);
```

## Key Methods
- `.int(min, max)`: Generate a random integer between `min` and `max` (inclusive).
- `.float(min, max)`: Generate a random float between `min` and `max`.
- `.boolean()`: Generate a random boolean value (`true` or `false`).
