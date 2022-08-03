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

            let mut carry = false;
            let mut i = 0;

            while i < other.data.len() {
                (self.data[i], carry) = self.data[i].carrying_add(other.data[i], carry);

                i += 1;
            }

            while i < self.data.len() {
                (self.data[i], carry) = self.data[i].carrying_add(0, carry);

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
    use crate::{Base, BaseExt};

    macro_rules! test_add {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a + $b, $e);
            }
        };
    }

    test_add!(one_one: BigInt::one(), BigInt::one(), 2);

    test_add!(one_zero: BigInt::one(), BigInt::zero(), 1);

    test_add!(zero_zero: BigInt::zero(), BigInt::zero(), 0);

    test_add!(carry: BigInt::from(Base::MAX), BigInt::one(), Base::MAX as BaseExt + 1);

    test_add!(big:
        BigInt::from(BaseExt::MAX),
        BigInt::from(BaseExt::MAX),
        BigInt { signed: false, data: vec![Base::MAX - 1, Base::MAX, 1] }
    );

    mod negative {
        use super::*;

        test_add!(two_neg_one: BigInt::from(2), BigInt::from(-1), 1);

        test_add!(one_neg_two: BigInt::one(), BigInt::from(-2), -1);

        test_add!(neg_one_two: BigInt::from(-1), BigInt::from(2), 1);

        test_add!(neg_two_one: BigInt::from(-2), BigInt::one(), -1);

        test_add!(neg_one_neg_one: BigInt::from(-1), BigInt::from(-1), -2);

        test_add!(big_inv:
            BigInt::from(Base::MAX),
            BigInt { signed: true, data: vec![Base::MAX] },
            0
        );

        test_add!(big_neg_big:
            BigInt::from(Base::MAX - 1),
            BigInt { signed: true, data: vec![(Base::MAX - 1) / 2] },
            (Base::MAX - 1) / 2
        );
    }
}
