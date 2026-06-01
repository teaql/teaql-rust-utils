# teaql-tool-std

Zero-dependency standard utilities for the [TeaQL Tool](https://github.com/teaql/teaql-rust-utils) ecosystem.

This crate provides a pure, memory-bound utility toolkit for everyday Rust development. It features extremely lightweight dependencies, making it cross-platform and highly suitable for strict security environments.

## Core Capabilities

- **`T::text()`** / **`T::regex()`**: String manipulation and validation.
- **`T::file()`**: Ergonomic filesystem wrapper.
- **`T::time()`**: Date formatting and parsing.
- **`T::desensitize()`**: Data privacy masking (Phone, SSN, Credit Card, Email).
- **`T::tree()`**: Fast builders for converting flat arrays into nested tree structures.
- **`T::net()`**: Local network probes (IP acquisition, port status).
- **`T::money()`** / **`T::decimal()`**: Precise financial computations.
- **`T::id()`**: Snowflake, UUID, NanoID, and ULID generators.

## Usage

You should rarely depend on this crate directly. Instead, depend on the `teaql-tool` facade crate.

```toml
[dependencies]
teaql-tool-std = "0.1"
```
