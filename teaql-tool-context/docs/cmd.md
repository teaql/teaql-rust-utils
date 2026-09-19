# TeaQL Tool API: Command Utilities

Use `ctx.cmd()` to execute external system commands.

> [!WARNING]
> Always provide a `.audit_as()` describing the auditing reason for executing the command.

## Required Dependencies

To use the Command utilities, you must add `teaql-tool` to your project with the `extra` feature enabled. Run the following command in your terminal:

```bash
cargo add teaql-tool-context --features std,extra
```

Or manually add it to your `Cargo.toml`:

```toml
teaql-tool-context = { version = "2.0.0", features = ["std", "extra"] }
```

## Run Command

```rust
// Returns a tuple of (stdout, stderr, exit_code)
let (stdout, stderr, exit_code) = ctx.cmd()
    .run_with_timeout("python3 process_data.py", 30)
    .audit_as("run external data processing script")?;
```

## Key Methods
- `.run_with_timeout(cmd_line, timeout_secs)`: Builds a deferred command action.
- `.audit_as(description)`: **(Required)** Describes the reason, executes the command, and returns its standard output, standard error, and exit status code.
