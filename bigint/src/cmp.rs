use std::cmp::Ordering;

use crate::BigInt;

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.signed == other.signed && self.data == other.data || self.data == [0] && other.data == [0]
    }
}

impl Eq for BigInt {}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord: Ordering;
        let sign = self.signed.cmp(&other.signed);

        if self == other {
            ord = Ordering::Equal;
        } else {
            match sign {
                Ordering::Less => {
                    ord = Ordering::Greater;
                }
                Ordering::Greater => {
                    ord = Ordering::Less;
                }
                Ordering::Equal => {
                    match self.signed {
                        false => {
                            match self.data.len().cmp(&other.data.len()) {
                                Ordering::Equal => {
                                    ord = self.data.cmp(&other.data);
                                }
                                Ordering::Less => {
                                    ord = Ordering::Less;
                                }
                                Ordering::Greater => {
                                    ord = Ordering::Greater;
                                }
                            }
                        }
                        true => {
                            match self.data.len().cmp(&other.data.len()) {
                                Ordering::Equal => {
                                    ord = self.data.cmp(&other.data).reverse();
                                }
                                Ordering::Less => {
                                    ord = Ordering::Greater;
                                }
                                Ordering::Greater => {
                                    ord = Ordering::Less;
                                }
                            }
                        }
                    }
                }
            }
        }

        ord
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base, BaseExt};

    #[test]
    fn eq() {
        let a = BigInt::from(42);
        let b = BigInt::from(42);

        assert_eq!(a, b);
    }

    #[test]
    fn eq_neg() {
        let a = BigInt::from(-42);
        let b = BigInt::from(-42);

        assert_eq!(a, b);
    }

    #[test]
    fn ne() {
        let a = BigInt::from(42);
        let b = BigInt::from(64);

        assert_ne!(a, b);
    }

    #[test]
    fn ne_sign() {
        let a = BigInt::from(42);
        let b = BigInt::from(-42);

        assert_ne!(a, b);
    }

    #[test]
    fn eq_0_neg0() {
        let a = BigInt::from(0);
        let mut b = a.clone();
        b.signed = true;

        assert_eq!(a, b);
    }

    #[test]
    fn eq_long() {
        let a = BigInt::from(BaseExt::MAX);
        let b = BigInt::from(BaseExt::MAX);

        assert_eq!(a, b);
    }

    #[test]
    fn ord() {
        let a = BigInt::from(1);
        let b = BigInt::from(8);

        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn ord_neg() {
        let a = BigInt::from(-8);
        let b = BigInt::from(-1);

        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn ord_pos_neg() {
        let a = BigInt::from(-8);
        let b = BigInt::from(1);

        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn ord_size() {
        let a = BigInt::from(1);
        let b = BigInt::from(Base::MAX as BaseExt + 1);

        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn ord_long() {
        let a = BigInt::from(Base::MAX as BaseExt + 1);
        let b = BigInt::from(Base::MAX as BaseExt + 2);

        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn ord_neg_long() {
        let a = BigInt::from(Base::MIN as BaseExt + 2) * -1;
        let b = BigInt::from(Base::MIN as BaseExt + 1) * -1;

        assert!(a < b);
        assert!(b > a);
    }
}
