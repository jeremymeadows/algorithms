//! Uses the `RDRAND` cpu instruction to get random numbers on supported
//! platforms.

use crate::{Rng, RngOutput};

use std::arch::asm;
use std::cmp;
use std::convert::Infallible;

/// A random number generator using a supported CPU's random instructions.
/// 
/// Provides outputs for all primitive numeric types.
pub struct CpuRng {}

impl CpuRng {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rng for CpuRng {
    type Seed = Infallible;

    fn seed(&mut self, _: Self::Seed) {
        panic!("cannot seed the CPU RNG")
    }
}

macro_rules! impl_int_cpu_rng_output {
    ($($t:ty),+) => {
        $(
            impl RngOutput<CpuRng> for $t {
                fn gen(_: &mut CpuRng) -> Self {
                    const LEN: usize = <$t>::BITS as usize / 8;
                    const PTR_LEN: usize = usize::BITS as usize / 8;

                    let len = cmp::min(LEN, PTR_LEN);
                    let mut bytes = [0; LEN];

                    // grab pointer-sized chunks of random data until enough has been gathered to
                    // create the output type. this has the flexibility to work on processors with
                    // different sized registers
                    for i in 0..cmp::max(1, LEN / PTR_LEN) {
                        let x: usize;

                        unsafe { asm! {
                            "RDRAND {}",
                            out(reg) x,
                        }}

                        bytes[(i * len)..][..len].copy_from_slice(&x.to_ne_bytes()[..len]);
                    }
                    Self::from_ne_bytes(bytes)
                }
            }
        )+
    };
}

impl_int_cpu_rng_output!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl RngOutput<CpuRng> for f32 {
    fn gen(g: &mut CpuRng) -> Self {
        crate::u32_to_f32(g.get::<u32>())
    }
}

impl RngOutput<CpuRng> for f64 {
    fn gen(g: &mut CpuRng) -> Self {
        crate::u64_to_f64(g.get::<u64>())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::{CpuRng, Rng};

        let mut rng = CpuRng::new();

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
