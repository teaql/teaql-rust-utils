# TeaQL Tool API: Tree Utilities

Use `ctx.tree()` to manipulate tree structures, such as building nested JSON objects from flat arrays. This tool is fully integrated with the TeaQL runtime for handling hierarchical data.

## Required Dependencies

To use the tree utilities and other standard features, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Example Usage

```rust
let flat_data = serde_json::json!([
    { "id": 1, "parent_id": null, "name": "Root" },
    { "id": 2, "parent_id": 1, "name": "Child" }
]);

let tree = ctx.tree().build(
    flat_data,
    "id",
    "parent_id",
    "children",
    serde_json::Value::Null
)?;
```

## Key Methods
- `.build(flat_array, id_field, parent_id_field, children_field, root_parent_id)`: Build a nested tree structure (JSON value) from a flat array based on parent-child relationships.
