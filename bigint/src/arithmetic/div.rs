use std::ops::{Div, DivAssign};

use crate::BigInt;

impl BigInt {
    pub(crate) fn div_rem(mut self, other: Self) -> (BigInt, BigInt) {
        assert!(other != 0, "attempt to divide by zero");
        let mut rem = BigInt::zero();

        if self.abs() < other.abs() {
            rem = self.clone();
            self = BigInt::zero();
        } else if self.abs() == other.abs() {
            self.signed = self.signed ^ other.signed;
            self.data = vec![1];
        } else if other.abs() != 1 {
            let mut quot = BigInt::zero();

            for i in (0..(self.bits())).rev() {
                rem <<= 1;
                rem |= (self.clone() >> i) & 1;

                if rem >= other.abs() {
                    rem -= other.abs();
                    quot |= BigInt::one() << i;
                }
            }
            while self.data.ends_with(&[0]) && self.data.len() > 1 {
                self.data.pop();
            }

            rem.signed = self.signed;
            self.signed = self.signed ^ other.signed;
            self.data = quot.data;
        }

        (self, rem)
    }
}

impl Div for BigInt {
    type Output = Self;

    fn div(mut self, other: Self) -> Self {
        self /= other;
        self
    }
}

impl DivAssign<Self> for BigInt {
    fn div_assign(&mut self, other: Self) {
        (*self, _) = self.clone().div_rem(other);
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

impl_primitive_div!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base, BaseExt};

    macro_rules! test_div {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a / $b, $e);
                assert_eq!($b * ($a / $b) + $a % $b, $a);
            }
        };
    }

    test_div!(zero: BigInt::zero(), BigInt::from(2), 0);

    test_div!(one_one: BigInt::one(), BigInt::one(), 1);

    test_div!(two_two: BigInt::from(2), BigInt::from(2), 1);

    test_div!(one_neg_one: BigInt::one(), BigInt::from(-1), -1);

    test_div!(neg_two_neg_two: BigInt::from(-2), BigInt::from(-2), 1);

    test_div!(small_small: BigInt::from(252), BigInt::from(6), 42);

    test_div!(negative: BigInt::from(-7), BigInt::from(-3), 2);

    test_div!(both_pos: BigInt::from(7), BigInt::from(3), 2);

    test_div!(
        carry: BigInt::from(Base::MAX),
        BigInt::from(2),
        Base::MAX / 2
    );

    test_div!(
        big: BigInt::from(BaseExt::MAX),
        BigInt::from(0x1234),
        BaseExt::MAX / 0x1234
    );

    test_div!(bigger:
        BigInt { signed: false, data: vec![1, 0, Base::MAX - 1, Base::MAX] },
        BigInt::from(BaseExt::MAX),
        BigInt::from(BaseExt::MAX)
    );

    #[test]
    #[should_panic = "attempt to divide by zero"]
    fn divide_by_zero() {
        _ = BigInt::one() / BigInt::zero();
    }
}
