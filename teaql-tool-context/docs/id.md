# TeaQL Tool API: ID Generation

Use `ctx.id()` to generate various types of unique identifiers such as UUIDs, ULIDs, and NanoIDs.

## Required Dependencies

To use the ID generation utilities, you must add `teaql-tool` to your project with the `std` feature enabled.

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## ID Generation

```rust
// Generate a standard UUID (v4)
let my_uuid = ctx.id().uuid();

// Generate a time-ordered UUID v7
let time_uuid = ctx.id().uuid_v7();

// Generate a sortable ULID
let my_ulid = ctx.id().ulid();

// Generate a compact NanoID
let my_nanoid = ctx.id().nanoid();

// Generate an ID with a custom prefix
let user_id = ctx.id().with_prefix("usr_");
```

## Key Methods
- `.uuid()`: Generates a standard UUID v4 string.
- `.uuid_v7()`: Generates a time-ordered UUID v7 string.
- `.ulid()`: Generates a Universally Unique Lexicographically Sortable Identifier (ULID).
- `.nanoid()`: Generates a compact NanoID string.
- `.with_prefix(prefix)`: Generates a unique identifier prefixed with the given string.
