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

## 🛠️ Feature Inventory

The workspace contains **52 unique tools** exposed through the `T::xxx()` facade: **26 standard tools** and **26 optional extension tools**. Facade and context crates reuse these implementations and are therefore not counted again.

### Package Overview

| Package | Role | Contents |
| --- | --- | --- |
| `teaql-tool-core` | Shared foundation | No `T::` tools. Provides `Result`, `TeaQLToolError`, `MustPurpose`, `MustComment`, `MustAuditAs`, and audit configuration/formatting types. |
| `teaql-tool-std` | Standard tool implementation | 26 lightweight, general-purpose tools. |
| `teaql-tool-extra` | Optional extension package | 26 heavier tools for IO, protocols, automation, and integrations. |
| `teaql-tool` | Public facade | Exposes all 52 tools through `T::xxx()`; `std` enables 26 and `extra` enables another 26. The default `minimal` feature enables `std`. |
| `teaql-tool-context` | Application-layer adapters | Adds `UserContext` access and intent wrappers. It currently adapts all 26 standard tools, 21 extension tools, and a separate async `ctx.http()` adapter. It does not add new tool implementations. |

### Standard Package (`teaql-tool-std`, 26 tools)

| Tool | Main capabilities |
| --- | --- |
| `T::codec()` | Base64, hex, URL, and HTML encoding/decoding |
| `T::color()` | Named CSS colors |
| `T::daterange()` | Day/hour ranges and offsets |
| `T::decimal()` | Exact decimal arithmetic, rounding, ratios, and percentages |
| `T::desensitize()` | Mask IDs, phone numbers, names, SSNs, cards, email, and passwords |
| `T::diff()` | Text diff generation |
| `T::emoji()` | Detect, remove, and replace emoji |
| `T::file()` | Read, write, inspect, copy, rename, list, and delete files/directories |
| `T::filter()` | Build sensitive-word tries, detect matches, and replace matches |
| `T::hash()` | SHA-256, SHA-512, BLAKE3, and CRC32 |
| `T::high_res_timer()` | Nanosecond, microsecond, and millisecond timing |
| `T::i18n()` | Locale dictionaries, JSON loading, lookup, and interpolation |
| `T::id()` | UUID, UUID v7, ULID, NanoID, and prefixed IDs |
| `T::json()` | Parse, serialize, query, merge, diff, and patch JSON |
| `T::list()` | Chunk, deduplicate, intersect, union, and subtract lists |
| `T::map()` | Merge and invert maps |
| `T::money()` | Currency-safe arithmetic, allocation, rounding, and formatting |
| `T::net()` | Local IPv4 lookup, port checks, and private-IP detection |
| `T::regex()` | Match, find, replace, split, escape, and validate regexes |
| `T::system()` | Environment, OS, architecture, and current-directory information |
| `T::text()` | Trimming, casing, case conversion, and whitespace normalization |
| `T::time()` | Current time/date, parsing, date math, boundaries, and timezones |
| `T::tree()` | Convert flat records into nested trees |
| `T::unit()` | Byte-size and Celsius/Fahrenheit conversions |
| `T::url()` | URL parsing and percent encoding/decoding |
| `T::validate()` | Email, URL, and string-length validation |

### Extension Package (`teaql-tool-extra`, 26 tools)

Enable these heavier dependencies with the `extra` feature.

| Tool | Main capabilities |
| --- | --- |
| `T::address()` | Extract Chinese provinces from addresses |
| `T::archive()` | Create and extract ZIP archives |
| `T::barcode()` | Generate Code 128 as PNG or SVG |
| `T::cache()` | In-memory cache put/get operations |
| `T::clipboard()` | Read and write system clipboard text |
| `T::cmd()` | Run shell commands with a timeout |
| `T::config()` | Load `.env` files and read environment variables |
| `T::cron()` | Schedule jobs with cron expressions |
| `T::crypto()` | Generate keys and perform AES-GCM encryption/decryption |
| `T::csv()` | Parse and generate CSV data |
| `T::email()` | Send email through SMTP |
| `T::excel()` | Read and write simple spreadsheets |
| `T::geo()` | Calculate geographic distance |
| `T::html()` | Select text and attributes with CSS selectors |
| `T::http()` | Perform blocking HTTP GET requests |
| `T::image()` | Resize images |
| `T::jwt()` | Sign and verify JSON Web Tokens |
| `T::kv()` | Open an embedded `sled` key-value database |
| `T::phone()` | Parse, validate, and format phone numbers |
| `T::pinyin()` | Convert Chinese text to Pinyin |
| `T::proxy()` | Run a reverse proxy |
| `T::qrcode()` | Generate QR codes as PNG or SVG |
| `T::random()` | Generate random integers, floats, and booleans |
| `T::server()` | Serve a directory over HTTP |
| `T::template()` | Render Tera templates with JSON data |
| `T::watcher()` | Watch filesystem changes recursively |

### Extension Packages and Feature Flags

- `teaql-tool-extra` is the optional heavy-tool extension. Enable it with `features = ["std", "extra"]` to make all 52 `T::` tools available.
- `teaql-tool-context` is the application-layer extension. Its `std`, `extra`, and `http` features expose context-bound `ctx.xxx()` adapters; `all` enables all three.
- The context `extra` adapters currently cover `address`, `archive`, `barcode`, `cache`, `clipboard`, `cmd`, `config`, `crypto`, `csv`, `email`, `excel`, `geo`, `html`, `image`, `jwt`, `kv`, `phone`, `pinyin`, `qrcode`, `random`, and `template`. `http` has its own feature; `cron`, `proxy`, `server`, and `watcher` currently have no context adapter.

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
