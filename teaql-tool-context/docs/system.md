# TeaQL Tool API: System Information

Use `ctx.system()` to access environment variables, operating system details, and directory paths.

## Required Dependencies

To use the system utilities, you must add `teaql-tool` to your project with the `std` feature enabled.

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## System Operations

```rust
// Read environment variables
let db_url = ctx.system().env("DATABASE_URL");
let host = ctx.system().env_or("HOST", "localhost");

// Get OS and architecture details
let os_name = ctx.system().os();
let arch_name = ctx.system().arch();

// Get the current working directory
if let Some(cwd) = ctx.system().current_dir() {
    println!("Current directory: {}", cwd);
}
```

## Key Methods
- `.env(key)`: Retrieves the value of an environment variable, returning `None` if not found.
- `.env_or(key, default)`: Retrieves an environment variable, returning the provided default string if it is not set.
- `.os()`: Returns a string representing the operating system (e.g., "linux", "macos", "windows").
- `.arch()`: Returns a string representing the CPU architecture (e.g., "x86_64", "aarch64").
- `.current_dir()`: Retrieves the current working directory path as a string.
