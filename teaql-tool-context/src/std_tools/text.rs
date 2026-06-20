use crate::macros::*;

use teaql_tool_std::text::TextTool;
use teaql_tool_core::MustPurpose;

define_context_facade!("std", text, ContextTextExt, ContextTextFacade);

#[cfg(feature = "std")]
impl<'a> ContextTextFacade<'a> {
    delegate_comment! { TextTool::new(),
        fn trim(&self, s: &str) -> String;
        fn lowercase(&self, s: &str) -> String;
        fn uppercase(&self, s: &str) -> String;
        fn capitalize(&self, s: &str) -> String;
        fn to_snake_case(&self, s: &str) -> String;
        fn to_camel_case(&self, s: &str) -> String;
        fn to_pascal_case(&self, s: &str) -> String;
        fn to_kebab_case(&self, s: &str) -> String;
        fn normalize_whitespace(&self, s: &str) -> String;
        fn contains(&self, s: &str, substring: &str) -> bool;
        fn starts_with(&self, s: &str, prefix: &str) -> bool;
        fn ends_with(&self, s: &str, suffix: &str) -> bool
    }
}
