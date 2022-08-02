use std::mem;
use std::ops::{Sub, SubAssign};

use crate::BigInt;

impl Sub<Self> for BigInt {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;
        self
    }
}

impl SubAssign<Self> for BigInt {
    fn sub_assign(&mut self, mut other: Self) {
        if self.signed ^ other.signed {
            if self.signed {
                other.signed = true;
            } else if other.signed {
                other.signed = false;
            }
            *self += other;
        } else {
            if self.abs() < other.abs() {
                mem::swap(self, &mut other);
                self.signed = true;
            }

            let mut i = 0;
            while i < other.data.len() {
                let (mut diff, mut overflow) = self.data[i].overflowing_sub(other.data[i]);
                self.data[i] = diff;

                let mut j = i + 1;
                while overflow && j < other.data.len() {
                    (diff, overflow) = self.data[j].overflowing_sub(1);
                    self.data[j] = diff;

                    j += 1;
                }

                i += 1;
            }
        }
    }
}

impl Sub<&Self> for BigInt {
    type Output = Self;

    fn sub(mut self, other: &Self) -> Self::Output {
        self -= other;
        self
    }
}

impl SubAssign<&Self> for BigInt {
    fn sub_assign(&mut self, other: &Self) {
        *self -= other.clone();
    }
}

impl Sub<BigInt> for &BigInt {
    type Output = BigInt;

    fn sub(self, other: BigInt) -> Self::Output {
        self.clone() - other
    }
}

impl Sub<Self> for &BigInt {
    type Output = BigInt;

    fn sub(self, other: Self) -> Self::Output {
        self.clone() - other.clone()
    }
}

macro_rules! impl_primitive_sub {
    ($($t:ty),*) => {
        $(
            impl Sub<$t> for BigInt {
                type Output = Self;

                fn sub(self, other: $t) -> Self::Output {
                    self - BigInt::from(other)
                }
            }

            impl SubAssign<$t> for BigInt {
                fn sub_assign(&mut self, other: $t) {
                    *self = self.clone() - other;
                }
            }
        )*
    }
}

impl_primitive_sub!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

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
        let a = BigInt::from(0);
        let b = BigInt::from(1);
        let e = BigInt::from(-1);
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
