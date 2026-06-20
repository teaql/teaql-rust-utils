# TeaQL Tool API: Map Utilities

Use `ctx.map()` to perform operations on HashMaps, such as merging multiple maps or flipping their keys and values.

## Required Dependencies

To use the map utilities, you must add `teaql-tool` to your project with the `std` feature enabled.

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Map Operations

```rust
use std::collections::HashMap;

let mut map1 = HashMap::new();
map1.insert("a", 1);

let mut map2 = HashMap::new();
map2.insert("b", 2);

// Merge maps
let merged = ctx.map().merge(&map1, &map2);
// {"a": 1, "b": 2}

// Flip keys and values
let flipped = ctx.map().flip(&merged);
// {1: "a", 2: "b"}
```

## Key Methods
- `.merge(a, b)`: Combines two `HashMap`s into one. Conflicting keys from `b` typically overwrite those from `a`.
- `.flip(map)`: Creates a new `HashMap` where the values of the original map become the keys, and the keys become the values.
