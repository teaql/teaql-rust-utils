use crate::macros::*;

use teaql_tool_std::decimal::DecimalTool;
use rust_decimal::Decimal;
use teaql_tool_core::{MustPurpose, Result};

define_context_facade!("std", decimal, ContextDecimalExt, ContextDecimalFacade);

#[cfg(feature = "std")]
impl<'a> ContextDecimalFacade<'a> {
    delegate_comment! { DecimalTool::new(),
        fn zero(&self) -> Decimal;
        fn one(&self) -> Decimal;
        fn add(&self, a: Decimal, b: Decimal) -> Decimal;
        fn sub(&self, a: Decimal, b: Decimal) -> Decimal;
        fn mul(&self, a: Decimal, b: Decimal) -> Decimal;
        fn round(&self, a: Decimal, dp: u32) -> Decimal;
        fn ceil(&self, a: Decimal) -> Decimal;
        fn floor(&self, a: Decimal) -> Decimal;
        fn abs(&self, a: Decimal) -> Decimal;
        fn min(&self, a: Decimal, b: Decimal) -> Decimal;
        fn max(&self, a: Decimal, b: Decimal) -> Decimal;
        fn percent(&self, amount: Decimal, pct: Decimal) -> Decimal
    }
    delegate_res_comment! { DecimalTool::new(),
        fn of(&self, s: &str) -> Decimal;
        fn div(&self, a: Decimal, b: Decimal) -> Decimal;
        fn ratio(&self, part: Decimal, total: Decimal) -> Decimal
    }
}
