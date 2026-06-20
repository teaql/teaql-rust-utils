# TeaQL Tool API: Cache Utilities

Use `ctx.cache()` to create and manage in-memory caches. 

> [!NOTE]
> `CacheTool` is stateful — it wraps a cache with a 1-hour TTL. Each call to `.create()` creates a NEW instance with its own independent cache, so data will NOT persist across calls. For production use, store a `CacheTool` instance externally and call its methods directly.

## Required Dependencies

To use the Cache utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Create Cache Instance

```rust
let cache_tool = ctx.cache().create();
// Use cache_tool methods directly to store and retrieve data.
```

## Key Methods
- `.create() -> CacheTool`: Creates a new CacheTool instance with its own independent 1-hour TTL cache store.
