use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use std::str::FromStr;
use teaql_tool_core::{MustComment, Result, TeaQLToolError};

pub struct DecimalTool;

impl DecimalTool {
    pub fn new() -> Self {
        Self
    }

    pub fn of(&self, s: &str) -> Result<MustComment<Decimal>> {
        Decimal::from_str(s)
            .map(MustComment::new)
            .map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn zero(&self) -> MustComment<Decimal> {
        MustComment::new(Decimal::ZERO)
    }

    pub fn one(&self) -> MustComment<Decimal> {
        MustComment::new(Decimal::ONE)
    }

    pub fn add(&self, a: Decimal, b: Decimal) -> MustComment<Decimal> {
        MustComment::new(a + b)
    }

    pub fn sub(&self, a: Decimal, b: Decimal) -> MustComment<Decimal> {
        MustComment::new(a - b)
    }

    pub fn mul(&self, a: Decimal, b: Decimal) -> MustComment<Decimal> {
        MustComment::new(a * b)
    }

    pub fn div(&self, a: Decimal, b: Decimal) -> Result<MustComment<Decimal>> {
        if b.is_zero() {
            Err(TeaQLToolError::InvalidArgument(
                "Division by zero".to_string(),
            ))
        } else {
            Ok(MustComment::new(a / b))
        }
    }

    pub fn round(&self, a: Decimal, dp: u32) -> MustComment<Decimal> {
        MustComment::new(a.round_dp_with_strategy(dp, RoundingStrategy::MidpointNearestEven))
    }

    pub fn ceil(&self, a: Decimal) -> MustComment<Decimal> {
        MustComment::new(a.ceil())
    }

    pub fn floor(&self, a: Decimal) -> MustComment<Decimal> {
        MustComment::new(a.floor())
    }

    pub fn abs(&self, a: Decimal) -> MustComment<Decimal> {
        MustComment::new(a.abs())
    }

    pub fn min(&self, a: Decimal, b: Decimal) -> MustComment<Decimal> {
        MustComment::new(a.min(b))
    }

    pub fn max(&self, a: Decimal, b: Decimal) -> MustComment<Decimal> {
        MustComment::new(a.max(b))
    }

    pub fn percent(&self, amount: Decimal, pct: Decimal) -> MustComment<Decimal> {
        MustComment::new((amount * pct) / Decimal::from(100))
    }

    pub fn ratio(&self, part: Decimal, total: Decimal) -> Result<MustComment<Decimal>> {
        self.div(part, total)
    }
}

impl Default for DecimalTool {
    fn default() -> Self {
        Self::new()
    }
}
