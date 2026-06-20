use crate::macros::*;

use teaql_tool_std::net::NetTool;
use teaql_tool_core::Result;

define_context_facade!("std", net, ContextNetExt, ContextNetFacade);

#[cfg(feature = "std")]
impl<'a> ContextNetFacade<'a> {
    delegate_comment! { NetTool::new(),
        fn is_usable_local_port(&self, port: u16) -> bool
    }
    delegate_res_comment! { NetTool::new(),
        fn get_local_ipv4(&self) -> String
    }

    /// Check if an IP address string is an internal/private IP
    pub fn is_inner_ip(&self, ip_str: impl AsRef<str>) -> bool {
        NetTool::new().is_inner_ip(ip_str)
    }
}
