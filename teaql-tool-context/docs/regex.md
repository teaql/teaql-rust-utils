# TeaQL Tool API: Regular Expressions

Use `ctx.regex()` to validate, match, and manipulate text using regular expressions.

## Required Dependencies

To use the regex utilities, you must add `teaql-tool` to your project with the `std` feature enabled.

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Regex Operations

```rust
let email_pattern = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$";

// Validate a pattern before using it
if ctx.regex().validate(email_pattern) {
    // Check if text matches
    let is_valid = ctx.regex().is_match(email_pattern, "test@example.com")?;
}

// Find substrings
let text = "My numbers are 123 and 456.";
let first_num = ctx.regex().find(r"\d+", text)?;
let all_nums = ctx.regex().find_all(r"\d+", text)?;

// Replace text
let hidden = ctx.regex().replace_all(r"\d", text, "*")?;

// Split text
let words = ctx.regex().split(r"\s+", "split these words")?;

// Escape special characters for use in a regex
let escaped = ctx.regex().escape("C++");
```

## Key Methods
- `.validate(pattern)`: Checks if the regex pattern is valid and can be compiled.
- `.escape(text)`: Escapes special regex characters in a string.
- `.is_match(pattern, text)`: Returns `true` if the text matches the pattern.
- `.find(pattern, text)`: Finds the first match of the pattern in the text.
- `.find_all(pattern, text)`: Returns all occurrences of the pattern in the text.
- `.replace(pattern, text, rep)`: Replaces the first match of the pattern with the replacement string.
- `.replace_all(pattern, text, rep)`: Replaces all matches of the pattern with the replacement string.
- `.split(pattern, text)`: Splits the text by occurrences of the pattern.
