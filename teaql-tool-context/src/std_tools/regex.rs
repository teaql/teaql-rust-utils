use crate::macros::*;

use teaql_tool_std::regex::RegexTool;
use teaql_tool_core::Result;

define_context_facade!("std", regex, ContextRegexExt, ContextRegexFacade);

#[cfg(feature = "std")]
impl<'a> ContextRegexFacade<'a> {
    delegate_comment! { RegexTool::new(),
        fn escape(&self, text: &str) -> String;
        fn validate(&self, pattern: &str) -> bool
    }
    delegate_res_comment! { RegexTool::new(),
        fn is_match(&self, pattern: &str, text: &str) -> bool;
        fn find(&self, pattern: &str, text: &str) -> Option<String>;
        fn find_all(&self, pattern: &str, text: &str) -> Vec<String>;
        fn replace(&self, pattern: &str, text: &str, rep: &str) -> String;
        fn replace_all(&self, pattern: &str, text: &str, rep: &str) -> String;
        fn split(&self, pattern: &str, text: &str) -> Vec<String>
    }
}
