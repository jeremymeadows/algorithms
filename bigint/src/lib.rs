#![feature(bigint_helper_methods)]

pub mod arithmetic;
pub mod cmp;
pub mod convert;
pub mod fmt;
pub mod logical;

#[cfg(target_pointer_width = "64")]
type Base = u64;
#[cfg(all(test, target_pointer_width = "64"))]
type BaseExt = u128;

#[cfg(not(target_pointer_width = "64"))]
type Base = u32;
#[cfg(all(test, not(target_pointer_width = "64")))]
type BaseExt = u64;

#[derive(Clone, Debug, Eq)]
pub struct BigInt {
    signed: bool,
    data: Vec<Base>,
}

impl BigInt {
    pub fn new() -> Self {
        BigInt::zero()
    }

    pub fn zero() -> Self {
        Self {
            signed: false,
            data: vec![0],
        }
    }

    pub fn one() -> Self {
        Self {
            signed: false,
            data: vec![1],
        }
    }

    pub fn is_positive(&self) -> bool {
        *self > 0
    }

    pub fn is_negative(&self) -> bool {
        *self < 0
    }
}

impl Default for BigInt {
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_positive() {
        assert!(BigInt::from(1).is_positive());
        assert!(!BigInt::from(1).is_negative());
    }

    #[test]
    fn is_negative() {
        assert!(BigInt::from(-1).is_negative());
        assert!(!BigInt::from(-1).is_positive());
    }

    #[test]
    fn is_zero() {
        assert!(!BigInt::from(0).is_positive());
        assert!(!BigInt::from(0).is_negative());
    }
}
