use std::str::FromStr;

use crate::{Base, BigInt};

impl BigInt {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        self.data
            .clone()
            .into_iter()
            .flat_map(|e| e.to_le_bytes())
            .rev()
            .collect()
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.data
            .clone()
            .into_iter()
            .flat_map(|e| e.to_le_bytes())
            .collect()
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        let mut bytes = bytes.to_vec();
        bytes.reverse();
        Self::from_le_bytes(&bytes)
    }

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

    pub fn from_str_radix(mut s: &str, radix: u8) -> Result<Self, &'static str> {
        assert!(2 <= radix && radix <= 36, "radix must be within 2..=36");
        let mut sign = false;

        if s.starts_with('-') {
            sign = true;
            s = &s[1..];
        } else if s.starts_with('+') {
            s = &s[1..];
        }

        let mut v = Vec::with_capacity(s.len());
        for b in s.bytes() {
            let d = match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'z' => b - b'a' + 10,
                b'A'..=b'Z' => b - b'A' + 10,
                b'_' => continue,
                _ => u8::MAX,
            };
            if d < radix {
                v.push(d as Base);
            } else {
                return Err("failed to parse int");
            }
        }

        _ = BigInt {
            signed: sign,
            data: v,
        };
        todo!()
    }
}

impl FromStr for BigInt {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            Self::from_str_radix(&s[2..].to_lowercase(), 16)
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
}
