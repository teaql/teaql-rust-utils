# TeaQL Tool API: Clipboard Utilities

Use `ctx.clipboard()` to read from and write to the system clipboard.

> [!WARNING]
> Always provide a `.comment()` describing the purpose of the operation.

## Required Dependencies

To use the Clipboard utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Read from Clipboard

```rust
let clipboard_content = ctx.clipboard()
    .comment("read user copied token")
    .read_text()?;
```

## Write to Clipboard

```rust
ctx.clipboard()
    .comment("copy generated password to clipboard")
    .write_text("secure_password_123")?;
```

## Key Methods
- `.comment(reason)`: **(Required)** Describe what this operation is doing.
- `.read_text() -> Result<String>`: Reads text content from the system clipboard.
- `.write_text(text: &str) -> Result<()>`: Writes text content to the system clipboard.
