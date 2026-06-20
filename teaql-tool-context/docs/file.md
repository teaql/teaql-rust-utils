# TeaQL Tool API: File Utilities

Use `ctx.file()` to perform local filesystem operations. This client is fully integrated with the TeaQL runtime to ensure proper auditing and tracing.

> [!WARNING]
> Always provide a `.purpose()` describing the business reason for read and check operations.
> Always provide an `.audit_as()` describing the business reason for write, modify, and delete operations.

## Required Dependencies

To use the file utilities, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Reading Files

```rust
let content = ctx.file()
    .read_string("config.json")?
    .purpose("load application configuration")?;
```

## Writing Files

```rust
ctx.file()
    .write_string("output.txt", "Hello World!")?
    .audit_as("export generated report to file")?;
```

## Checking Existence

```rust
let is_present = ctx.file()
    .exists("data/report.csv")
    .purpose("check if report exists before generation")?;
```

## Key Methods
- `.read_string(path)` / `.read_bytes(path)`: Read file contents. Requires `.purpose()`.
- `.write_string(path, content)`: Write or overwrite a file. Requires `.audit_as()`.
- `.exists(path)` / `.is_file(path)` / `.is_dir(path)`: Check file or directory status. Requires `.purpose()`.
- `.mkdir(path)` / `.mkdir_all(path)`: Create directories. Requires `.audit_as()`.
- `.delete_file(path)` / `.delete_dir(path)` / `.delete_recursive(path)`: Delete files or directories. Requires `.audit_as()`.
- `.copy(from, to)` / `.rename(from, to)`: Copy or move files/directories. Requires `.audit_as()`.
- `.list_files(dir)` / `.list_dirs(dir)`: List directory contents. Requires `.purpose()`.
