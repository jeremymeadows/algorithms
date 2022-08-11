#![feature(bigint_helper_methods)]
#![feature(let_chains)]
#![feature(step_trait)]

// #[cfg(num)]
use num_bigint;

// #[cfg(num)]
fn to_num_bigint(x: &BigInt) -> num_bigint::BigInt {
    num_bigint::BigInt::from_bytes_le(
        if x.signed {
            num_bigint::Sign::Minus
        } else {
            num_bigint::Sign::Plus
        },
        &x.to_le_bytes(),
    )
}

pub mod arithmetic;
pub mod cmp;
pub mod convert;
pub mod fmt;
pub mod logical;
pub mod misc;

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
    /// Creates a `BigInt` with the value of `0`.
    pub fn zero() -> Self {
        Self {
            signed: false,
            data: vec![0],
        }
    }

    /// Creates a `BigInt` with the value of `1`.
    pub fn one() -> Self {
        Self {
            signed: false,
            data: vec![1],
        }
    }

    /// Returns true if `self` is even.
    pub fn is_even(&self) -> bool {
        self.data[0] & 1 == 0
    }

    /// Returns true if `self` is odd.
    pub fn is_odd(&self) -> bool {
        self.data[0] & 1 == 1
    }

    /// Returns true if `self` is greater than `0`.
    pub fn is_positive(&self) -> bool {
        *self > 0
    }

    /// Returns true if `self` is less than `0`.
    pub fn is_negative(&self) -> bool {
        *self < 0
    }

    /// Returns `1` is `is_positive()`, `-1` if `is_negative()`, or `0` otherwise.
    pub fn signum(&self) -> i8 {
        if self.data == &[0] {
            0
        } else {
            if self.signed {
                -1
            } else {
                1
            }
        }
    }

    /// Returns the number of bits representing the number, ignoring leading zeros.
    pub fn bits(&self) -> usize {
        let len = self.data.len();
        len * Base::BITS as usize - self.data[len - 1].leading_zeros() as usize
    }

    /// Returns the number of ones in the binary representation of the number.
    pub fn count_ones(&self) -> usize {
        self.data
            .iter()
            .fold(0, |acc, x| acc + x.count_ones() as usize)
    }

    /// Returns the number of zeros in the binary representation of the number, ignoring leading zeros.
    pub fn count_zeros(&self) -> usize {
        self.bits() - self.count_ones()
    }

    /// Returns the number of trailing ones in the binary representation of the number.
    pub fn trailing_ones(&self) -> usize {
        let mut x = 0usize;
        for i in self.data.iter() {
            let b = i.trailing_ones();
            x += b as usize;
            if b != Base::BITS {
                break;
            }
        }
        x
    }

    /// Returns the number of trailing zeros in the binary representation of the number.
    pub fn trailing_zeros(&self) -> usize {
        let mut x = 0usize;
        for i in self.data.iter() {
            let b = i.trailing_zeros();
            x += b as usize;
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
