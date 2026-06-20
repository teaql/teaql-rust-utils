# TeaQL Tool API: Template Utilities

Use `ctx.template()` to render text templates using JSON context data.

## Required Dependencies

To use the Template utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
let template_str = "Hello, {{name}}!";
let context_json = r#"{"name": "Alice"}"#;

// Render the template with the provided JSON context
let rendered = ctx.template().render(template_str, context_json)?;
println!("Rendered: {}", rendered);
```

## Key Methods
- `.render(template, ctx_json)`: Render a template string using context data provided as a JSON string.
