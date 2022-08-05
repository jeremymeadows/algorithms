pub mod add;
pub mod div;
pub mod modulo;
pub mod mul;
pub mod rem;
pub mod sub;

use crate::BigInt;

impl BigInt {
    /// Returns the absolute value of a `BigInt`.
    pub fn abs(&self) -> Self {
        BigInt {
            signed: false,
            data: self.data.clone(),
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
