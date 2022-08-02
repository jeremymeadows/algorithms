use std::ops::Not;

use crate::{Base, BigInt};

impl Not for BigInt {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        let mut i = 0;
        while i < self.data.len() - 1 {
            self.data[i] = !self.data[i];
            i += 1;
        }

        let shift =
            (Base::BITS - self.data[i].leading_zeros()) as i8 + if self >= 0 { 1 } else { -1 };
        let (mask, overflow) = (1 as Base).overflowing_shl(shift as u32);
        self.data[i] = !self.data[i] & (mask - 1);

        if overflow {
            self.data.push(1);
        }

        self.signed ^= true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_0() {
        assert_eq!(!BigInt::from(0), -1);
    }

    #[test]
    fn not_1() {
        assert_eq!(!BigInt::from(1), -2);
    }

    #[test]
    fn not_neg_1() {
        assert_eq!(!BigInt::from(-1), 0);
    }

    #[test]
    fn not_neg_2() {
        assert_eq!(!BigInt::from(-2), 1);
    }

    #[test]
    fn not_max() {
        assert_eq!(
            !BigInt::from(Base::MAX),
            BigInt {
                signed: true,
                data: vec![0, 1]
            }
        );
    }

    #[test]
    fn not_max_max() {
        assert_eq!(
            !BigInt {
                signed: false,
                data: vec![Base::MAX, 1]
            },
            BigInt {
                signed: true,
                data: vec![0, 2]
            }
        );
    }
}
