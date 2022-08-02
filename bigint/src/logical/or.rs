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

    #[test]
    fn or_1_0() {
        let a = BigInt::from(1);
        let b = BigInt::from(0);
        let c = BigInt::from(1);

        assert_eq!(a | b, c);
    }

    #[test]
    fn or_256_1() {
        let a = BigInt::from(256);
        let b = BigInt::from(1);
        let c = BigInt::from(257);

        assert_eq!(a | b, c);
    }
}
