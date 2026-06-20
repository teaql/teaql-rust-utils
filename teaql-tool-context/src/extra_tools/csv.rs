use crate::macros::*;

use teaql_tool_extra::csv::CsvTool;
use teaql_tool_core::Result;

define_context_facade!("extra", csv, ContextCsvExt, ContextCsvFacade);

#[cfg(feature = "extra")]
impl<'a> ContextCsvFacade<'a> {
    delegate_res_comment! { CsvTool::new(),
        fn parse(&self, data: &str) -> Vec<Vec<String>>;
        fn generate(&self, records: &[Vec<String>]) -> String
    }
}
