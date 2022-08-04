use std::ops::{Rem, RemAssign};

use crate::BigInt;

impl Rem for BigInt {
    type Output = Self;

    fn rem(mut self, other: Self) -> Self {
        self %= other;
        self
    }
}

impl RemAssign<Self> for BigInt {
    fn rem_assign(&mut self, other: Self) {
        (_, *self) = self.clone().div_rem(other);
    }
}

impl Rem<&Self> for BigInt {
    type Output = Self;

    fn rem(mut self, other: &Self) -> Self::Output {
        self %= other;
        self
    }
}

impl RemAssign<&Self> for BigInt {
    fn rem_assign(&mut self, other: &Self) {
        *self %= other.clone();
    }
}

impl Rem<BigInt> for &BigInt {
    type Output = BigInt;

    fn rem(self, other: BigInt) -> Self::Output {
        self.clone() % other
    }
}

impl Rem<Self> for &BigInt {
    type Output = BigInt;

    fn rem(self, other: Self) -> Self::Output {
        self.clone() % other.clone()
    }
}

macro_rules! impl_primitive_rem {
    ($($t:ty),*) => {
        $(
            impl Rem<$t> for BigInt {
                type Output = Self;

                fn rem(self, other: $t) -> Self::Output {
                    self % BigInt::from(other)
                }
            }

            impl RemAssign<$t> for BigInt {
                fn rem_assign(&mut self, other: $t) {
                    *self = self.clone() % other;
                }
            }
        )*
    }
}

impl_primitive_rem!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base, BaseExt};

    macro_rules! test_rem {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a % $b, $e);
            }
        };
    }

    test_rem!(zero: BigInt::zero(), BigInt::from(2), 0);

    test_rem!(all: BigInt::from(0xfu8), BigInt::from(0x10u8), 0xf);

    test_rem!(even: BigInt::from(0x10u8), BigInt::from(2), 0);

    test_rem!(odd: BigInt::from(0x11u8), BigInt::from(2), 1);

    test_rem!(small: BigInt::from(0xffu8), BigInt::from(0x42u8), 0x39u8);

    test_rem!(
        big: BigInt::from(Base::MAX),
        BigInt::from(Base::MAX / 4),
        Base::MAX % (Base::MAX / 4)
    );

    test_rem!(
        bigger: BigInt::from(BaseExt::MAX),
        BigInt::from(Base::MAX / 4),
        BaseExt::MAX % (Base::MAX as BaseExt / 4)
    );
}
