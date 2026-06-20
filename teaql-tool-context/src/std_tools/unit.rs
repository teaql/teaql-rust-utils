use crate::macros::*;

use teaql_tool_std::unit::UnitTool;

define_context_facade!("std", unit, ContextUnitExt, ContextUnitFacade);

#[cfg(feature = "std")]
impl<'a> ContextUnitFacade<'a> {
    delegate_comment! { UnitTool::new(),
        fn bytes_to_kb(&self, bytes: u64) -> f64;
        fn bytes_to_mb(&self, bytes: u64) -> f64;
        fn bytes_to_gb(&self, bytes: u64) -> f64;
        fn c_to_f(&self, c: f64) -> f64;
        fn f_to_c(&self, f: f64) -> f64
    }
}
