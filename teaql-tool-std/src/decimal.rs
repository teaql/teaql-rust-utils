use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use std::str::FromStr;
use teaql_tool_core::{Result, TeaQLToolError};

pub struct DecimalTool;

impl DecimalTool {
    pub fn new() -> Self {
        Self
    }

    pub fn of(&self, s: &str) -> Result<Decimal> {
        Decimal::from_str(s).map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn zero(&self) -> Decimal {
        Decimal::ZERO
    }

    pub fn one(&self) -> Decimal {
        Decimal::ONE
    }

    pub fn add(&self, a: Decimal, b: Decimal) -> Decimal {
        a + b
    }

    pub fn sub(&self, a: Decimal, b: Decimal) -> Decimal {
        a - b
    }

    pub fn mul(&self, a: Decimal, b: Decimal) -> Decimal {
        a * b
    }

    pub fn div(&self, a: Decimal, b: Decimal) -> Result<Decimal> {
        if b.is_zero() {
            Err(TeaQLToolError::InvalidArgument(
                "Division by zero".to_string(),
            ))
        } else {
            Ok(a / b)
        }
    }

    pub fn round(&self, a: Decimal, dp: u32) -> Decimal {
        a.round_dp_with_strategy(dp, RoundingStrategy::MidpointNearestEven)
    }

    pub fn ceil(&self, a: Decimal) -> Decimal {
        a.ceil()
    }

    pub fn floor(&self, a: Decimal) -> Decimal {
        a.floor()
    }

    pub fn abs(&self, a: Decimal) -> Decimal {
        a.abs()
    }

    pub fn min(&self, a: Decimal, b: Decimal) -> Decimal {
        a.min(b)
    }

    pub fn max(&self, a: Decimal, b: Decimal) -> Decimal {
        a.max(b)
    }

    pub fn percent(&self, amount: Decimal, pct: Decimal) -> Decimal {
        (amount * pct) / Decimal::from(100)
    }

    pub fn ratio(&self, part: Decimal, total: Decimal) -> Result<Decimal> {
        self.div(part, total)
    }
}

impl Default for DecimalTool {
    fn default() -> Self {
        Self::new()
    }
}
