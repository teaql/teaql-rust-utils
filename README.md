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
    T:: The Trusted Tool Facade
```

**A unified, stateless, and AI-friendly standard library facade for the TeaQL platform in Rust.**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](#)
[![License](https://img.shields.io/badge/license-Apache%202-blue.svg)](#)

---

## 🎯 Motivation

When developing business applications, quick automation scripts, or working with AI coding agents, you frequently need essential tools: parsing dates, generating UUIDs, calculating exact monetary values, reading JSON, checking emails, or encrypting data. 

In the Rust ecosystem, this normally requires hunting down dozens of different crates (`chrono`, `uuid`, `rust_decimal`, `regex`, `base64`, `reqwest`, `zip`, etc.), learning each of their unique APIs, and dealing with potential breaking changes. 

**TeaQL Tool solves this by providing the `T::` Facade.** 

It gathers the most robust, community-standard crates and wraps them in a completely **unified, stateless, and highly predictable API**. This drastically lowers the cognitive load for developers and makes it incredibly easy for AI agents to generate correct Rust code without needing to constantly learn new third-party library updates.

## 📦 Installation

Add `teaql-tool` to your `Cargo.toml`. You can selectively opt into features depending on the weight of the dependencies you need.

```toml
[dependencies]
# For the standard lightweight utilities
teaql-tool = { version = "0.1", features = ["std"] }

# For everything (including network, crypto, images, web scraping, and watchers)
teaql-tool = { version = "0.1", features = ["std", "extra"] }
```

---

## 🛠️ Module Overview & Examples

The facade exposes a static struct `T`, which you can use to immediately access any module. Simply `use teaql_tool::T;`.

### 🧰 Core Data & Logic (`teaql-tool-std`)

- **`T::id()`**: Instantly generate `uuid()`, `ulid()`, or `nanoid()`.
- **`T::time()`**: Ergonomic timezone-aware math and formatting powered by `chrono`.
  ```rust
  let tomorrow = T::time().add_days(&T::time().now(), 1);
  ```
- **`T::money()`**: Exact monetary division and math without floating-point errors.
- **`T::decimal()`**: Arbitrary precision math.
- **`T::codec()`**: Base64, Hex, URL-encoding in one line.
  ```rust
  let encoded = T::codec().base64_encode(b"Hello");
  ```
- **`T::text()`**: Case conversions (snake, camel, kebab), truncations, and padding.
- **`T::json()`**: Parse to/from JSON, apply JSON Patches, and query via JSON Pointers.
- **`T::regex()`**: Extract patterns or validate regex without manual compilation.
- **`T::list()` / `T::map()`**: Quick iterators, grouping, sorting, and map inversions.
- **`T::diff()`**: Compare text strings to get unified diffs.
- **`T::unit()`**: Convert bytes (KB/MB) and durations.
- **`T::color()`**: Convert RGB, HEX, and HSL.

### 🚀 Automation & Scripts (`teaql-tool-extra`)

The `extra` feature pulls in heavier dependencies designed to give your scripts superpowers.

- **`T::cmd()`**: Execute shell commands easily with built-in timeouts.
  ```rust
  let (stdout, stderr, code) = T::cmd().run_with_timeout("ls -al", 5).unwrap();
  ```
- **`T::server()`**: Start a static file HTTP server in one line (blocking).
  ```rust
  T::server().serve_dir("./public", 8080).unwrap();
  ```
- **`T::proxy()`**: Start a transparent reverse proxy.
  ```rust
  T::proxy().start(8081, "http://127.0.0.1:3000").unwrap();
  ```
- **`T::watcher()`**: Monitor filesystem changes recursively.
  ```rust
  T::watcher().watch("./src", |changed_path| println!("Changed: {}", changed_path)).unwrap();
  ```
- **`T::archive()`**: One-line Zip creation and extraction.
  ```rust
  T::archive().zip_dir("./src", "backup.zip").unwrap();
  ```
- **`T::html()`**: Scrape web content using CSS selectors.
  ```rust
  let links = T::html().select_attr(html_str, "a.active", "href").unwrap();
  ```
- **`T::cron()`**: Run background scheduling with cron expressions.
  ```rust
  T::cron().schedule("0 * * * * *", || println!("Ran every minute!")).unwrap();
  ```
- **`T::kv()`**: Open an embedded, pure-Rust key-value database (`sled`).
  ```rust
  let db = T::kv().open("./local.db").unwrap();
  db.insert("key", "value").unwrap();
  ```
- **`T::clipboard()`**: Read and write cross-platform clipboard content.
- **`T::pinyin()`**: Convert Chinese characters to Pinyin effortlessly.

### 🌍 Business & Integration (`teaql-tool-extra`)

- **`T::http()`**: Fire off GET/POST requests without setting up async clients manually.
- **`T::crypto()`**: Symmetrical (`aes-gcm`), asymmetrical (`rsa`), and HMAC signatures.
- **`T::jwt()`**: Sign and verify JSON Web Tokens.
- **`T::email()`**: Construct and send emails over SMTP.
- **`T::excel()` / `T::csv()`**: Parse spreadsheets directly into lists of structures.
- **`T::image()`**: Resize, crop, and convert image formats.
- **`T::barcode()` / `T::qrcode()`**: Generate barcodes and QRCodes as PNG or SVG.
- **`T::template()`**: Render Tera templates with JSON data.

---

## 💻 Full Scripting Example

With `teaql-tool`, you can write incredibly concise utilities. Here is an example of checking a server, saving the status to a local DB, and writing to the clipboard:

```rust
use teaql_tool::T;

