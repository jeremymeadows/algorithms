use std::mem;
use std::ops::BitXor;

use crate::BigInt;

impl BitXor for BigInt {
    type Output = Self;

    fn bitxor(mut self, mut other: Self) -> Self::Output {
        if self.data.len() > other.data.len() {
            mem::swap(&mut self, &mut other);
        }

        for i in 0..self.data.len() {
            if i < other.data.len() {
                self.data[i] ^= other.data[i];
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_0_1() {
        let a = BigInt::from(3);
        let b = BigInt::from(4);

        assert_eq!(a ^ b, BigInt::from(7));
    }
}
