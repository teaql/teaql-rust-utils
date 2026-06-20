# TeaQL Tool API: Filter Utilities

Use `ctx.filter()` to efficiently detect and filter sensitive words or patterns from text using the Aho-Corasick algorithm.

## Required Dependencies

To use the filter utilities, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Basic Usage

```rust
// 1. Build a search trie with patterns
let sensitive_words = vec!["badword1", "badword2"];
let trie = ctx.filter().build_trie(sensitive_words);

let text = "This is a sentence containing badword1.";

// 2. Check for sensitive words
let has_sensitive = ctx.filter().contains_sensitive(text, &trie); // true

// 3. Replace sensitive words
let safe_text = ctx.filter().replace_sensitive(text, &trie, "***");
// "This is a sentence containing ***."
```

## Key Methods
- `.build_trie(patterns)`: Build an `AhoCorasick` trie from a list of strings or bytes for efficient multi-pattern search.
- `.contains_sensitive(text, trie)`: Check if the text contains any of the patterns in the trie.
- `.replace_sensitive(text, trie, replacement)`: Replace all matched patterns in the text with the provided replacement string.
