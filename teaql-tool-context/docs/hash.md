# TeaQL Tool API: Hash Utilities

Use `ctx.hash()` to compute common cryptographic and non-cryptographic hashes for data integrity and verification.

## Required Dependencies

To use the hash utilities, you must add `teaql-tool` to your project with the `std` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Basic Usage

```rust
let data = b"hello world";

let sha256_hash = ctx.hash().sha256(data);
let blake3_hash = ctx.hash().blake3(data);
let checksum = ctx.hash().crc32(data);
```

## Key Methods
- `.sha256(data)`: Compute the SHA-256 hash of the given data and return it as a hex string.
- `.sha512(data)`: Compute the SHA-512 hash of the given data and return it as a hex string.
- `.blake3(data)`: Compute the BLAKE3 hash of the given data and return it as a hex string.
- `.crc32(data)`: Compute the CRC32 checksum of the given data and return it as a `u32`.
