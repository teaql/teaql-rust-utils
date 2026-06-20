# TeaQL Tool API: Network Utilities

Use `ctx.net()` to interact with network configurations, check for available ports, and inspect IP addresses.

## Required Dependencies

To use the network utilities, you must add `teaql-tool` to your project with the `std` feature enabled.

```toml
teaql-tool = { version = "1.0", features = ["std"] }
```

## Network Operations

```rust
// Check if a local port is free and usable
let is_free = ctx.net().is_usable_local_port(8080);

// Get the local IPv4 address
let local_ip = ctx.net().get_local_ipv4()?;

// Check if an IP address is an internal/private IP
let is_private = ctx.net().is_inner_ip("192.168.1.100");
```

## Key Methods
- `.is_usable_local_port(port)`: Returns `true` if the given local port is available to bind.
- `.get_local_ipv4()`: Retrieves the local machine's primary IPv4 address.
- `.is_inner_ip(ip_str)`: Determines whether a given IP address string is an internal (private) network IP.
