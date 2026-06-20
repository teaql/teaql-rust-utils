# TeaQL Tool API: Address Utilities

Use `ctx.address()` to access address parsing and utility methods.

> [!WARNING]
> Always provide a `.comment()` describing the purpose of the operation.

## Required Dependencies

To use the Address utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Extract Province

```rust
let province = ctx.address()
    .comment("extract province from user address")
    .extract_province("123 Main St, Springfield, IL 62701");
```

## Key Methods
- `.comment(reason)`: **(Required)** Describe what this operation is doing.
- `.extract_province(address: &str) -> String`: Extracts the province or state from a given address string.
