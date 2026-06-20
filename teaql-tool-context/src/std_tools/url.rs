use crate::macros::*;

use teaql_tool_std::url::UrlTool;
use url::Url;
use teaql_tool_core::Result;

define_context_facade!("std", url, ContextUrlExt, ContextUrlFacade);

#[cfg(feature = "std")]
impl<'a> ContextUrlFacade<'a> {
    delegate_comment! { UrlTool::new(),
        fn encode(&self, text: &str) -> String
    }
    delegate_res_comment! { UrlTool::new(),
        fn parse(&self, url: &str) -> Url;
        fn decode(&self, text: &str) -> String
    }
}
