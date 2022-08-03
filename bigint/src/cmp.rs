use std::cmp::Ordering;

use crate::BigInt;

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.data == [0] && other.data == [0]
            || self.signed == other.signed && self.data == other.data
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.signed ^ other.signed {
            self.signed.cmp(&other.signed).reverse()
        } else {
            let a = &self.data;
            let b = &other.data;

            let ord = a.len().cmp(&b.len()).then_with(|| {
                for i in (0..(a.len())).rev() {
                    let ord = a[i].cmp(&b[i]);
                    if ord.is_ne() {
                        return ord;
                    }
                }
                unreachable!()
            });

            if !self.signed {
                ord
            } else {
                ord.reverse()
            }
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! impl_primitive_cmp {
    ($($t:ty),*) => {
        $(
            impl PartialEq<$t> for BigInt {
                fn eq(&self, other: &$t) -> bool {
                    *self == BigInt::from(*other)
                }
            }

            impl PartialOrd<$t> for BigInt {
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    Some(self.cmp(&BigInt::from(*other)))
                }
            }
        )*
    }
}

impl_primitive_cmp!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Base;

    mod eq {
        use super::*;

        macro_rules! test_eq {
            ($name:ident: $a:expr, $b:expr) => {
                #[test]
                fn $name() {
                    assert_eq!($a, $b);
                    assert_eq!($a.cmp(&$b), Ordering::Equal);
                }
            };
        }

        macro_rules! test_ne {
            ($name:ident: $a:expr, $b:expr) => {
                #[test]
                fn $name() {
                    assert_ne!($a, $b);
                    assert_ne!($a.cmp(&$b), Ordering::Equal);
                }
            };
        }

        test_eq!(eq: BigInt::from(42), BigInt::from(42));

        test_eq!(eq_neg: BigInt::from(-42), BigInt::from(-42));

        test_ne!(pos_ne_neg: BigInt::from(42), BigInt::from(-42));

        test_ne!(ne: BigInt::from(42), BigInt::from(64));

        test_eq!(zeroes: BigInt::zero(), BigInt::zero());

        test_eq!(zero_eq_neg_zero: BigInt::zero(), BigInt { signed: true, data: vec![0] });

        test_eq!(big_eq: BigInt::from(0xfedcba9876543210_u128), BigInt::from(0xfedcba9876543210_u128));

        test_ne!(big_ne: BigInt::from(0xfedcba9876543210_u128), BigInt::from(-0xfedcba9876543210_i128));
    }

    mod ord {
        use super::*;

        macro_rules! test_ord {
            ($name:ident: $a:expr, $b:expr) => {
                #[test]
                fn $name() {
                    assert!($a < $b);
                    assert!($b > $a);
                    assert_eq!($a.cmp(&$b), Ordering::Less);
                    assert_eq!($b.cmp(&$a), Ordering::Greater);
                }
            };
        }

        test_ord!(ord: BigInt::one(), BigInt::from(2));

        test_ord!(ord_neg: BigInt::from(-2), BigInt::from(-1));

        test_ord!(neg_lt_pos: BigInt::from(-2), BigInt::one());

        test_ord!(diff_data_size:
            BigInt { signed: false, data: vec![Base::MAX] },
            BigInt { signed: false, data: vec![1, Base::MAX] }
        );

        test_ord!(diff_data_size_neg:
            BigInt { signed: true, data: vec![1, Base::MAX] },
            BigInt { signed: true, data: vec![Base::MAX] }
        );

        test_ord!(big_ord: BigInt::from(0x1234567890abcdef_u128), BigInt::from(0xfedcba9876543210_u128));
    }
}
