use std::cmp::{self, Ordering};
use std::ops::{Add, AddAssign};

use crate::{Base, BaseExt, BigInt};

impl Add for BigInt {
    type Output = Self;

    fn add(mut self, mut other: Self) -> Self {
        let mut carry: Base = 0;
        let size = cmp::max(self.data.len(), other.data.len());
        let mut greater_neg = false;

        self.data.resize(size, 0);
        other.data.resize(size, 0);

        if self.signed ^ other.signed {
            if self.signed {
                std::mem::swap(&mut self, &mut other);
            }

            match self.clone().abs().cmp(&other.clone().abs()) {
                Ordering::Equal => {
                    return Self {
                        signed: false,
                        data: vec![0],
                    };
                }
                Ordering::Less => {
                    println!("grater_neg");
                    greater_neg = true;
                }
                Ordering::Greater => {}
            }

            other = !other;
        }

        for i in 0..size {
            let sum = self.data[i] as BaseExt + other.data[i] as BaseExt + carry as BaseExt;
            self.data[i] = (sum % (Base::MAX as BaseExt + 1)) as Base;
            carry = (sum / (Base::MAX as BaseExt + 1)) as Base;
        }

        if self.signed ^ other.signed && carry > 0 {
            self = self + BigInt::from(1u8);
        } else if carry > 0 {
            self.data.push(carry);
        }

        if greater_neg {
            self.signed = true;
            println!("grater_neg");
            // self = !self;
        }

        self
    }
}

impl AddAssign<Self> for BigInt {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

macro_rules! impl_add {
    ($($t:ty),*) => {
        $(
            impl Add<$t> for BigInt {
                type Output = Self;

                fn add(self, other: $t) -> Self::Output {
                    self + BigInt::from(other)
                }
            }

            impl AddAssign<$t> for BigInt {
                fn add_assign(&mut self, other: $t) {
                    *self = self.clone() + other;
                }
            }
        )*
    }
}

impl_add!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_1_1() {
        let a = BigInt::from(1);
        let b = BigInt::from(1);
        let e = BigInt::from(2);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_0_0() {
        let a = BigInt::from(1);
        let b = BigInt::from(1);
        let e = BigInt::from(2);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_requiring_carry() {
        let a = BigInt::from(255);
        let b = BigInt::from(1);
        let e = BigInt::from(256);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_big_small() {
        let a = BigInt::from(0xfffffff);
        let b = BigInt::from(0xac);
        let e = BigInt::from(0x100000ab);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_big_big() {
        let a = BigInt::from(0xfedcba9876543210_i128);
        let b = BigInt::from(0x1234567890abcdef_i128);
        let e = BigInt::from(0x11111111106ffffff_i128);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_big_big_neg() {
        let a = BigInt::from(0xfedcba9876543210_i128);
        let b = BigInt::from(-0x1234567890abcdef_i128);
        let e = BigInt::from(0xeca8641fe5a86421_i128);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_1_neg1() {
        let a = BigInt::from(1);
        let b = BigInt::from(-1);
        let e = BigInt::from(0);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_2_neg1() {
        let a = BigInt::from(2);
        let b = BigInt::from(-1);
        let e = BigInt::from(1);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_1_neg2() {
        let a = BigInt::from(1);
        let b = BigInt::from(-2);
        let e = BigInt::from(-1);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_neg1_2() {
        let a = BigInt::from(-1);
        let b = BigInt::from(2);
        let e = BigInt::from(1);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_neg2_1() {
        let a = BigInt::from(-2);
        let b = BigInt::from(1);
        let e = BigInt::from(-1);
        assert_eq!(a + b, e);
    }

    #[test]
    fn add_neg1_neg1() {
        let a = BigInt::from(-1);
        let b = BigInt::from(-1);
        let e = BigInt::from(-2);
        assert_eq!(a + b, e);
    }
}
