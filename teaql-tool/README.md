# teaql-tool

The ultimate utility toolkit for Rust, heavily inspired by Java's Hutool. Designed specifically to provide unified, hallucination-free, AI-friendly APIs for modern enterprise development.

## Features

`teaql-tool` is a **Facade** crate. All utilities are accessed cleanly through the central `T::` namespace, significantly lowering the cognitive load for developers and AI code generators.

- **`teaql-tool-std`**: Zero-dependency standard utilities (String manipulation, File I/O, Data privacy masking, Tree building, Math, Time).
- **`teaql-tool-extra`**: Heavy-dependency extensions (HTTP client, ZIP archive, Static Server, File watcher, JWT, Excel/CSV).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
# By default, only the lightweight "std" utilities are included.
teaql-tool = "0.1"

# To enable networking, web servers, and heavy IO features:
teaql-tool = { version = "0.1", features = ["std", "extra"] }
```

In your Rust code, simply use the `T` facade:

```rust
use teaql_tool::T;

fn main() {
    // String processing
    let camel = T::text().camel_case("hello_world");
    
    // File I/O
    let content = T::file().read_string("config.toml").unwrap();
    
    // Privacy Masking
    let masked = T::desensitize().chinese_phone("13812345678");
    
    // HTTP Request (requires "extra" feature)
    let body = T::http().get("https://api.github.com").unwrap();
}
```

For full documentation, architecture design, and our AI-friendly `.cursorrules` guide, please visit the [GitHub repository](https://github.com/teaql/teaql-rust-utils).
