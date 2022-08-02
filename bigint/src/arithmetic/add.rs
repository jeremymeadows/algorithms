use std::mem;
use std::ops::{Add, AddAssign};

use crate::BigInt;

impl Add<Self> for BigInt {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign<Self> for BigInt {
    fn add_assign(&mut self, mut other: Self) {
        if self.signed ^ other.signed {
            if self.signed {
                mem::swap(self, &mut other);
            }
            other.signed = false;

            *self -= other;
        } else {
            if *self < other {
                mem::swap(self, &mut other);
            }

            let mut sum;
            let mut carry = false;

            let mut i = 0;
            while i < other.data.len() {
                (sum, carry) = self.data[i].carrying_add(other.data[i], carry);
                self.data[i] = sum;

                i += 1;
            }

            while i < self.data.len() {
                (sum, carry) = self.data[i].carrying_add(0, carry);
                self.data[i] = sum;

                i += 1;
            }

            if carry {
                self.data.push(1);
            }
        }
    }
}

impl Add<&Self> for BigInt {
    type Output = Self;

    fn add(mut self, other: &Self) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign<&Self> for BigInt {
    fn add_assign(&mut self, other: &Self) {
        *self += other.clone();
    }
}

impl Add<BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> Self::Output {
        self.clone() + other
    }
}

impl Add<Self> for &BigInt {
    type Output = BigInt;

    fn add(self, other: Self) -> Self::Output {
        self.clone() + other.clone()
    }
}

macro_rules! impl_primitave_add {
    ($($t:ty),*) => {
        $(
            impl Add<$t> for BigInt {
                type Output = Self;

                fn add(self, other: $t) -> Self::Output {
                    self + BigInt::from(other)
                }
            }

            impl AddAssign<$t> for BigInt {
                fn add_assign(&mut self, other: $t) {
                    *self = self.clone() + other;
                }
            }
        )*
    }
}

impl_primitave_add!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_1_1() {
        let a = BigInt::from(1);
        let b = BigInt::from(1);
        let e = BigInt::from(2);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_0_0() {
        let a = BigInt::from(1);
        let b = BigInt::from(1);
        let e = BigInt::from(2);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_requiring_carry() {
        let a = BigInt::from(255);
        let b = BigInt::from(1);
        let e = BigInt::from(256);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_big_small() {
        let a = BigInt::from(0xfffffff);
        let b = BigInt::from(0xac);
        let e = BigInt::from(0x100000ab);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_big_big() {
        let a = BigInt::from(0xfedcba9876543210_i128);
        let b = BigInt::from(0x1234567890abcdef_i128);
        let e = BigInt::from(0x11111111106ffffff_i128);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_big_big_neg() {
        let a = BigInt::from(0xfedcba9876543210_i128);
        let b = BigInt::from(-0x1234567890abcdef_i128);
        let e = BigInt::from(0xeca8641fe5a86421_i128);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_1_neg1() {
        let a = BigInt::from(1);
        let b = BigInt::from(-1);
        let e = BigInt::from(0);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_2_neg1() {
        let a = BigInt::from(2);
        let b = BigInt::from(-1);
        let e = BigInt::from(1);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_1_neg2() {
        let a = BigInt::from(1);
        let b = BigInt::from(-2);
        let e = BigInt::from(-1);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_neg1_2() {
        let a = BigInt::from(-1);
        let b = BigInt::from(2);
        let e = BigInt::from(1);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_neg2_1() {
        let a = BigInt::from(-2);
        let b = BigInt::from(1);
        let e = BigInt::from(-1);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_neg1_neg1() {
        let a = BigInt::from(-1);
        let b = BigInt::from(-1);
        let e = BigInt::from(-2);
        assert_eq!(a + b, e);
    }
}
