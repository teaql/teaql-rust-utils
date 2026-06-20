use crate::macros::*;

use teaql_tool_extra::phone::PhoneTool;
use teaql_tool_core::Result;

define_context_facade!("extra", phone, ContextPhoneExt, ContextPhoneFacade);

#[cfg(feature = "extra")]
impl<'a> ContextPhoneFacade<'a> {
    delegate_comment! { PhoneTool::new(),
        fn is_valid(&self, number: &str) -> bool
    }
    delegate_res_comment! { PhoneTool::new(),
        fn parse(&self, number: &str) -> phonenumber::PhoneNumber;
        fn format_international(&self, number: &str) -> String
    }
}
