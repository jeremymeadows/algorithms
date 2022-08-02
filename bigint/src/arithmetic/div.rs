use std::ops::{Div, DivAssign};

use crate::BigInt;

impl Div for BigInt {
    type Output = Self;

    fn div(mut self, other: Self) -> Self {
        self /= other;
        self
    }
}

impl DivAssign<Self> for BigInt {
    fn div_assign(&mut self, other: Self) {
        assert!(other != 0, "attempt to divide by 0");

        if *self == 0 || *self == 1 && other.abs() > 1 {
            self.data = vec![0];
        } else if self.abs() == other.abs() {
            self.data = vec![1];
        } else if other.abs() != 1 {
            todo!();
        }

        self.signed = self.signed ^ other.signed;
    }
}

impl Div<&Self> for BigInt {
    type Output = Self;

    fn div(mut self, other: &Self) -> Self::Output {
        self /= other;
        self
    }
}

impl DivAssign<&Self> for BigInt {
    fn div_assign(&mut self, other: &Self) {
        *self /= other.clone();
    }
}

impl Div<BigInt> for &BigInt {
    type Output = BigInt;

    fn div(self, other: BigInt) -> Self::Output {
        self.clone() / other
    }
}

impl Div<Self> for &BigInt {
    type Output = BigInt;

    fn div(self, other: Self) -> Self::Output {
        self.clone() / other.clone()
    }
}

macro_rules! impl_primitive_div {
    ($($t:ty),*) => {
        $(
            impl Div<$t> for BigInt {
                type Output = Self;

                fn div(self, other: $t) -> Self::Output {
                    self / BigInt::from(other)
                }
            }

            impl DivAssign<$t> for BigInt {
                fn div_assign(&mut self, other: $t) {
                    *self = self.clone() / other;
                }
            }
        )*
    }
}

impl_primitive_div!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_1_1() {
        let a = BigInt::from(1);
        let b = BigInt::from(1);
        let e = BigInt::from(1);
        assert_eq!(a / b, e);
    }

    #[test]
    fn div_2_2() {
        let a = BigInt::from(2);
        let b = BigInt::from(2);
        let e = BigInt::from(1);
        assert_eq!(a / b, e);
    }

    #[test]
    fn div_1_neg1() {
        let a = BigInt::from(1);
        let b = BigInt::from(-1);
        let e = BigInt::from(-1);
        assert_eq!(a / b, e);
    }

    #[test]
    fn div_neg1_neg1() {
        let a = BigInt::from(-1);
        let b = BigInt::from(-1);
        let e = BigInt::from(1);
        assert_eq!(a / b, e);
    }

    #[test]
    fn div_big_small() {
        let a = BigInt::from(0xfffffff);
        let b = BigInt::from(0xac);
        let e = BigInt::from(0x17d05f);
        assert_eq!(a / b, e);
    }

    #[test]
    fn div_big_big() {
        let a = BigInt::from(0xfedcba9876543210_i128);
        let b = BigInt::from(0x1234567890abcdef_i128);
        let e = BigInt::from(0xe);
        assert_eq!(a / b, e);
    }
}
