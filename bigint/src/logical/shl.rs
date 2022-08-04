use std::ops::{Shl, ShlAssign};

use crate::{Base, BigInt};

macro_rules! impl_primitive_shl {
    ($($t:ty),*) => {
        $(
            impl Shl<$t> for BigInt {
                type Output = Self;

                fn shl(mut self, len: $t) -> Self::Output {
                    self <<= len;
                    self
                }
            }

            impl ShlAssign<$t> for BigInt {
                fn shl_assign(&mut self, mut len: $t) {
                    const BITS: $t = Base::BITS as $t;

                    while len > 0 {
                        let mut v = vec![0; self.data.len()];
                        let l = len.clamp(0, BITS - 1);

                        for i in 0..(self.data.len()) {
                            let val = self.data[i] << l;
                            let carry = self.data[i] >> BITS - l;

                            v[i] += val;
                            if carry > 0 {
                                if v.len() == i + 1 {
                                    v.push(0);
                                }
                                v[i + 1] = carry;
                            }
                        }
                        self.data = v;

                        len = len.saturating_sub(BITS - 1);
                    }
                }
            }
        )*
    }
}

impl_primitive_shl!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BaseExt;

    macro_rules! test_shl {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a << $b, $e);
            }
        };
    }

    test_shl!(one: BigInt::from(0b1), 4, 0b10000);

    test_shl!(
        carry: BigInt::from(Base::MAX),
        1,
        (Base::MAX as BaseExt) << 1
    );

    test_shl!(
        overflow: BigInt::one(),
        Base::BITS,
        Base::MAX as BaseExt + 1
    );

    test_shl!(
        big: BigInt::one(),
        BaseExt::BITS,
        BigInt {
            signed: false,
            data: vec![0, 0, 1]
        }
    );
}
