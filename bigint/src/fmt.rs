use std::fmt::{self, Binary, Display, Formatter, LowerHex, Octal, UpperHex};

use crate::{Base, BigInt};

impl BigInt {
    fn to_string_radix(&self, radix: u8) -> String {
        assert!(2 <= radix && radix <= 36, "radix must be within 2..=36");

        let mut s = String::new();
        if *self == 0 {
            s.push('0');
        }
        _ = s;
        todo!()
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if *self == 0 {
            return write!(f, "0");
        }

        write!(f, "{}", self.to_string_radix(10))
    }
}

impl Binary for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if *self == 0 {
            return write!(f, "0");
        }

        let digits = Base::BITS as usize;
        let s = self
            .data
            .iter()
            .rev()
            .map(|e| format!("{:0digits$b}", e))
            .collect::<String>();

        write!(
            f,
            "{}{}",
            if self.signed { "-" } else { "" },
            s.trim_start_matches("0")
        )
    }
}

impl Octal for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if *self == 0 {
            return write!(f, "0");
        }

        let mut bin = format!("{:b}", self);
        while bin.len() % 3 != 0 {
            bin = format!("0{bin}");
        }

        let s = bin
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|e| e.iter().collect::<String>())
            .map(|e| Base::from_str_radix(&e, 2).unwrap().to_string())
            .collect::<String>();

        write!(
            f,
            "{}{}",
            if self.signed { "-" } else { "" },
            s.trim_start_matches("0")
        )
    }
}

impl LowerHex for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if *self == 0 {
            return write!(f, "0");
        }

        let digits = Base::BITS as usize / 4;
        let s = self
            .data
            .iter()
            .rev()
            .map(|e| format!("{:0digits$x}", e))
            .collect::<String>();

        write!(
            f,
            "{}{}",
            if self.signed { "-" } else { "" },
            s.trim_start_matches("0")
        )
    }
}

impl UpperHex for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if *self == 0 {
            println!("here");
            return write!(f, "0");
        }

        let digits = Base::BITS as usize / 4;
        let s = self
            .data
            .iter()
            .rev()
            .map(|e| format!("{:0digits$X}", e))
            .collect::<String>();

        write!(
            f,
            "{}{}",
            if self.signed { "-" } else { "" },
            s.trim_start_matches("0")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_fmts {
        ($name:ident: $a:expr, $dec:literal, $bin:expr, $oct:expr, $hex:expr) => {
            #[test]
            fn $name() {
                // assert_eq!($a.to_string(), $dec, "failed decimal output");
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

    test_fmts!(zero: BigInt::from(0), "0", "0", "0", "0");

    test_fmts!(one: BigInt::from(1), "1", "1", "1", "1");

    test_fmts!(two: BigInt::from(2), "2", "10", "2", "2");

    test_fmts!(ten: BigInt::from(10), "10", "1010", "12", "a");

    test_fmts!(sixteen: BigInt::from(16), "16", "10000", "20", "10");

    test_fmts!(big:
        BigInt::from(0xffffffff_ffffffff_ffffffff_ffffffffu128),
        "340282366920938463463374607431768211455", "1".repeat(128), format!("3{}", "7".repeat(42)), "f".repeat(32)
    );
}
