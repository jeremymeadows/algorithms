//! Uses the `getentropy` libc method to get random numbers on supported
//! platforms.

use crate::{Rng, RngOutput};

use libc::{self, c_void};
use std::convert::Infallible;

/// A random number generator using a operating system's random number generator.
/// 
/// Provides outputs for all primitive numeric types.
pub struct OsRng {}

impl OsRng {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rng for OsRng {
    type Seed = Infallible;

    fn seed(&mut self, _: Self::Seed) {
        panic!("cannot seed the OS RNG")
    }
}

macro_rules! impl_int_os_rng_output {
    ($($t:ty),+) => {
        $(
            impl RngOutput<OsRng> for $t {
                fn gen(_: &mut OsRng) -> Self {
                    const LEN: usize = <$t>::BITS as usize / 8;
                    let mut buf = [0u8; LEN];

                    unsafe {
                        libc::getentropy(&mut buf as *mut [u8; LEN] as *mut c_void, LEN);
                    }
                    Self::from_ne_bytes(buf)
                }
            }
        )+
    };
}

impl_int_os_rng_output!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl RngOutput<OsRng> for f32 {
    fn gen(g: &mut OsRng) -> Self {
        crate::u32_to_f32(g.get::<u32>())
    }
}

impl RngOutput<OsRng> for f64 {
    fn gen(g: &mut OsRng) -> Self {
        crate::u64_to_f64(g.get::<u64>())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::{OsRng, Rng};

        let mut rng = OsRng::new();

        println!("{}", rng.get::<u8>());
        println!("{}", rng.get::<u16>());
        println!("{}", rng.get::<u32>());
        println!("{}", rng.get::<u64>());
        println!("{}", rng.get::<u128>());

        println!("{}", rng.get::<i8>());
        println!("{}", rng.get::<i16>());
        println!("{}", rng.get::<i32>());
        println!("{}", rng.get::<i64>());
        println!("{}", rng.get::<i128>());

        println!("{}", rng.get::<f32>());
        println!("{}", rng.get::<f64>());
    }
}
