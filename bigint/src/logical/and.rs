use std::ops::BitAnd;

use crate::BigInt;

impl BitAnd for BigInt {
    type Output = Self;

    fn bitand(mut self, mut other: Self) -> Self::Output {
        if self.data.len() > other.data.len() {
            std::mem::swap(&mut self, &mut other);
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

    #[test]
    fn and_1_0() {
        let a = BigInt::from(1);
        let b = BigInt::from(0);
        let c = BigInt::from(0);

        assert_eq!(a & b, c);
    }
    
    #[test]
    fn and_257_1() {
        let a = BigInt::from(257);
        let b = BigInt::from(1);
        let c = BigInt::from(1);

        assert_eq!(a & b, c);
    }
}
