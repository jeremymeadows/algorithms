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

    /// returns the number of bits representing the number, ignoring leading zeros
    pub fn bits(&self) -> usize {
        let len = self.data.len();
        len * Base::BITS as usize - self.data[len - 1].leading_zeros() as usize
    }

    pub fn count_ones(&self) -> Self {
        self.data
            .iter()
            .fold(BigInt::zero(), |acc, x| acc + x.count_ones())
    }

    pub fn count_zeroes(&self) -> Self {
        self.data
            .iter()
            .fold(BigInt::zero(), |acc, x| acc + x.count_zeros())
    }

    pub fn trailing_ones(&self) -> Self {
        let mut x = BigInt::zero();
        for i in self.data.iter() {
            let b = i.trailing_ones();
            x += b;
            if b != Base::BITS {
                break;
            }
        }
        x
    }

    pub fn trailing_zeroes(&self) -> Self {
        let mut x = BigInt::zero();
        for i in self.data.iter() {
            let b = i.trailing_zeros();
            x += b;
            if b != Base::BITS {
                break;
            }
        }
        x
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
