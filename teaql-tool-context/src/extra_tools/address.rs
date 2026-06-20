use crate::macros::*;

use teaql_tool_extra::address::AddressTool;

define_context_facade!("extra", address, ContextAddressExt, ContextAddressFacade);

#[cfg(feature = "extra")]
impl<'a> ContextAddressFacade<'a> {
    delegate_comment! { AddressTool::new(),
        fn extract_province(&self, address: &str) -> String
    }
}
