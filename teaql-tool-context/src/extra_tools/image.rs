use crate::macros::*;

use teaql_tool_extra::image::ImageTool;
use teaql_tool_core::Result;

define_context_facade!("extra", image, ContextImageExt, ContextImageFacade);

#[cfg(feature = "extra")]
impl<'a> ContextImageFacade<'a> {
    delegate_res_comment! { ImageTool::new(),
        fn resize(&self, input_path: &str, output_path: &str, width: u32, height: u32) -> ()
    }
}
