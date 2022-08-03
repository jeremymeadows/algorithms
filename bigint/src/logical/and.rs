use std::mem;
use std::ops::BitAnd;

use crate::BigInt;

impl BitAnd for BigInt {
    type Output = Self;

    fn bitand(mut self, mut other: Self) -> Self::Output {
        if self.data.len() > other.data.len() {
            mem::swap(&mut self, &mut other);
        }

        for i in 0..self.data.len() {
            if i < other.data.len() {
                self.data[i] &= other.data[i];
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Base;

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
}
