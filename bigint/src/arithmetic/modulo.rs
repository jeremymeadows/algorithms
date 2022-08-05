use crate::BigInt;

impl BigInt {
    /// Calculates the modulus of a `BigInt`.
    pub fn modulo(&self, m: &BigInt) -> BigInt {
        let mut rem = self % m;
        if rem < 0 {
            rem += m.abs();
        }
        rem
    }

    // Calculates the power of a number within a modulus.
    pub fn mod_pow(&self, exp: &BigInt, m: &BigInt) -> BigInt {
        if exp == 0 {
            return BigInt::one().modulo(m);
        }

        let base = self.clone();
        let mut val = self.clone();
        let mut exp = exp.clone();

        while exp > 0 {
            val = val.modulo(m) * base.modulo(m);
            exp -= 1;
        }
        val.modulo(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base, BaseExt};

    macro_rules! test_mod {
        ($name:ident: $a:expr, $b:expr, $e:expr) => {
            #[test]
            fn $name() {
                assert_eq!($a.modulo(&$b), $e);
            }
        };
    }

    test_mod!(positive: BigInt::from(7), BigInt::from(3), 1);

    test_mod!(neg_numerator: BigInt::from(-7), BigInt::from(3), 2);

    test_mod!(neg_denominator: BigInt::from(7), BigInt::from(-3), 1);

    test_mod!(negative: BigInt::from(-7), BigInt::from(-3), 2);
}
