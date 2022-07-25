use std::ops::Mul;

use crate::{Base, BaseExt, BigInt};

impl Mul for BigInt {
    type Output = Self;

    fn mul(mut self, mut other: Self) -> Self {
        if self.data.len() < other.data.len() {
            std::mem::swap(&mut self, &mut other);
        }

        if other == 0.into() {
            return BigInt::from(0);
        }

        let mut chunks = vec![0 as Base; self.data.len() + other.data.len()];
        let mut carry: Base = 0;

        for i in 0..(self.data.len()) {
            for j in 0..(other.data.len()) {
                let prod = self.data[i] as BaseExt * other.data[j] as BaseExt + carry as BaseExt;
                chunks[i] += (prod % (Base::MAX as BaseExt + 1)) as Base;
                carry = (prod / (Base::MAX as BaseExt + 1)) as Base;
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

        self
    }
}

macro_rules! impl_mul {
    ($($t:ty),*) => {
        $(
            impl Mul<$t> for BigInt {
                type Output = Self;

                fn mul(self, other: $t) -> Self::Output {
                    self * BigInt::from(other)
                }
            }
        )*
    }
}

impl_mul!(u8, u16, u32, u64, u128);
impl_mul!(i8, i16, i32, i64, i128);

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
