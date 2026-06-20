/// Defines the boilerplate for a context tool facade:
/// - Extension trait on `UserContext`
/// - Facade struct holding `&UserContext`
///
/// Usage:
/// ```ignore
/// define_context_facade!("std", time, ContextTimeExt, ContextTimeFacade);
/// ```
macro_rules! define_context_facade {
    ($feature:literal, $method_name:ident, $trait_name:ident, $facade_name:ident) => {
        #[cfg(feature = $feature)]
        pub trait $trait_name {
            fn $method_name(&self) -> $facade_name<'_>;
        }

        #[cfg(feature = $feature)]
        impl $trait_name for teaql_runtime::UserContext {
            fn $method_name(&self) -> $facade_name<'_> {
                $facade_name { ctx: self }
            }
        }

        #[cfg(feature = $feature)]
        pub struct $facade_name<'a> {
            #[allow(dead_code)]
            pub(crate) ctx: &'a teaql_runtime::UserContext,
        }
    };
}

/// Delegates methods from a facade to an underlying tool.
/// The tool is constructed via `Tool::new()` for each call.
///
/// Supports the following patterns:
///
/// **Simple delegation (no generics):**
/// ```ignore
/// delegate! { ToolType::new(),
///     fn method_name(&self) -> ReturnType;
///     fn method_name(&self, arg: Type) -> ReturnType;
///     fn method_name(&self, arg1: Type1, arg2: Type2) -> ReturnType;
/// }
/// ```
///
/// **With &mut self tool methods (for methods that take &mut args):**
/// These must be written manually, not through the macro.
macro_rules! delegate {
    ($tool_expr:expr, $( fn $method:ident(&self $(, $arg:ident : $ty:ty)*) -> $ret:ty );* $(;)?) => {
        $(
            #[inline]
            pub fn $method(&self $(, $arg: $ty)*) -> $ret {
                $tool_expr.$method($($arg),*)
            }
        )*
    };
}

macro_rules! delegate_comment {
    ($tool_expr:expr, $( fn $method:ident(&self $(, $arg:ident : $ty:ty)*) -> $ret:ty );* $(;)?) => {
        $(
            #[inline]
            pub fn $method(&self $(, $arg: $ty)*) -> teaql_tool_core::MustComment<$ret> {
                teaql_tool_core::MustComment::new($tool_expr.$method($($arg),*))
            }
        )*
    };
}

macro_rules! delegate_purpose {
    ($tool_expr:expr, $( fn $method:ident(&self $(, $arg:ident : $ty:ty)*) -> $ret:ty );* $(;)?) => {
        $(
            #[inline]
            pub fn $method(&self $(, $arg: $ty)*) -> teaql_tool_core::MustPurpose<$ret> {
                teaql_tool_core::MustPurpose::new($tool_expr.$method($($arg),*))
            }
        )*
    };
}

macro_rules! delegate_audit {
    ($tool_expr:expr, $( fn $method:ident(&self $(, $arg:ident : $ty:ty)*) -> $ret:ty );* $(;)?) => {
        $(
            #[inline]
            pub fn $method(&self $(, $arg: $ty)*) -> teaql_tool_core::MustAuditAs<$ret> {
                teaql_tool_core::MustAuditAs::new($tool_expr.$method($($arg),*))
            }
        )*
    };
}

macro_rules! delegate_res_comment {
    ($tool_expr:expr, $( fn $method:ident(&self $(, $arg:ident : $ty:ty)*) -> $ret:ty );* $(;)?) => {
        $(
            #[inline]
            pub fn $method(&self $(, $arg: $ty)*) -> teaql_tool_core::Result<teaql_tool_core::MustComment<$ret>> {
                $tool_expr.$method($($arg),*).map(teaql_tool_core::MustComment::new)
            }
        )*
    };
}

macro_rules! delegate_res_purpose {
    ($tool_expr:expr, $( fn $method:ident(&self $(, $arg:ident : $ty:ty)*) -> $ret:ty );* $(;)?) => {
        $(
            #[inline]
            pub fn $method(&self $(, $arg: $ty)*) -> teaql_tool_core::Result<teaql_tool_core::MustPurpose<$ret>> {
                $tool_expr.$method($($arg),*).map(teaql_tool_core::MustPurpose::new)
            }
        )*
    };
}

macro_rules! delegate_res_audit {
    ($tool_expr:expr, $( fn $method:ident(&self $(, $arg:ident : $ty:ty)*) -> $ret:ty );* $(;)?) => {
        $(
            #[inline]
            pub fn $method(&self $(, $arg: $ty)*) -> teaql_tool_core::Result<teaql_tool_core::MustAuditAs<$ret>> {
                $tool_expr.$method($($arg),*).map(teaql_tool_core::MustAuditAs::new)
            }
        )*
    };
}

pub(crate) use define_context_facade;
pub(crate) use delegate;
pub(crate) use delegate_comment;
pub(crate) use delegate_purpose;
pub(crate) use delegate_audit;
pub(crate) use delegate_res_comment;
pub(crate) use delegate_res_purpose;
pub(crate) use delegate_res_audit;
