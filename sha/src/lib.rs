#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(stmt_expr_attributes)]

pub mod sha1;
pub mod sha256;
pub mod sha512;

use std::fmt::{Debug, Display};
use std::ops::Deref;

/// A type that can be used to hash data.
pub trait Sha<T: Digest>: private::Sealed {
    const OUTPUT_SIZE: usize;

    fn new() -> Self;
    fn hash(data: &[u8]) -> T;
    fn hmac(key: &[u8], data: &[u8]) -> T;
    fn add(&mut self, data: &[u8]);
    fn digest(&mut self) -> T;
    fn digest_hmac(&mut self, key: &[u8]) -> T;
}

/// A type which represents hashed data.
pub trait Digest:
    Clone + Copy + Debug + Deref + Display + Eq + PartialEq + private::Sealed
{
    const OUTPUT_SIZE: usize;

    fn as_bytes(&self) -> [u8; Self::OUTPUT_SIZE];
}

mod private {
    pub trait Sealed {}
}

// Creates a `digest` struct to complement the `Sha` trait, and implements all
// required traits on it.
macro_rules! impl_digest {
    ($digest:ident for $hasher:ty) => {
        /// The completed digest for a given hash.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $digest([u8; <$hasher>::OUTPUT_SIZE]);

        impl Digest for $digest {
            const OUTPUT_SIZE: usize = <$hasher>::OUTPUT_SIZE;

            fn as_bytes(&self) -> [u8; Self::OUTPUT_SIZE] {
                self.0
            }
        }

        impl std::ops::Deref for $digest {
            type Target = [u8; Self::OUTPUT_SIZE];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $digest {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}", self.map(|e| format!("{:02x}", e)).join(""))
            }
        }

        impl crate::private::Sealed for $hasher {}
        impl crate::private::Sealed for $digest {}

        crate::to_digest!([u8], $digest);
        crate::to_digest!([u32], $digest);
        crate::to_digest!([u64], $digest);
    };
}

use impl_digest;

// Implements `From<[uX; N]>` for a `digest` struct.
macro_rules! to_digest {
    ([$arr:ty], $digest:ty) => {
        impl From<[$arr; <$digest>::OUTPUT_SIZE / (<$arr>::BITS as usize / 8)]> for $digest {
            fn from(digest: [$arr; <$digest>::OUTPUT_SIZE / (<$arr>::BITS as usize / 8)]) -> Self {
                let mut bytes = [0u8; Self::OUTPUT_SIZE];
                let size = <$arr>::BITS as usize / 8;

                for i in 0..(digest.len()) {
                    bytes[size * i..][..size].copy_from_slice(&digest[i].to_be_bytes());
                }
                Self(bytes)
            }
        }
    };
}

use to_digest;
