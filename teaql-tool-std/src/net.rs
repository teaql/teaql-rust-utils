use std::net::{IpAddr, Ipv4Addr, TcpListener, UdpSocket};
use teaql_tool_core::{Result, TeaQLToolError};

/// Network utility wrapper
pub struct NetTool;

impl NetTool {
    pub fn new() -> Self {
        Self
    }

    /// Quickly get the local IPv4 address by attempting a dummy UDP connection
    pub fn get_local_ipv4(&self) -> Result<String> {
        let socket = UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to bind socket: {}", e)))?;
        socket.connect("8.8.8.8:80")
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to connect socket: {}", e)))?;
        let addr = socket.local_addr()
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to get local address: {}", e)))?;
        Ok(addr.ip().to_string())
    }

    /// Check if a local port is usable (not occupied)
    pub fn is_usable_local_port(&self, port: u16) -> bool {
        match TcpListener::bind(("127.0.0.1", port)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Check if an IP address string is an internal/private IP
    pub fn is_inner_ip(&self, ip_str: impl AsRef<str>) -> bool {
        let ip: IpAddr = match ip_str.as_ref().parse() {
            Ok(ip) => ip,
            Err(_) => return false,
        };

        match ip {
            IpAddr::V4(ipv4) => {
                let octets = ipv4.octets();
                // 10.0.0.0/8
                if octets[0] == 10 { return true; }
                // 172.16.0.0/12
                if octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31 { return true; }
                // 192.168.0.0/16
                if octets[0] == 192 && octets[1] == 168 { return true; }
                // 127.0.0.0/8 (Loopback is generally considered private/internal)
                if octets[0] == 127 { return true; }
                false
            }
            IpAddr::V6(ipv6) => {
                let segments = ipv6.segments();
                // Unique Local Addresses (fc00::/7)
                if (segments[0] & 0xfe00) == 0xfc00 { return true; }
                // Loopback
                if ipv6 == std::net::Ipv6Addr::LOCALHOST { return true; }
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_tool() {
        let tool = NetTool::new();
        // port 0 is always assignable
        assert!(tool.is_usable_local_port(0));
        assert!(tool.is_inner_ip("192.168.1.10"));
        assert!(tool.is_inner_ip("10.0.0.5"));
        assert!(tool.is_inner_ip("127.0.0.1"));
        assert!(!tool.is_inner_ip("8.8.8.8"));
    }
}
