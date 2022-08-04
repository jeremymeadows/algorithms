use std::mem;
use std::ops::{BitXor, BitXorAssign};

use crate::BigInt;

impl BitXor for BigInt {
    type Output = Self;

    fn bitxor(mut self, other: Self) -> Self::Output {
        self ^= other;
        self
    }
}

impl BitXorAssign for BigInt {
    fn bitxor_assign(&mut self, mut other: Self) {
        if self.data.len() > other.data.len() {
            mem::swap(self, &mut other);
        }

        for i in 0..self.data.len() {
            if i < other.data.len() {
                self.data[i] ^= other.data[i];
            }
        }
        while self.data.ends_with(&[0]) && self.data.len() > 1 {
            self.data.pop();
        }
    }
}

macro_rules! impl_primitive_xor {
    ($($t:ty),*) => {
        $(
            impl BitXor<$t> for BigInt {
                type Output = Self;

                fn bitxor(self, other: $t) -> Self::Output {
                    self ^ BigInt::from(other)
                }
            }

            impl BitXorAssign<$t> for BigInt {
                fn bitxor_assign(&mut self, other: $t) {
                    *self = self.clone() ^ other;
                }
            }
        )*
    }
}

impl_primitive_xor!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Base;

    macro_rules! test_xor {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a ^ $b, $e);
            }
        };
    }

    test_xor!(zero: BigInt::zero(), BigInt::zero(), 0);

    test_xor!(one: BigInt::one(), BigInt::zero(), 1);

    test_xor!(ones: BigInt::one(), BigInt::one(), 0);

    test_xor!(
        zero_max: BigInt::from(Base::MAX),
        BigInt::from(Base::MAX),
        0
    );

    test_xor!(seven: BigInt::from(3u8), BigInt::from(4u8), 7);

    test_xor!(eight: BigInt::from(12u8), BigInt::from(4u8), 8);
}
