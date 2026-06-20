# TeaQL Tool API: Excel Utilities

Use `ctx.excel()` to read and write simple Excel files.

## Required Dependencies

To use the Excel utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
// Writing data to an Excel file
let data = vec![
    vec!["Name".to_string(), "Age".to_string()],
    vec!["Alice".to_string(), "30".to_string()],
    vec!["Bob".to_string(), "25".to_string()],
];
ctx.excel().write_simple("output.xlsx", &data)?;

// Reading data from an Excel file
let read_data = ctx.excel().read_simple("output.xlsx")?;
for row in read_data {
    println!("{:?}", row);
}
```

## Key Methods
- `.write_simple(path, data)`: Write a 2D vector of strings to an Excel file.
- `.read_simple(path)`: Read an Excel file into a 2D vector of strings.
