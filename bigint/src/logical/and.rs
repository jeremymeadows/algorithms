use std::mem;
use std::ops::{BitAnd, BitAndAssign};

use crate::BigInt;

impl BitAnd for BigInt {
    type Output = Self;

    fn bitand(mut self, other: Self) -> Self::Output {
        self &= other;
        self
    }
}

impl BitAndAssign for BigInt {
    fn bitand_assign(&mut self, mut other: Self) {
        if self.data.len() > other.data.len() {
            mem::swap(self, &mut other);
        }

        for i in 0..self.data.len() {
            if i < other.data.len() {
                self.data[i] &= other.data[i];
            }
        }
    }
}

macro_rules! impl_primitive_and {
    ($($t:ty),*) => {
        $(
            impl BitAnd<$t> for BigInt {
                type Output = Self;

                fn bitand(self, other: $t) -> Self::Output {
                    self & BigInt::from(other)
                }
            }

            impl BitAndAssign<$t> for BigInt {
                fn bitand_assign(&mut self, other: $t) {
                    *self = self.clone() & other;
                }
            }
        )*
    }
}

impl_primitive_and!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base, BaseExt};

    macro_rules! test_and {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a & $b, $e);
            }
        };
    }

    test_and!(zero: BigInt::one(), BigInt::zero(), 0);

    test_and!(one: BigInt::from(Base::MAX), BigInt::one(), 1);

    test_and!(big_zero: BigInt::from(BaseExt::MAX), BigInt::zero(), 0);
}
