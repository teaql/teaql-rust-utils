use crate::macros::*;

use teaql_tool_extra::config::ConfigTool;
use teaql_tool_core::Result;

define_context_facade!("extra", config, ContextConfigExt, ContextConfigFacade);

#[cfg(feature = "extra")]
impl<'a> ContextConfigFacade<'a> {
    delegate_res_comment! { ConfigTool::new(),
        fn load_env(&self) -> ();
        fn get_env(&self, key: &str) -> String
    }
}
