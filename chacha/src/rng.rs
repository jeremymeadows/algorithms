//!

use crate::ChaCha;

use random;
pub use random::{Rng, RngOutput};

pub struct ChaChaRng(ChaCha);

impl Default for ChaChaRng {
    /// Creates a new generator seeded with a memory address. Useful to get a
    /// semi-non-deterministic seed value.
    fn default() -> Self {
        ChaChaRng(ChaCha::new([(&() as *const () as u32); 8]))
    }
}

impl ChaChaRng {
    pub fn new() -> ChaChaRng {
        ChaChaRng(ChaCha::new([0; 8]))
    }

    pub fn from(seed: [u32; 8]) -> ChaChaRng {
        ChaChaRng(ChaCha::new(seed).with_counter(0))
    }
}

macro_rules! impl_rng_output {
    ($($t:ty),+) => {
        $(
            impl RngOutput<ChaChaRng> for $t {
                fn gen(rng: &mut ChaChaRng) -> Self {
                    Self::from_ne_bytes(rng.0.encrypt(&[0; Self::BITS as usize / 8]).try_into().unwrap())
                }
            }
        )+
    };
}

impl_rng_output!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl RngOutput<ChaChaRng> for f32 {
    fn gen(g: &mut ChaChaRng) -> Self {
        random::u32_to_f32(g.get::<u32>())
    }
}

impl RngOutput<ChaChaRng> for f64 {
    fn gen(g: &mut ChaChaRng) -> Self {
        random::u64_to_f64(g.get::<u64>())
    }
}

impl Rng for ChaChaRng {
    type Seed = [u32; 8];

    fn seed(&mut self, seed: Self::Seed) {
        *self = Self::from(seed);
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn chi_sqrd() {
        let mut rng = ChaChaRng::default();
        let n = 100000;
        let mut buckets = [vec![0; 10], vec![0; 100]];

        for _ in 0..n {
            let r = rng.get::<u32>();

            for b in buckets.iter_mut() {
                let l = b.len() as u32;
                b[(r % l) as usize] += 1;
            }
        }

        for b in buckets.iter() {
            let d = (n / b.len()) as f32;

            for i in b.iter() {
                assert!(*i as f32 > d * 0.9 && (*i as f32) < d * 1.1);
            }
        }
    }
}
