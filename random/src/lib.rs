//! Provides an interface for random number generators, along with some hardware
//! based implementations for supported platforms.

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_feature = "rdrand"))]
mod rdrand;
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_feature = "rdrand"))]
pub use rdrand::CpuRng;

#[cfg(target_family = "unix")]
mod urandom;
#[cfg(target_family = "unix")]
pub use urandom::OsRng;

/// A type that can produce random numbers.
pub trait Rng {
    /// The type which the generator is able to use to re-seed itself.
    type Seed;

    /// Seeds the generator with a value.
    fn seed(&mut self, s: Self::Seed);

    /// Get the next value from the generator.
    fn get<T: RngOutput<Self>>(&mut self) -> T
    where
        Self: Sized,
    {
        T::gen(self)
    }

    /// Get a vector containing the next `n` values from the generator.
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

    /// Fill a slice with random values.
    fn fill<T: RngOutput<Self>>(&mut self, buf: &mut [T])
    where
        Self: Sized,
    {
        for i in 0..(buf.len()) {
            buf[i] = T::gen(self)
        }
    }
}

/// A value which can be output from the generator.
pub trait RngOutput<G: Rng> {
    fn gen(generator: &mut G) -> Self;
}

#[doc(hidden)]
/// Convert a `u32` to an `f32` with an approximitely even distribution.
pub fn u32_to_f32(i: u32) -> f32 {
    (i >> 7) as f32 * (1.0 / 0x1ff_ffffu32 as f32)
}

#[doc(hidden)]
/// Convert a `u64` to an `f64` with an approximitely even distribution.
pub fn u64_to_f64(i: u64) -> f64 {
    (i >> 11) as f64 * (1.0 / 0x1f_ffff_ffff_ffffu64 as f64)
}
