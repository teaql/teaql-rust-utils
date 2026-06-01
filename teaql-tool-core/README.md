# teaql-tool-core

Core types and error definitions for the [TeaQL Tool](https://github.com/teaql/teaql-rust-utils) ecosystem.

This crate is the strict foundation of the TeaQL Tool library. It contains the unified `Result` type alias and the `TeaQLToolError` enum that all other sub-crates use to guarantee consistent error handling across the entire `T::` facade.

## Usage

You typically do not need to depend on this crate directly unless you are writing a custom plugin for the TeaQL ecosystem. Instead, depend on the `teaql-tool` crate.

```toml
[dependencies]
teaql-tool-core = "0.1"
```

```rust
use teaql_tool_core::{Result, TeaQLToolError};
```
