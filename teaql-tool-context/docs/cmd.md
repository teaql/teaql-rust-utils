# TeaQL Tool API: Command Utilities

Use `ctx.cmd()` to execute external system commands.

> [!WARNING]
> Always provide a `.audit_as()` describing the auditing reason for executing the command.

## Required Dependencies

To use the Command utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool = { version = "1.0", features = ["std", "extra"] }
```

## Run Command

```rust
// Returns a tuple of (stdout, stderr, exit_code)
let (stdout, stderr, exit_code) = ctx.cmd()
    .audit_as("run external data processing script")
    .run_with_timeout("python3 process_data.py", 30)?;
```

## Key Methods
- `.audit_as(description)`: **(Required)** Describe the auditing reason for running this external command.
- `.run_with_timeout(cmd_line: &str, timeout_secs: u64) -> Result<(String, String, i32)>`: Executes a shell command with the specified timeout in seconds, returning its standard output, standard error, and exit status code.
