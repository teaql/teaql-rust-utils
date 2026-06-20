# TeaQL Tool API: HTML Utilities

Use `ctx.html()` to parse and extract data from HTML strings using CSS selectors.

## Required Dependencies

To use the HTML utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
let html_content = r#"
    <div class="user">
        <span class="name" data-id="1">Alice</span>
        <span class="name" data-id="2">Bob</span>
    </div>
"#;

// Select text content
let names = ctx.html().select_text(html_content, ".user .name")?;
println!("Names: {:?}", names);

// Select attribute values
let ids = ctx.html().select_attr(html_content, ".user .name", "data-id")?;
println!("IDs: {:?}", ids);
```

## Key Methods
- `.select_text(html_str, selector_str)`: Extract the text content of elements matching the CSS selector.
- `.select_attr(html_str, selector_str, attr)`: Extract the value of a specific attribute from elements matching the CSS selector.
