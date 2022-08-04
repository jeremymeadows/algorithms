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
            other.signed = self.signed;
            *self += other;
        } else {
            if self.abs() < other.abs() {
                mem::swap(self, &mut other);
                self.signed ^= true;
            }

            let mut overflow;
            let mut i = 0;

            while i < other.data.len() {
                (self.data[i], overflow) = self.data[i].overflowing_sub(other.data[i]);

                let mut j = i + 1;
                while overflow && j < self.data.len() {
                    (self.data[j], overflow) = self.data[j].overflowing_sub(1);
                    j += 1;
                }

                i += 1;
            }
        }

        while self.data.ends_with(&[0]) && self.data.len() > 1 {
            self.data.pop();
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
    use crate::{Base, BaseExt};

    macro_rules! test_sub {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a - $b, $e);
            }
        };
    }

    test_sub!(one_one: BigInt::one(), BigInt::one(), 0);

    test_sub!(one_zero: BigInt::one(), BigInt::zero(), 1);

    test_sub!(zero_one: BigInt::zero(), BigInt::one(), -1);

    test_sub!(zero_zero: BigInt::zero(), BigInt::zero(), 0);

    test_sub!(carry: BigInt::from(Base::MAX as BaseExt + 1), BigInt::one(), Base::MAX);

    test_sub!(big:
        BigInt { signed: false, data: vec![Base::MAX - 1, Base::MAX, 1] },
        BigInt::from(BaseExt::MAX),
        BaseExt::MAX
    );

    mod negative {
        use super::*;

        test_sub!(two_neg_one: BigInt::from(2), BigInt::from(-1), 3);

        test_sub!(one_neg_two: BigInt::one(), BigInt::from(-2), 3);

        test_sub!(neg_one_two: BigInt::from(-1), BigInt::from(2), -3);

        test_sub!(neg_two_one: BigInt::from(-2), BigInt::one(), -3);

        test_sub!(neg_one_neg_one: BigInt::from(-1), BigInt::from(-1), 0);

        test_sub!(
            big_inv: BigInt::from(Base::MAX),
            BigInt {
                signed: true,
                data: vec![Base::MAX]
            },
            Base::MAX as BaseExt * 2
        );

        test_sub!(big_neg_big:
            BigInt::from(Base::MAX - 1),
            BigInt { signed: true, data: vec![(Base::MAX - 1) / 2] },
            Base::MAX as BaseExt - 1 + ((Base::MAX - 1) / 2) as BaseExt
        );
    }
}
