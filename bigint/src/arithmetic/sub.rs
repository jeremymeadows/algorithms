use std::ops::Sub;

use crate::BigInt;

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, mut other: Self) -> Self {
        other.signed = !other.signed;

        self + other
    }
}

macro_rules! impl_sub {
    ($($t:ty),*) => {
        $(
            impl Sub<$t> for BigInt {
                type Output = Self;

                fn sub(self, other: $t) -> Self::Output {
                    self - BigInt::from(other)
                }
            }
        )*
    }
}

impl_sub!(u8, u16, u32, u64, u128);
impl_sub!(i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_1_1() {
        let a = BigInt::from(1);
        let b = BigInt::from(1);
        let e = BigInt::from(0);
        assert_eq!(a - b, e);
    }

    #[test]
    fn sub_1_0() {
        let a = BigInt::from(1);
        let b = BigInt::from(0);
        let e = BigInt::from(1);
        assert_eq!(a - b, e);
    }

    #[test]
    fn sub_1_neg1() {
        let a = BigInt::from(1);
        let b = BigInt::from(-1);
        let e = BigInt::from(2);
        assert_eq!(a - b, e);
    }

    #[test]
    fn sub_neg1_neg1() {
        let a = BigInt::from(-1);
        let b = BigInt::from(-1);
        let e = BigInt::from(0);
        assert_eq!(a - b, e);
    }

    #[test]
    fn sub_requiring_carry() {
        let a = BigInt::from(1);
        let b = BigInt::from(258);
        let e = BigInt::from(-257);
        assert_eq!(a - b, e);
    }

    #[test]
    fn sub_big_small() {
        let a = BigInt::from(0xfffffff);
        let b = BigInt::from(0xac);
        let e = BigInt::from(0xfffff53);
        assert_eq!(a - b, e);
    }

    #[test]
    fn sub_big_big() {
        let a = BigInt::from(0xfedcba9876543210_i128);
        let b = BigInt::from(0x1234567890abcdef_i128);
        let e = BigInt::from(0xeca8641fe5a86421_i128);
        assert_eq!(a - b, e);
    }

    #[test]
    fn sub_big_big_negative() {
        let a = BigInt::from(0x1234567890abcdef_i128);
        let b = BigInt::from(0xfedcba9876543210_i128);
        let e = BigInt::from(-0xeca8641fe5a86421_i128);
        assert_eq!(a - b, e);
    }
}
