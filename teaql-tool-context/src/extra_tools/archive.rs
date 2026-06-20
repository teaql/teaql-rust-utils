use crate::macros::*;

use teaql_tool_extra::archive::ArchiveTool;
use teaql_tool_core::Result;

define_context_facade!("extra", archive, ContextArchiveExt, ContextArchiveFacade);

#[cfg(feature = "extra")]
impl<'a> ContextArchiveFacade<'a> {
    delegate_res_comment! { ArchiveTool::new(),
        fn zip_dir(&self, src_dir: &str, dst_file: &str) -> ();
        fn unzip(&self, src_file: &str, dst_dir: &str) -> ()
    }
}
