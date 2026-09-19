# TeaQL Tool API: Email Utilities

Use `ctx.email()` to send emails.

> [!WARNING]
> Always provide a `.audit_as()` describing the auditing reason for sending the email.

## Required Dependencies

To use the Email utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool-context --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool-context = { version = "2.0.0", features = ["std", "extra"] }
```

## Send Email

```rust
ctx.email()
    .send(
        "smtp.example.com:587",
        "username",
        "password",
        "no-reply@example.com",
        "user@domain.com",
        "Welcome to our service!",
        "Hello, thank you for signing up!"
    )
    .audit_as("send welcome email to new user")?;
```

## Key Methods
- `.send(server, user, pass, from, to, subject, body)`: Builds a deferred email action.
- `.audit_as(description)`: **(Required)** Describes the reason and sends the email.
