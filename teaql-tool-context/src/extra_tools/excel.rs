use crate::macros::*;

use teaql_tool_extra::excel::ExcelTool;
use teaql_tool_core::Result;

define_context_facade!("extra", excel, ContextExcelExt, ContextExcelFacade);

#[cfg(feature = "extra")]
impl<'a> ContextExcelFacade<'a> {
    delegate_res_comment! { ExcelTool::new(),
        fn write_simple(&self, path: &str, data: &[Vec<String>]) -> ();
        fn read_simple(&self, path: &str) -> Vec<Vec<String>>
    }
}
