//! Implementation for the Mersenne-Twister pseudo-random number generator.

use random;
pub use random::{Rng, RngOutput};

const STATE_SIZE: usize = 312;
const HALF_SIZE: usize = STATE_SIZE / 2;

const MAGIC: u64 = 0xB5026F5AA96619E9;

const UPPER: u64 = 0xFFFFFFFF80000000;
const LOWER: u64 = 0x000000007FFFFFFF;

/// A Mersenne Twister pseudo-random number generator
pub struct MersenneTwister {
    state: [u64; STATE_SIZE],
    next: usize,
}

impl Default for MersenneTwister {
    /// Creates a new generator seeded with its memory address. Useful to get a
    /// non-deterministic seed value.
    fn default() -> Self {
        let mut gen = Self::new();
        gen.seed(&gen as *const Self as u64);
        gen
    }
}

impl From<u64> for MersenneTwister {
    /// Creates a new generator with a seed value.
    fn from(seed: u64) -> Self {
        let mut gen = Self::new();
        gen.seed(seed);
        gen
    }
}

impl MersenneTwister {
    /// Creates a new generator.
    pub fn new() -> Self {
        let mut gen = MersenneTwister {
            state: [0; STATE_SIZE],
            next: STATE_SIZE + 1,
        };

        gen.seed(5489);
        gen
    }

    // Seeds the generator with a new value and regenerates the internal state.
    fn seed(&mut self, seed: u64) {
        self.state[0] = seed;
        for i in 1..STATE_SIZE {
            self.state[i] = 6364136223846793005u64
                .wrapping_mul(self.state[i - 1] ^ (self.state[i - 1] >> 62))
                .wrapping_add(i as u64);
        }
        self.next = STATE_SIZE;
    }

    fn twist(&mut self) {
        let mut x;
        let magic = [0, MAGIC];

        for i in 0..(HALF_SIZE) {
            x = (self.state[i] & UPPER) | (self.state[i + 1] & LOWER);
            self.state[i] = self.state[i + HALF_SIZE] ^ (x >> 1) ^ magic[(x & 1) as usize];
        }

        for i in (HALF_SIZE)..(STATE_SIZE - 1) {
            x = (self.state[i] & UPPER) | (self.state[i + 1] & LOWER);
            self.state[i] = self.state[(i.wrapping_sub(HALF_SIZE) as isize) as usize]
                ^ (x >> 1)
                ^ magic[(x & 1) as usize];
        }

        x = (self.state[STATE_SIZE - 1] & UPPER) | (self.state[0] & LOWER);
        self.state[STATE_SIZE - 1] = self.state[HALF_SIZE - 1] ^ (x >> 1) ^ magic[(x & 1) as usize];
    }

    // Returns the next value and regenerates the state if needed.
    fn next(&mut self) -> u64 {
        if self.next >= STATE_SIZE {
            self.twist();
            self.next = 0;
        }

        let mut x = self.state[self.next];
        self.next += 1;

        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71D67FFFEDA60000;
        x ^= (x << 37) & 0xFFF7EEE000000000;
        x ^ x >> 43
    }
}

impl Rng for MersenneTwister {
    type Seed = u64;

    fn seed(&mut self, seed: u64) {
        self.seed(seed);
    }
}

macro_rules! impl_int_mt_output {
    ($($t:ty),*) => {
        $(
            impl RngOutput<MersenneTwister> for $t {
                fn gen(g: &mut MersenneTwister) -> Self {
                    let mut x = g.next() as Self;
                    if Self::BITS > u64::BITS {
                        x = (((x as u128) << 64) | g.next() as u128) as Self;
                    }
                    x
                }
            }
        )*
    };
}

impl_int_mt_output!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl RngOutput<MersenneTwister> for f32 {
    fn gen(g: &mut MersenneTwister) -> Self {
        random::u32_to_f32(g.get::<u32>())
    }
}

impl RngOutput<MersenneTwister> for f64 {
    fn gen(g: &mut MersenneTwister) -> Self {
        random::u64_to_f64(g.get::<u64>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut gen = MersenneTwister::new();
        let exp = [
            14514284786278117030,
            04620546740167642908,
            13109570281517897720,
            17462938647148434322,
            00355488278567739596,
            07469126240319926998,
            04635995468481642529,
            00418970542659199878,
            09604170989252516556,
            06358044926049913402,
        ];

        for i in 0..10 {
            assert_eq!(gen.next(), exp[i]);
        }
    }

    #[test]
    fn test_default() {
        let mut gen = MersenneTwister::default();
        for _ in 0..10 {
            gen.next();
        }
    }

    #[test]
    fn test_seed() {
        let mut gen = MersenneTwister::from(0xff);
        let exp = [
            03220586997909315655,
            03303451203970382242,
            11896436706893466529,
            08960318650144385956,
            04679212705770455613,
            15567843309247195414,
            06961994097256010468,
            10807484256991480663,
            11890420171946432686,
            15474158341220671739,
        ];

        for i in 0..10 {
            assert_eq!(gen.next(), exp[i]);
        }

        gen.seed(0xff);

        for i in 0..10 {
            assert_eq!(gen.next(), exp[i]);
        }
    }

    #[test]
    fn test_real() {
        let mut gen = MersenneTwister::new();
        let exp = [
            0.7868209548678021,
            0.2504803406880287,
            0.7106712289786555,
            0.9466678009609706,
            0.0192710581958138,
            0.4049021448161677,
            0.2513178179280376,
            0.0227124386279267,
            0.5206431525734918,
            0.3446703060791877,
        ];

        let err = 0.0000000000000001;

        for i in 0..10 {
            assert!(gen.get::<f64>() - exp[i] < err);
        }
    }
}