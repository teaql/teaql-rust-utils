# TeaQL Tool API: High Resolution Timer

Use `ctx.high_res_timer()` to access high-resolution timing utilities. This is useful for precise performance measurement and profiling within your TeaQL applications.

## Required Dependencies

To use the high resolution timer, you must add `teaql-tool` to your project with the `std` feature enabled.

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Basic Usage

```rust
// Start a new high-resolution timer
let timer = ctx.high_res_timer().start();

// Perform some work
// ...

// Get the elapsed time (assuming HighResTimer has standard methods to check elapsed time)
// let elapsed = timer.elapsed();
```

## Key Methods
- `.start()`: Starts and returns a new `HighResTimer` instance for measuring elapsed time.
