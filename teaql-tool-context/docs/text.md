# TeaQL Tool API: Text Utilities

Use `ctx.text()` to access string manipulation and checking utilities. This tool is integrated with the TeaQL runtime to provide common text operations.

## Required Dependencies

To use the text utilities and other standard features, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Example Usage

```rust
let snake_case = ctx.text().to_snake_case("Hello World");
let is_valid = ctx.text().starts_with("prefix_string", "prefix");

if ctx.text().contains("some text", "some") {
    let normalized = ctx.text().normalize_whitespace("  multiple   spaces  ");
}
```

## Key Methods
- `.trim(s)`: Trim whitespace from the start and end of a string.
- `.lowercase(s)` / `.uppercase(s)`: Convert string case.
- `.capitalize(s)`: Capitalize the first letter of a string.
- `.to_snake_case(s)` / `.to_camel_case(s)` / `.to_pascal_case(s)` / `.to_kebab_case(s)`: Change string casing format.
- `.normalize_whitespace(s)`: Normalize multiple whitespaces into a single space.
- `.contains(s, substring)`: Check if the string contains a given substring.
- `.starts_with(s, prefix)`: Check if the string starts with a given prefix.
- `.ends_with(s, suffix)`: Check if the string ends with a given suffix.
