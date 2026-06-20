use crate::macros::*;

use teaql_tool_std::money::{MoneyTool, Money};
use rust_decimal::Decimal;
use teaql_tool_core::{MustPurpose, Result};

define_context_facade!("std", money, ContextMoneyExt, ContextMoneyFacade);

#[cfg(feature = "std")]
impl<'a> ContextMoneyFacade<'a> {
    delegate_comment! { MoneyTool::new(),
        fn same_currency(&self, a: &Money, b: &Money) -> bool;
        fn format(&self, a: &Money) -> String
    }
    delegate_res_comment! { MoneyTool::new(),
        fn of(&self, amount_str: &str, currency_code: &str) -> Money;
        fn zero(&self, currency_code: &str) -> Money;
        fn add(&self, a: &Money, b: &Money) -> Money;
        fn sub(&self, a: &Money, b: &Money) -> Money;
        fn mul(&self, a: &Money, multiplier: Decimal) -> Money;
        fn div(&self, a: &Money, divisor: Decimal) -> Money;
        fn round(&self, a: &Money) -> Money;
        fn allocate(&self, a: &Money, ratios: Vec<u32>) -> Vec<Money>
    }
}
