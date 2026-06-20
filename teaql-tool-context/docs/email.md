# TeaQL Tool API: Email Utilities

Use `ctx.email()` to send emails.

> [!WARNING]
> Always provide a `.audit_as()` describing the auditing reason for sending the email.

## Required Dependencies

To use the Email utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Send Email

```rust
ctx.email()
    .audit_as("send welcome email to new user")
    .send(
        "smtp.example.com:587",
        "username",
        "password",
        "no-reply@example.com",
        "user@domain.com",
        "Welcome to our service!",
        "Hello, thank you for signing up!"
    )?;
```

## Key Methods
- `.audit_as(description)`: **(Required)** Describe the auditing reason for sending this email.
- `.send(server: &str, user: &str, pass: &str, from: &str, to: &str, subject: &str, body: &str) -> Result<()>`: Sends an email using the specified SMTP server and credentials.
