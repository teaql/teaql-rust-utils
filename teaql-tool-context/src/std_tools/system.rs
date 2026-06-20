use crate::macros::*;

use teaql_tool_std::system::SystemTool;

define_context_facade!("std", system, ContextSystemExt, ContextSystemFacade);

#[cfg(feature = "std")]
impl<'a> ContextSystemFacade<'a> {
    delegate_comment! { SystemTool::new(),
        fn env(&self, key: &str) -> Option<String>;
        fn env_or(&self, key: &str, default: &str) -> String;
        fn os(&self) -> &'static str;
        fn arch(&self) -> &'static str;
        fn current_dir(&self) -> Option<String>
    }
}
