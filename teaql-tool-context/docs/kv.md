# TeaQL Tool API: KV Utilities

Use `ctx.kv()` to open a key-value store database using `sled`.

> [!WARNING]
> Always provide a `.purpose()` describing the business reason for opening the database.

## Required Dependencies

To use the KV utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
// Open a sled database
let db = ctx.kv()
    .purpose("open local key-value store for caching")
    .open("my_db_path")?;

// Use the sled database
db.insert("my_key", "my_value")?;
if let Some(value) = db.get("my_key")? {
    println!("Retrieved: {:?}", value);
}
```

## Key Methods
- `.purpose(reason)`: **(Required)** Describe why this database is being opened.
- `.open(path)`: Open a `sled` database at the specified path.
