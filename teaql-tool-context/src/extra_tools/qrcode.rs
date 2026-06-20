use crate::macros::*;

use teaql_tool_extra::qrcode::QrcodeTool;
use teaql_tool_core::Result;

define_context_facade!("extra", qrcode, ContextQrcodeExt, ContextQrcodeFacade);

#[cfg(feature = "extra")]
impl<'a> ContextQrcodeFacade<'a> {
    delegate_res_comment! { QrcodeTool::new(),
        fn generate_png(&self, data: &str, output_path: &str) -> ();
        fn generate_svg(&self, data: &str) -> String
    }
}
