# TeaQL Tool API: List Utilities

Use `ctx.list()` to perform common collection and slice operations such as chunking, making lists unique, and computing intersections, unions, and differences.

## Required Dependencies

To use the list utilities, you must add `teaql-tool` to your project with the `std` feature enabled.

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## List Operations

```rust
let items = vec![1, 2, 2, 3, 4, 5];
let other = vec![4, 5, 6, 7];

// Chunking
let chunks = ctx.list().chunk(&items, 2); 
// [[1, 2], [2, 3], [4, 5]]

// Unique elements
let unique_items = ctx.list().unique(&items); 
// [1, 2, 3, 4, 5]

// Set Operations
let intersection = ctx.list().intersect(&items, &other); 
// [4, 5]

let union_items = ctx.list().union(&items, &other); 
// [1, 2, 3, 4, 5, 6, 7]

let diff = ctx.list().difference(&items, &other); 
// [1, 2, 3] (items in `items` but not in `other`)
```

## Key Methods
- `.chunk(list, size)`: Splits a list into smaller vectors of the specified size.
- `.unique(list)`: Returns a new vector containing only unique elements from the list.
- `.intersect(a, b)`: Returns a vector of elements that are present in both lists.
- `.union(a, b)`: Returns a vector of all unique elements from both lists.
- `.difference(a, b)`: Returns a vector of elements that are in the first list but not in the second.
