//!

use crate::{Rng, RngOutput, u32_to_f32, u64_to_f64};

use std::arch::asm;
use std::convert::Infallible;

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
    ($($t:ty),*) => {
        $(
            impl RngOutput<CpuRng> for $t {
                fn gen(_: &mut CpuRng) -> Self {
                    let x: usize;

                    unsafe { asm! {
                        "RDRAND {}",
                        out(reg) x,
                    }}

                    if Self::BITS > 64 {
                        let y: u64;

                        unsafe { asm! {
                            "RDRAND {}",
                            out(reg) y,
                        }}

                        ((x as u128) << 64 | (y as u128)) as Self
                    } else {
                        x as Self
                    }
                }
            }
        )*
    };
}

impl_int_cpu_rng_output!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl RngOutput<CpuRng> for f32 {
    fn gen(g: &mut CpuRng) -> Self {
        u32_to_f32(g.get::<u32>())
    }
}

impl RngOutput<CpuRng> for f64 {
    fn gen(g: &mut CpuRng) -> Self {
        u64_to_f64(g.get::<u64>())
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
