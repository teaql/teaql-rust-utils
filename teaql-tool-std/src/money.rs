use iso_currency::Currency;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use std::str::FromStr;
use teaql_tool_core::{MustComment, Result, TeaQLToolError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

pub struct MoneyTool;

impl MoneyTool {
    pub fn new() -> Self {
        Self
    }

    pub fn of(&self, amount_str: &str, currency_code: &str) -> Result<MustComment<Money>> {
        let amount =
            Decimal::from_str(amount_str).map_err(|e| TeaQLToolError::ParseError(e.to_string()))?;
        let currency = Currency::from_code(currency_code).ok_or_else(|| {
            TeaQLToolError::InvalidArgument(format!("Invalid currency code: {}", currency_code))
        })?;
        Ok(MustComment::new(Money { amount, currency }))
    }

    pub fn zero(&self, currency_code: &str) -> Result<MustComment<Money>> {
        self.of("0", currency_code)
    }

    pub fn same_currency(&self, a: &Money, b: &Money) -> MustComment<bool> {
        MustComment::new(a.currency == b.currency)
    }

    fn check_currency(&self, a: &Money, b: &Money) -> Result<()> {
        if !self.same_currency(a, b).comment("internal check") {
            Err(TeaQLToolError::InvalidArgument(
                "Currency mismatch".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn add(&self, a: &Money, b: &Money) -> Result<MustComment<Money>> {
        self.check_currency(a, b)?;
        Ok(MustComment::new(Money {
            amount: a.amount + b.amount,
            currency: a.currency.clone(),
        }))
    }

    pub fn sub(&self, a: &Money, b: &Money) -> Result<MustComment<Money>> {
        self.check_currency(a, b)?;
        Ok(MustComment::new(Money {
            amount: a.amount - b.amount,
            currency: a.currency.clone(),
        }))
    }

    pub fn mul(&self, a: &Money, multiplier: Decimal) -> Result<MustComment<Money>> {
        Ok(MustComment::new(Money {
            amount: a.amount * multiplier,
            currency: a.currency.clone(),
        }))
    }

    pub fn div(&self, a: &Money, divisor: Decimal) -> Result<MustComment<Money>> {
        if divisor.is_zero() {
            Err(TeaQLToolError::InvalidArgument(
                "Division by zero".to_string(),
            ))
        } else {
            Ok(MustComment::new(Money {
                amount: a.amount / divisor,
                currency: a.currency.clone(),
            }))
        }
    }

    pub fn round(&self, a: &Money) -> Result<MustComment<Money>> {
        let exp = a.currency.exponent().unwrap_or(2) as u32;
        Ok(MustComment::new(Money {
            amount: a
                .amount
                .round_dp_with_strategy(exp, RoundingStrategy::MidpointNearestEven),
            currency: a.currency.clone(),
        }))
    }

    pub fn allocate(&self, a: &Money, ratios: Vec<u32>) -> Result<MustComment<Vec<Money>>> {
        if ratios.is_empty() {
            return Err(TeaQLToolError::InvalidArgument(
                "Ratios cannot be empty".to_string(),
            ));
        }
        let total_ratio: u32 = ratios.iter().sum();
        if total_ratio == 0 {
            return Err(TeaQLToolError::InvalidArgument(
                "Total ratio cannot be zero".to_string(),
            ));
        }

        let mut remainder = a.amount;
        let mut results = Vec::with_capacity(ratios.len());

        let exp = a.currency.exponent().unwrap_or(2) as u32;
        let total_decimal = Decimal::from(total_ratio);

        for (i, &ratio) in ratios.iter().enumerate() {
            if i == ratios.len() - 1 {
                results.push(Money {
                    amount: remainder,
                    currency: a.currency.clone(),
                });
            } else {
                let share = (a.amount * Decimal::from(ratio)) / total_decimal;
                let rounded =
                    share.round_dp_with_strategy(exp, RoundingStrategy::MidpointNearestEven);
                results.push(Money {
                    amount: rounded,
                    currency: a.currency.clone(),
                });
                remainder -= rounded;
            }
        }
        Ok(MustComment::new(results))
    }

    pub fn format(&self, a: &Money) -> MustComment<String> {
        MustComment::new(format!("{} {}", a.amount, a.currency.code()))
    }
}

impl Default for MoneyTool {
    fn default() -> Self {
        Self::new()
    }
}
