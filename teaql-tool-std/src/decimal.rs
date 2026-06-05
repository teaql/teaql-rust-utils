use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use std::str::FromStr;
use teaql_tool_core::{MustPurpose, Result, TeaQLToolError};

pub struct DecimalTool;

impl DecimalTool {
    pub fn new() -> Self {
        Self
    }

    pub fn of(&self, s: &str) -> Result<MustPurpose<Decimal>> {
        Decimal::from_str(s)
            .map(MustPurpose::new)
            .map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn zero(&self) -> MustPurpose<Decimal> {
        MustPurpose::new(Decimal::ZERO)
    }

    pub fn one(&self) -> MustPurpose<Decimal> {
        MustPurpose::new(Decimal::ONE)
    }

    pub fn add(&self, a: Decimal, b: Decimal) -> MustPurpose<Decimal> {
        MustPurpose::new(a + b)
    }

    pub fn sub(&self, a: Decimal, b: Decimal) -> MustPurpose<Decimal> {
        MustPurpose::new(a - b)
    }

    pub fn mul(&self, a: Decimal, b: Decimal) -> MustPurpose<Decimal> {
        MustPurpose::new(a * b)
    }

    pub fn div(&self, a: Decimal, b: Decimal) -> Result<MustPurpose<Decimal>> {
        if b.is_zero() {
            Err(TeaQLToolError::InvalidArgument(
                "Division by zero".to_string(),
            ))
        } else {
            Ok(MustPurpose::new(a / b))
        }
    }

    pub fn round(&self, a: Decimal, dp: u32) -> MustPurpose<Decimal> {
        MustPurpose::new(a.round_dp_with_strategy(dp, RoundingStrategy::MidpointNearestEven))
    }

    pub fn ceil(&self, a: Decimal) -> MustPurpose<Decimal> {
        MustPurpose::new(a.ceil())
    }

    pub fn floor(&self, a: Decimal) -> MustPurpose<Decimal> {
        MustPurpose::new(a.floor())
    }

    pub fn abs(&self, a: Decimal) -> MustPurpose<Decimal> {
        MustPurpose::new(a.abs())
    }

    pub fn min(&self, a: Decimal, b: Decimal) -> MustPurpose<Decimal> {
        MustPurpose::new(a.min(b))
    }

    pub fn max(&self, a: Decimal, b: Decimal) -> MustPurpose<Decimal> {
        MustPurpose::new(a.max(b))
    }

    pub fn percent(&self, amount: Decimal, pct: Decimal) -> MustPurpose<Decimal> {
        MustPurpose::new((amount * pct) / Decimal::from(100))
    }

    pub fn ratio(&self, part: Decimal, total: Decimal) -> Result<MustPurpose<Decimal>> {
        self.div(part, total)
    }
}

impl Default for DecimalTool {
    fn default() -> Self {
        Self::new()
    }
}
