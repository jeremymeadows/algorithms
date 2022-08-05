use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::{Base, BigInt};

#[derive(Debug)]
/// An error generated when trying to parse a string into a `BigInt`.
pub struct ParseBigIntError(());

impl Error for ParseBigIntError {}

impl Display for ParseBigIntError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "invalid integer value")
    }
}

#[derive(Debug)]
/// An error generated when trying to convert a `BigInt` into a primitive integer.
pub struct TryFromBigIntError(());

impl Error for TryFromBigIntError {}

impl Display for TryFromBigIntError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "integer value out of bounds for destination type")
    }
}

impl BigInt {
    /// Converts `self` into bytes in a big-endian order.
    pub fn to_be_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().into_iter().rev().collect()
    }

    /// Converts `self` into bytes in a little-endian order.
    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.data
            .clone()
            .into_iter()
            .flat_map(|e| e.to_le_bytes())
            .collect()
    }

    /// Converts big-endian bytes into a `BigInt`.
    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        let mut bytes = bytes.to_vec();
        bytes.reverse();
        Self::from_le_bytes(&bytes)
    }

    /// Converts little-endian bytes into a `BigInt`.
    pub fn from_le_bytes(bytes: &[u8]) -> Self {
        const BYTES: usize = Base::BITS as usize / 8;

        let mut bytes = bytes.to_vec();
        bytes.append(&mut vec![0; BYTES.saturating_sub(bytes.len()) % BYTES]);

        let mut digits = bytes
            .chunks(BYTES)
            .map(|e| Base::from_le_bytes(e.try_into().unwrap()))
            .collect::<Vec<_>>();

        while digits.len() > 1 && digits[digits.len() - 1] == 0 {
            digits.pop();
        }

        Self {
            signed: false,
            data: digits,
        }
    }

    /// Converts a string in a given base to a `BigInt`. Panics if `radix` is not in `2..=36`.
    pub fn from_str_radix(mut s: &str, radix: u8) -> Result<Self, ParseBigIntError> {
        assert!((2..=36).contains(&radix), "radix must be within 2..=36");

        let mut sign = false;
        if s.starts_with('-') {
            sign = true;
            s = &s[1..];
        } else if s.starts_with('+') {
            s = &s[1..];
        }

        if s.len() == 0 {
            return Err(ParseBigIntError(()));
        }

        let mut pow = BigInt::one();
        let mut val = BigInt::zero();

        for c in s.to_ascii_lowercase().chars().rev() {
            let digit = match c {
                '0'..='9' => c as u8 - b'0',
                'a'..='z' => c as u8 - b'a' + 10,
                '_' => continue,
                _ => u8::MAX,
            };

            if digit < radix {
                val += pow.clone() * digit;
                pow *= radix;
            } else {
                return Err(ParseBigIntError(()));
            }
        }

        val.signed = sign;
        Ok(val)
    }
}

impl FromStr for BigInt {
    type Err = ParseBigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            Self::from_str_radix(&s[2..], 16)
        } else if s.starts_with("0o") {
            Self::from_str_radix(&s[2..], 8)
        } else if s.starts_with("0b") {
            Self::from_str_radix(&s[2..], 2)
        } else {
            Self::from_str_radix(&s, 10)
        }
    }
}

macro_rules! impl_from_uint {
    ($($t:ty),*) => {
        $(
            impl From<$t> for BigInt {
                fn from(num: $t) -> Self {
                    BigInt::from_le_bytes(&num.to_le_bytes())
                }
            }

            impl TryFrom<BigInt> for $t {
                type Error = TryFromBigIntError;

                fn try_from(i: BigInt) -> Result<Self, Self::Error> {
                    if i.bits() <= <$t>::BITS as usize && let Ok(val) = <$t>::try_from(i.data[0]) {
                        Ok(val)
                    } else {
                        Err(TryFromBigIntError(()))
                    }
                }
            }
        )*
    }
}

macro_rules! impl_from_int {
    ($($t:ty),*) => {
        $(
            impl From<$t> for BigInt {
                fn from(num: $t) -> Self {
                    BigInt {
                        data: BigInt::from_le_bytes(&num.abs().to_le_bytes()).data,
                        signed: num.is_negative(),
                    }
                }
            }

            impl TryFrom<BigInt> for $t {
                type Error = TryFromBigIntError;

                fn try_from(i: BigInt) -> Result<Self, Self::Error> {
                    if i.bits() < <$t>::BITS as usize && let Ok(val) = <$t>::try_from(i.data[0] * i.signum() as u64) {
                        Ok(val)
                    } else {
                        Err(TryFromBigIntError(()))
                    }
                }
            }
        )*
    }
}

impl_from_uint!(u8, u16, u32, u64, u128, usize);
impl_from_int!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BaseExt;

    #[test]
    fn it_works() {
        assert_eq!(BigInt::from(0u8).data, vec![0]);
        assert_eq!(BigInt::from(1).data, vec![1]);
        assert_eq!(BigInt::from(Base::MAX).data, vec![Base::MAX]);
        assert_eq!(BigInt::from(Base::MAX as BaseExt + 1).data, vec![0, 1]);
        assert_eq!(
            BigInt::from(Base::MAX as BaseExt * Base::MAX as BaseExt).data,
            vec![1, Base::MAX - 1]
        );
        assert_eq!(
            BigInt::from((Base::MAX as BaseExt + 1) * Base::MAX as BaseExt).data,
            vec![0, Base::MAX]
        );
    }

    #[test]
    fn to_be_bytes() {
        assert_eq!(
            BigInt::from(Base::MAX / 2).to_be_bytes(),
            (Base::MAX / 2).to_be_bytes()
        );
    }

    #[test]
    fn to_le_bytes() {
        assert_eq!(
            BigInt::from(Base::MAX / 2).to_le_bytes(),
            (Base::MAX / 2).to_le_bytes()
        );
    }

    #[test]
    fn from_str_small() {
        assert_eq!(BigInt::from_str("42").unwrap(), 42,);
    }

    #[test]
    fn from_str_big() {
        assert_eq!(BigInt::from_str(&Base::MAX.to_string()).unwrap(), Base::MAX);
    }

    #[test]
    fn from_str_bigger() {
        println!("{}", BaseExt::MAX);
        println!("{:?}", BigInt::from(BaseExt::MAX));
        assert_eq!(
            BigInt::from_str(&BaseExt::MAX.to_string()).unwrap(),
            BaseExt::MAX,
        );
    }

    #[test]
    fn from_str_hex() {
        assert_eq!(
            BigInt::from_str("0x77076D0A7318A57D3C16C17251B26645DF4C2F87EBC0992AB177FBA51DB92C2A")
                .unwrap(),
            BigInt {
                signed: false,
                data: vec![
                    0xB177FBA51DB92C2A,
                    0xDF4C2F87EBC0992A,
                    0x3C16C17251B26645,
                    0x77076D0A7318A57D
                ],
            }
        );
    }
}
