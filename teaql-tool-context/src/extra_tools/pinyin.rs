use crate::macros::*;

use teaql_tool_extra::pinyin::PinyinTool;

define_context_facade!("extra", pinyin, ContextPinyinExt, ContextPinyinFacade);

#[cfg(feature = "extra")]
impl<'a> ContextPinyinFacade<'a> {
    delegate_comment! { PinyinTool::new(),
        fn get_pinyin(&self, text: &str) -> String
    }
}
