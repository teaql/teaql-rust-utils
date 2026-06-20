# TeaQL Tool API: CSV Utilities

Use `ctx.csv()` to parse and generate Comma-Separated Values (CSV) data.

> [!WARNING]
> Always provide a `.comment()` describing the purpose of the operation.

## Required Dependencies

To use the CSV utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Parse CSV Data

```rust
let csv_content = "name,age\nAlice,30\nBob,25";

let records = ctx.csv()
    .comment("parse imported user list")
    .parse(csv_content)?;

for row in records {
    println!("{:?}", row);
}
```

## Generate CSV Data

```rust
let data = vec![
    vec!["name".to_string(), "age".to_string()],
    vec!["Alice".to_string(), "30".to_string()],
];

let csv_string = ctx.csv()
    .comment("generate export report")
    .generate(&data)?;
```

## Key Methods
- `.comment(reason)`: **(Required)** Describe what this operation is doing.
- `.parse(data: &str) -> Result<Vec<Vec<String>>>`: Parses a CSV formatted string into a vector of records.
- `.generate(records: &[Vec<String>]) -> Result<String>`: Generates a CSV formatted string from a list of string vectors.
