pub mod add;
pub mod div;
pub mod mul;
pub mod rem;
pub mod sub;

use crate::BigInt;

impl BigInt {
    pub fn abs(&self) -> Self {
        BigInt {
            signed: false,
            data: self.data.clone(),
        }
    }

    pub fn signum(&self) -> i8 {
        if self.data == &[0] {
            0
        } else {
            if self.signed {
                -1
            } else {
                1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs() {
        assert_eq!(BigInt::from(-123_i8).abs(), BigInt::from(123_u8));
        assert_eq!(BigInt::from(-256_i16).abs(), BigInt::from(256_u16));
        assert_eq!(BigInt::from(-65536_i32).abs(), BigInt::from(65536_u32));
    }
}