fn main() {
    // 1. Fetch data from a URL
    let html = T::http().get("https://rust-lang.org").unwrap();
    
    // 2. Extract specific text using CSS selectors
    let titles = T::html().select_text(&html, "title").unwrap();
    let site_title = &titles[0];

    // 3. Save it to a lightweight local database
    let db = T::kv().open("./scraper.db").unwrap();
    db.insert("latest_title", site_title).unwrap();

    // 4. Copy to your OS clipboard
    T::clipboard().write_text(site_title).unwrap();

    println!("Scraped: {}", site_title);
}
```

## 🛡️ Application Layer Usage (Context & Auditing)

While the `T::` facade is fantastic for standalone scripts or internal framework logic, **business applications have strict requirements for context-awareness (Timezones, Locales) and auditability.**

For application layer code, `teaql-tool` provides `teaql-tool-context`. This completely shadows the raw `T::` facade and binds all tools to the `UserContext` (`ctx`). 

### The `MustComment` Constraint
To prevent "naked" logic and enforce self-documenting code, every pure calculation or IO operation at the application layer is wrapped in a `MustComment<T>` or `PendingAction`. You cannot extract the result or execute the IO without explicitly chaining `.comment("intent")`.

```rust
use teaql_tool_context::prelude::*;

// 1. Context-Aware Pure Math (Timezone injected automatically)
let deadline = ctx.time().today().add_days(7).comment("Calculate grace period deadline");

// 2. Context-Aware IO (Automatically logs Trace ID and intent)
let data = ctx.http().get("https://api.github.com/tasks")
    .comment("Sync latest tasks from external provider")
    .await?;
```

---

## 🤖 AI & Developer Guardrails (Enforcing Context)

If you are using AI agents (like Cursor) or building a large team, you must prevent developers and AI from bypassing the `ctx` layer. We provide physical and prompt-based guardrails to ensure 100% compliance.

### 1. The Compiler Block (`clippy.toml`)
Place this in your application root to physically prevent compilation if raw tools or `std::fs` are used:

```toml
disallowed-types = [
    "teaql_tool::T",
    "chrono::Utc",
    "chrono::Local",
    "std::fs::File",
]

disallowed-methods = [
    "teaql_tool::T::*",
    "std::fs::read",
    "std::fs::read_to_string",
    "std::fs::write",
    "std::process::Command::new",
    "reqwest::get"
]
```

### 2. The AI Sandbox (`.cursorrules`)
Place this prompt directive in your project root to align the AI before it even writes code:

```markdown
# Business Logic Coding Rules (CRITICAL)

1. **ABSOLUTE BAN ON `T::` TOOLS**: Inside the application layer, you are strictly forbidden from calling any stateless utility from the `teaql_tool::T` facade directly. 
2. **MANDATORY CONTEXT USAGE**: All side effects (network, file) and all stateful computations (time, formatting, ID generation) MUST go through the user context (`ctx`).
3. **MANDATORY BUSINESS INTENT**: Every single tool call must be appended with `.comment("English intent description")`. Without this, the compiler will reject the `MustComment<T>` wrapper.
```

---

## 📜 License

This project is licensed under the Apache License, Version 2.0.
