use crate::macros::*;

use teaql_tool_std::i18n::I18nTool;

define_context_facade!("std", i18n, ContextI18nExt, ContextI18nFacade);

// Note: I18nTool has stateful &mut self methods (set_default_locale, add, load_json)
// which cannot be delegated through the facade pattern.
// Only the read-only methods (t, tf) are exposed here.
// For mutable operations, construct an I18nTool directly.

#[cfg(feature = "std")]
impl<'a> ContextI18nFacade<'a> {
    delegate_comment! { I18nTool::new(),
        fn t(&self, locale: &str, key: &str) -> String;
        fn tf(&self, locale: &str, key: &str, args: &[(&str, &str)]) -> String
    }
}
