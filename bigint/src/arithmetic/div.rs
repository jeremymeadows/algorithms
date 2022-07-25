use std::ops::Div;

use crate::{BigInt};

impl Div for BigInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self
    }
}

macro_rules! impl_div {
    ($($t:ty),*) => {
        $(
            impl Div<$t> for BigInt {
                type Output = Self;

                fn div(self, other: $t) -> Self::Output {
                    self / BigInt::from(other)
                }
            }
        )*
    }
}

impl_div!(u8, u16, u32, u64, u128);
impl_div!(i8, i16, i32, i64, i128);

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
