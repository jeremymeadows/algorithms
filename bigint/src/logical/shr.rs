use std::ops::{Shr, ShrAssign};

use crate::{Base, BigInt};

macro_rules! impl_primitive_shr {
    ($($t:ty),*) => {
        $(
            impl Shr<$t> for BigInt {
                type Output = Self;

                fn shr(mut self, len: $t) -> Self::Output {
                    self >>= len;
                    self
                }
            }

            impl ShrAssign<$t> for BigInt {
                fn shr_assign(&mut self, mut len: $t) {
                    const BITS: $t = Base::BITS as $t;

                    while len > 0 {
                        let mut v = vec![0; self.data.len()];
                        let l = std::cmp::min(BITS - 1, len);

                        for i in (0..(self.data.len())).rev() {
                            let val = self.data[i] >> l;
                            let carry = self.data[i] << BITS - l;

                            v[i] += val;
                            if i > 0 {
                                v[i - 1] = carry;
                            }
                        }
                        self.data = v;

                        while self.data.len() > 1 && self.data[self.data.len() - 1] == 0 {
                            self.data.pop();
                        }
                        len = len.saturating_sub(BITS - 1);
                    }
                }
            }
        )*
    }
}

impl_primitive_shr!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BaseExt;

    macro_rules! test_shr {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a >> $b, $e);
            }
        };
    }

    test_shr!(one: BigInt::from(0b10000), 4, 0b1);

    test_shr!(carry: BigInt::from((Base::MAX as BaseExt) << 1), 1, Base::MAX);

    test_shr!(overflow: BigInt::from(Base::MAX as BaseExt + 1), Base::BITS, 1);

    test_shr!(big: BigInt { signed: false, data: vec![Base::MAX, Base::MAX, 1] }, BaseExt::BITS, 1);
}
