use std::mem;
use std::ops::BitOr;

use crate::BigInt;

impl BitOr for BigInt {
    type Output = Self;

    fn bitor(mut self, mut other: Self) -> Self::Output {
        if self.data.len() < other.data.len() {
            mem::swap(&mut self, &mut other);
        }

        for i in 0..self.data.len() {
            if i < other.data.len() {
                self.data[i] |= other.data[i];
            }
        }

        self
    }
}

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
