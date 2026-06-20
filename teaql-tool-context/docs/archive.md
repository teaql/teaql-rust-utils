# TeaQL Tool API: Archive Utilities

Use `ctx.archive()` to create and extract ZIP archives.

> [!WARNING]
> Always provide a `.comment()` describing the purpose of the operation.

## Required Dependencies

To use the Archive utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Zip Directory

```rust
ctx.archive()
    .comment("zip user uploaded files")
    .zip_dir("/path/to/src", "/path/to/output.zip")?;
```

## Unzip File

```rust
ctx.archive()
    .comment("unzip downloaded resources")
    .unzip("/path/to/input.zip", "/path/to/dst")?;
```

## Key Methods
- `.comment(reason)`: **(Required)** Describe what this operation is doing.
- `.zip_dir(src_dir: &str, dst_file: &str) -> Result<()>`: Compresses a directory into a ZIP file.
- `.unzip(src_file: &str, dst_dir: &str) -> Result<()>`: Extracts a ZIP file into a directory.
