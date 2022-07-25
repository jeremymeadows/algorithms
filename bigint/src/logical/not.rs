use std::ops::Not;

use crate::BigInt;

impl Not for BigInt {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        for i in 0..self.data.len() {
            self.data[i] = !self.data[i];
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Base;

    #[test]
    fn xor_0_1() {
        let a = BigInt::from(0u8);

        assert_eq!(!a, BigInt::from(!0u8));
    }
}
