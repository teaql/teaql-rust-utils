# TeaQL Tool API: JSON Utilities

Use `ctx.json()` to parse, stringify, merge, diff, and manipulate JSON structures easily.

## Required Dependencies

To use the JSON utilities, you must add `teaql-tool` to your project with the `std` feature enabled.

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## JSON Operations

```rust
use serde_json::json;

// Parse and Stringify
let parsed_val = ctx.json().parse(r#"{"name": "TeaQL"}"#)?;
let json_str = ctx.json().stringify(&parsed_val)?;
let pretty_str = ctx.json().stringify_pretty(&parsed_val)?;

// Pointer Operations
let mut data = json!({ "user": { "id": 123, "active": true } });

// Check if a path exists
let has_id = ctx.json().has(&data, "/user/id");

// Get a value
if let Some(id_val) = ctx.json().get(&data, "/user/id") {
    // ...
}

// Set a value
ctx.json().set(&mut data, "/user/role", json!("admin"))?;

// Remove a value
ctx.json().remove(&mut data, "/user/active")?;

// Merging and Diffing
let base = json!({"a": 1});
let update = json!({"b": 2});
let merged = ctx.json().merge(&base, &update);

let diff_patch = ctx.json().diff(&base, &merged);

// Apply a patch
ctx.json().patch(&mut data, &diff_patch)?;
```

## Key Methods
- `.parse(s)`: Parses a JSON string into a `serde_json::Value`.
- `.stringify(v)`: Serializes a `serde_json::Value` into a compact string.
- `.stringify_pretty(v)`: Serializes a `serde_json::Value` into a formatted, pretty string.
- `.has(v, pointer)`: Checks if a JSON pointer path exists in the given value.
- `.get(v, pointer)`: Retrieves a reference to the value at the given JSON pointer path.
- `.set(v, pointer, value)`: Sets a value at the given JSON pointer path.
- `.remove(v, pointer)`: Removes a value at the given JSON pointer path.
- `.merge(a, b)`: Deeply merges two JSON values.
- `.diff(a, b)`: Computes a JSON Patch (`json_patch::Patch`) representing the differences between two values.
- `.patch(v, patch)`: Applies a JSON Patch to the given value.
