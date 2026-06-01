# teaql-tool-extra

Heavy-dependency extra utilities for the [TeaQL Tool](https://github.com/teaql/teaql-rust-utils) ecosystem.

This crate contains the "heavy lifting" utilities that bring in robust external dependencies like `tokio`, `reqwest`, `zip`, `notify`, and `calamine`. It is designed to act as a one-stop toolkit for complex IO and protocol operations.

## Core Capabilities

- **`T::http()`**: Synchronous and asynchronous HTTP clients.
- **`T::server()`**: Instant static HTTP servers.
- **`T::archive()`**: ZIP and tarball manipulation.
- **`T::config()`**: Environment and configuration loading (`.env`, `.toml`).
- **`T::watcher()`**: Filesystem hot-reloading watchers.
- **`T::csv()`** / **`T::excel()`**: Spreadsheet reading and writing.
- **`T::jwt()`**: JWT Token creation and verification.
- **`T::image()`**: Image encoding and processing.

## Usage

You should rarely depend on this crate directly. Instead, depend on the `teaql-tool` facade crate and enable the `"extra"` feature.

```toml
[dependencies]
teaql-tool-extra = "0.1"
```
