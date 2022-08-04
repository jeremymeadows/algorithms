use std::ops::Not;

use crate::{Base, BigInt};

impl Not for BigInt {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        let mut i = 0;
        while i < self.data.len() - 1 {
            self.data[i] = !self.data[i];
            i += 1;
        }

        let shift =
            (Base::BITS - self.data[i].leading_zeros()) as i8 + if self >= 0 { 1 } else { -1 };
        let (mask, overflow) = (1 as Base).overflowing_shl(shift as u32);
        self.data[i] = !self.data[i] & (mask - 1);

        if overflow {
            self.data.push(1);
        }
        while self.data.ends_with(&[0]) && self.data.len() > 1 {
            self.data.pop();
        }

        self.signed ^= true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BaseExt;

    macro_rules! test_not {
        ($name:ident: $a:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!(!$a, $e);
            }
        };
    }

    test_not!(zero: BigInt::zero(), -1);

    test_not!(one: BigInt::one(), -2);

    test_not!(neg_one: BigInt::from(-1), 0);

    test_not!(neg_2: BigInt::from(-2), 1);

    test_not!(
        max: BigInt::from(Base::MAX),
        BigInt {
            signed: true,
            data: vec![0, 1]
        }
    );

    test_not!(max_inv: BigInt { signed: true, data: vec![0, 1] }, Base::MAX);

    test_not!(
        big: BigInt::from(BaseExt::MAX),
        BigInt {
            signed: true,
            data: vec![0, 0, 1]
        }
    );

    test_not!(big_inv: BigInt { signed: true, data: vec![0, 0, 1] }, BaseExt::MAX);
}
