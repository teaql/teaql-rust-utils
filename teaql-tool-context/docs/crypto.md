# TeaQL Tool API: Crypto Utilities

Use `ctx.crypto()` for cryptographic operations such as key generation, encryption, and decryption.

> [!WARNING]
> Always provide a `.comment()` describing the purpose of the operation.

## Required Dependencies

To use the Crypto utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Key Generation

```rust
let secret_key = ctx.crypto()
    .comment("generate secure encryption key")
    .generate_key();
```

## Encrypt and Decrypt

```rust
let plaintext = b"secure sensitive data";

let encrypted_data = ctx.crypto()
    .comment("encrypt sensitive user information")
    .encrypt(plaintext, &secret_key)?;

let decrypted_data = ctx.crypto()
    .comment("decrypt sensitive user information")
    .decrypt(&encrypted_data, &secret_key)?;

assert_eq!(plaintext, decrypted_data.as_slice());
```

## Key Methods
- `.comment(reason)`: **(Required)** Describe what this operation is doing.
- `.generate_key() -> Vec<u8>`: Generates a new secure cryptographic key.
- `.encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>>`: Encrypts a byte slice using the provided key.
- `.decrypt(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>>`: Decrypts encrypted data using the provided key.
