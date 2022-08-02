use std::mem;
use std::ops::{Mul, MulAssign};

use crate::{Base, BigInt};

impl Mul for BigInt {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self *= other;
        self
    }
}

impl MulAssign<Self> for BigInt {
    fn mul_assign(&mut self, mut other: Self) {
        if self.data.len() < other.data.len() {
            mem::swap(self, &mut other);
        }

        if *self == 0 || other == 0 {
            *self = BigInt::from(0);
        } else if *self == 1 {
            *self = other;
        } else if other != 1 {
            let mut chunks = vec![0 as Base; self.data.len() + other.data.len()];
            let (mut prod, mut overflow);
            let mut carry = 0;

            for i in 0..(self.data.len()) {
                for j in 0..(other.data.len()) {
                    (prod, carry) = self.data[i].carrying_mul(other.data[j], carry);
                    (chunks[i], overflow) = chunks[i].carrying_add(prod, false);

                    let mut k = i + 1;
                    while overflow {
                        if k < chunks.len() {
                            (chunks[k], overflow) = chunks[k].carrying_add(0, overflow);
                        } else {
                            chunks.push(1);
                            overflow = false
                        }
                        k += 1;
                    }
                }
            }

            while chunks.ends_with(&[0]) && chunks.len() > 1 {
                chunks.pop();
            }

            if carry > 0 {
                chunks.push(carry);
            }

            self.data = chunks;
            self.signed = self.signed ^ other.signed;
        }
    }
}

impl Mul<&Self> for BigInt {
    type Output = Self;

    fn mul(mut self, other: &Self) -> Self::Output {
        self *= other;
        self
    }
}

impl MulAssign<&Self> for BigInt {
    fn mul_assign(&mut self, other: &Self) {
        *self *= other.clone();
    }
}

impl Mul<BigInt> for &BigInt {
    type Output = BigInt;

    fn mul(self, other: BigInt) -> Self::Output {
        self.clone() * other
    }
}

impl Mul<Self> for &BigInt {
    type Output = BigInt;

    fn mul(self, other: Self) -> Self::Output {
        self.clone() * other.clone()
    }
}

macro_rules! impl_primitive_mul {
    ($($t:ty),*) => {
        $(
            impl Mul<$t> for BigInt {
                type Output = Self;

                fn mul(self, other: $t) -> Self::Output {
                    self * BigInt::from(other)
                }
            }

            impl MulAssign<$t> for BigInt {
                fn mul_assign(&mut self, other: $t) {
                    *self = self.clone() * other;
                }
            }
        )*
    }
}

impl_primitive_mul!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_1_1() {
        let a = BigInt::from(1);
        let b = BigInt::from(1);
        let e = BigInt::from(1);
        assert_eq!(a * b, e);
    }

    #[test]
    fn mul_1_0() {
        let a = BigInt::from(1);
        let b = BigInt::from(0);
        let e = BigInt::from(0);
        assert_eq!(a * b, e);
    }

    #[test]
    fn mul_1_neg1() {
        let a = BigInt::from(1);
        let b = BigInt::from(-1);
        let e = BigInt::from(-1);
        assert_eq!(a * b, e);
    }

    #[test]
    fn mul_neg1_neg1() {
        let a = BigInt::from(-1);
        let b = BigInt::from(-1);
        let e = BigInt::from(1);
        assert_eq!(a * b, e);
    }

    #[test]
    fn mul_requiring_carry() {
        let a = BigInt::from(128);
        let b = BigInt::from(2);
        let e = BigInt::from(256);
        assert_eq!(a * b, e);
    }

    #[test]
    fn mul_small_small() {
        let a = BigInt::from(42);
        let b = BigInt::from(37);
        let e = BigInt::from(1554);
        assert_eq!(a * b, e);
    }

    #[test]
    fn mul_big_small() {
        let a = BigInt::from(0xfffffff);
        let b = BigInt::from(0xac);
        let e = BigInt::from(0xabfffff54_i64);
        assert_eq!(a * b, e);
    }

    #[test]
    fn mul_big_big() {
        let a = BigInt::from(0xfedcba9876543210_i128);
        let b = BigInt::from(0x1234567890abcdef_i128);
        let e = BigInt::from(0x11111111106ffffff_i128);
        assert_eq!(a + b, e);
    }
}
