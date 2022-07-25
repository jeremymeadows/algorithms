use std::fmt::{self, Display, Formatter};

pub mod arithmetic;
pub mod cmp;
pub mod logical;

type Base = u8;
type BaseExt = u16;

#[derive(Clone, Debug)]
pub struct BigInt {
    signed: bool,
    data: Vec<Base>,
}

impl BigInt {
    pub fn new() -> Self {
        Self {
            signed: false,
            data: vec![0],
        }
    }

    pub fn abs(mut self) -> Self {
        self.signed = false;
        self
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for i in self.data.iter().rev() {
            s += format!("{:02x}", i).as_str();
        }
        s = s.trim_start_matches("0").to_string();
        s = format!("0x{}", s);

        if self.signed {
            s = format!("-{}", s);
        }

        write!(f, "{}", s)
    }
}

macro_rules! impl_from_uint {
    ($($t:ty),*) => {
        $(
            impl From<$t> for BigInt {
                fn from(num: $t) -> Self {
                    let mut digits = num
                        .to_be_bytes()
                        .chunks((Base::BITS / 8).try_into().unwrap())
                        .map(|e| Base::from_be_bytes(e.try_into().unwrap()))
                        .rev()
                        .collect::<Vec<Base>>();

                    while digits.len() > 1 && digits[digits.len() - 1] == 0 {
                        digits.pop();
                    }

                    Self {
                        signed: false,
                        data: digits,
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
                    let signed = num < 0;
                    let mut digits = num
                        .abs()
                        .to_be_bytes()
                        .chunks((Base::BITS / 8).try_into().unwrap())
                        .map(|e| Base::from_be_bytes(e.try_into().unwrap()))
                        .rev()
                        .collect::<Vec<Base>>();

                    while digits.len() > 1 && digits[digits.len() - 1] == 0 {
                        digits.pop();
                    }

                    Self {
                        signed: signed,
                        data: digits,
                    }
                }
            }
        )*
    }
}

impl_from_uint!(u8, u16, u32, u64, u128);
impl_from_int!(i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(BigInt::from(0_u8).data, vec![0]);
        assert_eq!(BigInt::from(1_u8).data, vec![1]);
        assert_eq!(BigInt::from(255_u8).data, vec![255]);
        assert_eq!(BigInt::from(256_u16).data, vec![0, 1]);
        assert_eq!(BigInt::from(65535_u16).data, vec![255, 255]);
        assert_eq!(BigInt::from(65536_u32).data, vec![0, 0, 1]);

        assert_eq!(BigInt::from(-123_i8).abs(), BigInt::from(123_u8));
        assert_eq!(BigInt::from(-256_i16).abs(), BigInt::from(256_u16));
        assert_eq!(BigInt::from(-65536_i32).abs(), BigInt::from(65536_u32));
    }
}
