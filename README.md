# TeaQL Tool

```text
  ██████████ ███████   █████    ██████  ██      
      ██     ██       ██   ██  ██    ██ ██      
      ██     █████    ███████  ██    ██ ██      
      ██     ██       ██   ██  ██ ▄▄ ██ ██      
      ██     ███████  ██   ██   ██████  ███████ 
      ▲
      │
      │
      │
    T:: The Unified API Facade
```

**A unified, stateless, and AI-friendly standard library facade for the TeaQL platform in Rust.**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](#)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](#)

---

## 🎯 Motivation

When developing business applications or working with AI coding agents, you frequently need essential tools: parsing dates, generating UUIDs, calculating exact monetary values, reading JSON, checking emails, or encrypting data. 

In the Rust ecosystem, this normally requires hunting down dozens of different crates (`chrono`, `uuid`, `rust_decimal`, `regex`, `base64`, `aes-gcm`, etc.), learning each of their unique APIs, and dealing with potential breaking changes. 

**TeaQL Tool solves this by providing the `T::` Facade.** 

It gathers the most robust, community-standard crates and wraps them in a completely **unified, stateless, and highly predictable API**. This drastically lowers the cognitive load for developers and makes it incredibly easy for AI agents to generate correct Rust code without needing to constantly learn new third-party library updates.

---

## 📦 Installation

Add `teaql-tool` to your `Cargo.toml`. You can selectively opt into features depending on the weight of the dependencies you need.

```toml
[dependencies]
# For the standard lightweight utilities
teaql-tool = { version = "0.1", features = ["std"] }

# For everything (including cryptography, random generation, csv parsing)
teaql-tool = { version = "0.1", features = ["std", "extra"] }
```

---

## 🛠️ Feature Overview & Examples

The facade exposes a static struct `T`, which you can use to immediately access any module.

```rust
use teaql_tool::T;
```

### 1. `T::id()` - Identifiers
Generate unique IDs instantly without wrestling with crate features.
```rust
let uuid = T::id().uuid();      // "550e8400-e29b-41d4-a716-446655440000"
let ulid = T::id().ulid();      // Lexicographically sortable ID
let nanoid = T::id().nanoid();  // URL-friendly string ID
```

### 2. `T::time()` - Time and Dates (Powered by `chrono`)
Extremely ergonomic timezone-aware date operations.
```rust
let now = T::time().now();
let tokyo_time = T::time().to_timezone(&now, "Asia/Tokyo").unwrap();
let formatted = T::time().format(&tokyo_time, "%Y-%m-%d %H:%M:%S");

// Date math
let tomorrow = T::time().add_days(&now, 1);
```

### 3. `T::money()` - Exact Monetary Math
Eliminate floating-point errors and "missing penny" issues when splitting funds.
```rust
let bill = T::money().of("100.00", "USD").unwrap();
let tip = T::money().of("15.50", "USD").unwrap();
let total = T::money().add(&bill, &tip).unwrap();

// Split exactly into ratios without losing a cent
let ratios = vec![1, 2]; // e.g., 1/3 and 2/3 splits
let split = T::money().allocate(&total, ratios).unwrap();
```

### 4. `T::decimal()` - Arbitrary Precision Math
For exact business calculations.
```rust
let a = T::decimal().of("10.5").unwrap();
let b = T::decimal().of("2.0").unwrap();
let result = T::decimal().div(a, b).unwrap(); // 5.25
```

### 5. `T::codec()` - Encoding & Decoding
No more hunting for `base64` or `urlencoding` logic.
```rust
let b64 = T::codec().base64_encode("Hello World");
let hex = T::codec().hex_encode([0xFF, 0xAA]);
let url = T::codec().url_encode("https://example.com/?q=teaql");
let safe_html = T::codec().html_escape("<script>alert('1')</script>");
```

### 6. `T::hash()` - Cryptographic Hashes
```rust
let sha256_hash = T::hash().sha256(b"secret");
let fast_hash = T::hash().blake3(b"data stream");
```

### 7. `T::validate()` - Data Validation
Quickly sanitize user input.
```rust
assert!(T::validate().email("user@example.com"));
assert!(T::validate().url("https://teaql.com"));
assert!(T::validate().min_length("password123", 8));
```

### 8. `T::json()` - Dynamic JSON Manipulation
```rust
let mut doc = T::json().parse(r#"{"user": {"name": "Alice"}}"#).unwrap();
T::json().set(&mut doc, "/user/age", serde_json::json!(30)).unwrap();
```

### 9. `T::regex()` - Pattern Matching
```rust
let is_match = T::regex().matches("^[a-z]+$", "hello");
let replaced = T::regex().replace_all("[0-9]+", "foo123bar456", "X");
```

### 10. `T::text()` - String Utilities
```rust
let snake = T::text().to_snake_case("camelCaseString");
let camel = T::text().to_camel_case("snake_case_string");
```

### 11. `T::crypto()` (Requires `extra` feature)
State-of-the-art AES-256-GCM symmetric encryption made dead simple.
```rust
let key = T::crypto().generate_key(); // 32 bytes
let encrypted = T::crypto().encrypt(b"My confidential data", &key).unwrap();
let decrypted = T::crypto().decrypt(&encrypted, &key).unwrap();
```

### 12. `T::random()` (Requires `extra` feature)
```rust
let random_int = T::random().int(1, 100);
let chance = T::random().boolean();
```

### 13. `T::csv()` (Requires `extra` feature)
Parse and generate CSV formats instantly.
```rust
let data = T::csv().parse("name,age\nAlice,30").unwrap();
```

---

## 🏗️ Architecture

The project is broken into a workspace to maintain fast compile times:

1. **`teaql-tool-core`**: The bedrock. Defines `TeaQLToolError` and the unified `Result` type.
2. **`teaql-tool-std`**: The "Standard Profile". Contains all lightweight, high-usage modules (`id`, `time`, `codec`, `hash`, `validate`, `decimal`, `money`, `regex`, `text`, `json`).
3. **`teaql-tool-extra`**: The "Extended Profile". Contains heavier dependencies like cryptography (`aes-gcm`), data formats (`csv`), and random number generators (`rand`).
4. **`teaql-tool`**: The API Facade. Re-exports the underlying tools into the clean `T::` namespace based on active Cargo features.

---

## 🤝 Contributing

Contributions are welcome! If you want to add a new tool to the `T::` namespace:
1. Identify if it belongs in `std` (lightweight/common) or `extra` (heavy/domain-specific).
2. Create the wrapper in the respective crate.
3. Ensure the tool exposes a stateless struct and implements `new()` and `Default`.
4. Register the tool in `teaql-tool/src/lib.rs` under the appropriate feature flag.
5. Write tests using the Facade API (`tests/facade_test.rs`).

## 📄 License

This project is licensed under the MIT License.
