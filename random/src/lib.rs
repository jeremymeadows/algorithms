//!

mod rdrand;
mod urandom;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_feature = "rdrand"))]
pub use rdrand::CpuRng;

#[cfg(target_family = "unix")]
pub use urandom::OsRng;

pub trait Rng {
    type Seed;

    fn seed(&mut self, s: Self::Seed);

    fn get<T: RngOutput<Self>>(&mut self) -> T
    where
        Self: Sized,
    {
        T::gen(self)
    }

    fn get_n<T: RngOutput<Self>>(&mut self, n: usize) -> Vec<T>
    where
        Self: Sized,
    {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(T::gen(self));
        }
        v
    }

    fn fill<T: RngOutput<Self>>(&mut self, buf: &mut [T])
    where
        Self: Sized,
    {
        for i in 0..(buf.len()) {
            buf[i] = T::gen(self)
        }
    }
}

pub trait RngOutput<G: Rng> {
    fn gen(generator: &mut G) -> Self;
}

fn u32_to_f32(i: u32) -> f32 {
    (i >> 7) as f32 * (1.0 / 0x1ff_ffffu32 as f32)
}

fn u64_to_f64(i: u64) -> f64 {
    (i >> 11) as f64 * (1.0 / 0x1f_ffff_ffff_ffffu64 as f64)
}