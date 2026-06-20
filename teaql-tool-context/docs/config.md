# TeaQL Tool API: Config Utilities

Use `ctx.config()` to load and access environment variables.

> [!WARNING]
> Always provide a `.comment()` describing the purpose of the operation.

## Required Dependencies

To use the Config utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Load Environment Variables

```rust
ctx.config()
    .comment("load .env configuration")
    .load_env()?;
```

## Get Environment Variable

```rust
let api_key = ctx.config()
    .comment("read external API key")
    .get_env("EXTERNAL_API_KEY")?;
```

## Key Methods
- `.comment(reason)`: **(Required)** Describe what this operation is doing.
- `.load_env() -> Result<()>`: Loads environment variables from a `.env` file into the current process.
- `.get_env(key: &str) -> Result<String>`: Reads the value of an environment variable by its key.
