use std::fmt::{self, Binary, Display, Formatter, LowerHex, Octal, UpperHex};

use crate::BigInt;

impl BigInt {
    /// Converts a `BigInt` into a string with the given base. Panics if `radix` is not in `2..=36`.
    pub fn to_string_radix(&self, radix: u8) -> String {
        assert!((2..=36).contains(&radix), "radix must be within 2..=36");

        if self == 0 {
            return "0".to_string();
        }

        let radix = BigInt::from(radix);
        let mut val = self.clone();
        let mut s = String::new();
        let mut rem;

        while val.abs() > 0 {
            (val, rem) = val.div_rem(radix.clone());
            let r = u8::try_from(rem).unwrap();

            s.push(match r {
                0..=9 => r + b'0',
                10..=26 => r + b'a' - 10,
                _ => unreachable!(),
            } as char);
        }

        if self.is_negative() {
            s.push('-');
        }
        s.chars().rev().collect()
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(10))
    }
}

impl Binary for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(2))
    }
}

impl Octal for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(8))
    }
}

impl LowerHex for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(16))
    }
}

impl UpperHex for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(16).to_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_fmts {
        ($name:ident: $a:expr, $dec:literal, $bin:expr, $oct:expr, $hex:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a.to_string(), $dec, "failed decimal output");
                assert_eq!(format!("{:b}", $a), $bin, "failed binary output");
                assert_eq!(format!("{:o}", $a), $oct, "failed octal output");
                assert_eq!(
                    format!("{:x}", $a),
                    $hex.to_lowercase(),
                    "failed lower hex output"
                );
                assert_eq!(
                    format!("{:X}", $a),
                    $hex.to_uppercase(),
                    "failed upper hex output"
                );
            }
        };
    }

    test_fmts!(zero: BigInt::zero(), "0", "0", "0", "0");

    test_fmts!(one: BigInt::one(), "1", "1", "1", "1");

    test_fmts!(two: BigInt::from(2), "2", "10", "2", "2");

    test_fmts!(ten: BigInt::from(10), "10", "1010", "12", "a");

    test_fmts!(sixteen: BigInt::from(16), "16", "10000", "20", "10");

    test_fmts!(big:
        BigInt::from(0xffffffff_ffffffff_ffffffff_ffffffffu128),
        "340282366920938463463374607431768211455", "1".repeat(128), format!("3{}", "7".repeat(42)), "f".repeat(32)
    );
}
