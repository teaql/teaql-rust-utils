# TeaQL Tool API: JWT Utilities

Use `ctx.jwt()` to sign and verify JSON Web Tokens (JWT).

## Required Dependencies

To use the JWT utilities and other extra features, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Example Usage

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String,
    exp: usize,
}

let my_claims = Claims {
    sub: "user_123".to_owned(),
    exp: 10000000000,
};

let secret = "my_super_secret_key";

// Sign a JWT
let token = ctx.jwt().sign(&my_claims, secret)?;
println!("Token: {}", token);

// Verify and decode a JWT
let decoded: Claims = ctx.jwt().verify(&token, secret)?;
println!("Decoded: {:?}", decoded);
```

## Key Methods
- `.sign(claims, secret)`: Sign claims into a JWT token using the given secret.
- `.verify(token, secret)`: Verify and decode a JWT token using the given secret.
