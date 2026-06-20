use crate::macros::*;

use teaql_tool_extra::clipboard::ClipboardTool;
use teaql_tool_core::Result;

define_context_facade!("extra", clipboard, ContextClipboardExt, ContextClipboardFacade);

#[cfg(feature = "extra")]
impl<'a> ContextClipboardFacade<'a> {
    delegate_res_comment! { ClipboardTool::new(),
        fn read_text(&self) -> String;
        fn write_text(&self, text: &str) -> ()
    }
}
