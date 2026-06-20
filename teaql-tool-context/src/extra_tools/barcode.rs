use crate::macros::*;

use teaql_tool_extra::barcode::BarcodeTool;
use teaql_tool_core::Result;

define_context_facade!("extra", barcode, ContextBarcodeExt, ContextBarcodeFacade);

#[cfg(feature = "extra")]
impl<'a> ContextBarcodeFacade<'a> {
    delegate_res_comment! { BarcodeTool::new(),
        fn generate_code128_png(&self, data: &str, output_path: &str) -> ();
        fn generate_code128_svg(&self, data: &str) -> String
    }
}
