use random::{self, CpuRng, Rng};
use bigint::BigInt;

#[derive(Debug)]
pub struct Rsa {
    modulus: (u16, u16),
    private: BigInt,
    public: BigInt,
}

const PRIME_RANGE: usize = u16::MAX as usize;

fn primes() -> Vec<u16> {
    let mut v = vec![true; PRIME_RANGE - 1];
    let len = (PRIME_RANGE as f32).sqrt() as usize;

    for i in 2..len {
        if v[i] {
            let mut j = i.pow(2);
            while j < PRIME_RANGE - 1 {
                v[j] = false;
                j += i;
            }
        }
    }

    v.iter().enumerate().skip(2).filter(|(_, &e)| e).map(|(i, _)| i as u16).collect()
}

// fn is_prime(x: u128) -> bool {
//     println!("checking prime");
//     for i in (3..((x as f32).sqrt() as u128)).step_by(2) {
//         if x % i == 0 {
//             return false;
//         }
//     }
//     true
// }

// fn is_safe_prime(x: u128) -> bool {
//     is_prime((x - 1) / 2)
// }

impl Rsa {
    pub fn new() -> Self {
        let mut gen = CpuRng::new();
        let primes = primes().into_iter().filter(|e| (1023..).contains(e)).collect::<Vec<u16>>();
        let modulus =  (
            primes[gen.get::<usize>() % primes.len()],
            primes[gen.get::<usize>() % primes.len()],
        );
        let m = BigInt::from(modulus.0) * modulus.1;
        let other = (modulus.0 as u32 - 1) * (modulus.1 as u32 - 1);
        println!("{:?}", modulus);

        let public = BigInt::from(*primes.iter().filter(|&e| *e >= modulus.0 / 2).find(|&e| other % *e as u32 != 0).unwrap());
        println!("{public}");
        // let private = *primes.iter().filter(|&e| *e >= modulus.0 - 1).find(|&e| *e as u32 * public as u32 % other == 1).unwrap();
        // let private = (public..m).find(|&e| e as u32 * public % other == 1).unwrap();
        let mut private = BigInt::zero();
        for i in public.clone()..m {
            if &i * &public % other == BigInt::one() {
                private = i;
                break;
            }
        }

        Self { modulus, private, public }
    }

    pub fn encrypt(&self, bytes: &[u8]) -> Vec<u8> {
        let m = BigInt::from(self.modulus.0) * self.modulus.1;
        let d = BigInt::from_be_bytes(bytes);
        assert!(d < m, "too much data for the size of the modulus");

        d.modpow(&self.public, &m).to_be_bytes()
    }

    pub fn decrypt(&self, bytes: &[u8]) -> Vec<u8> {
        let m = BigInt::from(self.modulus.0) * self.modulus.1;
        let d = BigInt::from_be_bytes(bytes);
        assert!(d < m, "too much data for the size of the modulus");

        d.modpow(&self.private, &m).to_be_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let msg = &b"Hi";
        let keypair = Rsa::new();
        println!("{:?}", keypair);
        println!("{:?}", msg);
        println!("{:?}", keypair.encrypt(&msg[..]));
        // println!("{:?}", msg.as_bytes());
        // println!("{:?}", keypair.encrypt(msg.as_bytes()));
        // println!("{:?}", keypair.decrypt(&keypair.encrypt(msg.as_bytes())));
        // assert_eq!(String::from_utf8(keypair.decrypt(&keypair.encrypt(msg.as_bytes()))).unwrap(), msg);
        assert_eq!(keypair.decrypt(&keypair.encrypt(&msg[..])), msg.to_vec());
    }
}
