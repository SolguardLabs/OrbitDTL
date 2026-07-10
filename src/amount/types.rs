use crate::error::{OrbitError, OrbitResult};
use serde::{Deserialize, Serialize};
use std::fmt;

pub const BPS_DENOMINATOR: u128 = 10_000;

pub type BasisPoints = u16;

#[derive(
    Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Amount(pub u128);

impl Amount {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> u128 {
        self.0
    }

    pub fn checked_add(self, other: Self) -> OrbitResult<Self> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(OrbitError::AmountOverflow)
    }

    pub fn checked_sub(self, other: Self) -> OrbitResult<Self> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(OrbitError::InsufficientBalance)
    }

    pub fn checked_mul(self, factor: u128) -> OrbitResult<Self> {
        self.0
            .checked_mul(factor)
            .map(Self)
            .ok_or(OrbitError::AmountOverflow)
    }

    pub fn checked_div(self, divisor: u128) -> OrbitResult<Self> {
        if divisor == 0 {
            return Err(OrbitError::AmountOverflow);
        }

        Ok(Self(self.0 / divisor))
    }

    pub fn saturating_sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }

    pub fn mul_bps_floor(self, bps: BasisPoints) -> OrbitResult<Self> {
        self.mul_ratio_floor(u128::from(bps), BPS_DENOMINATOR)
    }

    pub fn mul_ratio_floor(self, numerator: u128, denominator: u128) -> OrbitResult<Self> {
        if denominator == 0 {
            return Err(OrbitError::AmountOverflow);
        }

        self.0
            .checked_mul(numerator)
            .and_then(|value| value.checked_div(denominator))
            .map(Self)
            .ok_or(OrbitError::AmountOverflow)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
