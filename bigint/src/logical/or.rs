use std::mem;
use std::ops::{BitOr, BitOrAssign};

use crate::BigInt;

impl BitOr for BigInt {
    type Output = Self;

    fn bitor(mut self, other: Self) -> Self::Output {
        self |= other;
        self
    }
}

impl BitOrAssign for BigInt {
    fn bitor_assign(&mut self, mut other: Self) {
        if self.data.len() < other.data.len() {
            mem::swap(self, &mut other);
        }

        for i in 0..self.data.len() {
            if i < other.data.len() {
                self.data[i] |= other.data[i];
            }
        }
    }
}

macro_rules! impl_primitive_or {
    ($($t:ty),*) => {
        $(
            impl BitOr<$t> for BigInt {
                type Output = Self;

                fn bitor(self, other: $t) -> Self::Output {
                    self | BigInt::from(other)
                }
            }

            impl BitOrAssign<$t> for BigInt {
                fn bitor_assign(&mut self, other: $t) {
                    *self = self.clone() | other;
                }
            }
        )*
    }
}

impl_primitive_or!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Base;

    macro_rules! test_or {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a | $b, $e);
            }
        };
    }

    test_or!(one: BigInt::one(), BigInt::zero(), 1);

    test_or!(max: BigInt::from(Base::MAX), BigInt::zero(), Base::MAX);
}
