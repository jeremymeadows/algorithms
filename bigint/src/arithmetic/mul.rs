use std::mem;
use std::ops::{Mul, MulAssign};

use crate::{Base, BigInt};

impl BigInt {
    pub fn pow(&self, exp: &BigInt) -> Self {
        if exp == 0 {
            return BigInt::one();
        }

        let base = self.clone();
        let mut val = self.clone();
        let mut exp = exp.clone();

        while exp > 0 {
            val *= base.clone();
            exp -= 1;
        }
        val
    }
}

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

        if self == 0 || other == 0 {
            *self = BigInt::zero();
        } else if self.abs() == 1 {
            self.signed = self.signed ^ other.signed;
            self.data = other.data;
        } else if other.abs() != 1 {
            let mut chunks = vec![0 as Base; self.data.len() + other.data.len()];
            let (mut prod, mut carry, mut overflow);

            for i in 0..(self.data.len()) {
                for j in 0..(other.data.len()) {
                    (prod, carry) = self.data[i].carrying_mul(other.data[j], 0);

                    let mut k = i + j;
                    (chunks[k], overflow) = chunks[k].overflowing_add(prod);
                    k += 1;
                    (chunks[k], overflow) = chunks[k].carrying_add(carry, overflow);

                    while overflow {
                        k += 1;
                        (chunks[k], overflow) = chunks[k].carrying_add(0, overflow);
                    }
                }
            }

            while chunks.ends_with(&[0]) && chunks.len() > 1 {
                chunks.pop();
            }

            self.signed = self.signed ^ other.signed;
            self.data = chunks;
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
    use crate::BaseExt;

    macro_rules! test_mul {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a * $b, $e);
            }
        };
    }

    test_mul!(one_one: BigInt::one(), BigInt::one(), 1);

    test_mul!(one_zero: BigInt::one(), BigInt::zero(), 0);

    test_mul!(one_neg_one: BigInt::one(), BigInt::from(-1), -1);

    test_mul!(neg_one_neg_one: BigInt::from(-1), BigInt::from(-1), 1);

    test_mul!(small_small: BigInt::from(42u8), BigInt::from(6u8), 252u8);

    test_mul!(
        carry: BigInt::from(Base::MAX),
        BigInt::from(2),
        Base::MAX as BaseExt * 2
    );

    test_mul!(
        big: BigInt::from(Base::MAX),
        BigInt::from(Base::MAX),
        Base::MAX as BaseExt * Base::MAX as BaseExt
    );

    test_mul!(
        bigger: BigInt::from(BaseExt::MAX),
        BigInt::from(BaseExt::MAX),
        BigInt {
            signed: false,
            data: vec![1, 0, Base::MAX - 1, Base::MAX]
        }
    );
}
