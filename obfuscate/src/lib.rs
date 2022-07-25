#![feature(once_cell)]

pub use proc_macros::obfuscate;

use std::fmt::{Debug, Display, Formatter};
use std::hint;
use std::lazy::SyncLazy;
use std::ops::Deref;

pub struct Obfuscated<T>(SyncLazy<T>);

pub trait Obfuscate {
    fn to_obfu_bytes(&self) -> Vec<u8>;
    fn from_obfu_bytes(bytes: &[u8]) -> Self;
}

impl<T: Obfuscate> Obfuscated<T> {
    pub const fn new(f: fn() -> T) -> Obfuscated<T> {
        Obfuscated(SyncLazy::new(f))
    }
}

impl<T> Deref for Obfuscated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.deref()
    }
}

impl<T: Display> Display for Obfuscated<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl<T: Debug> Debug for Obfuscated<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Obfuscated({:?})", self.deref())
    }
}

impl Obfuscate for String {
    fn to_obfu_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn from_obfu_bytes(bytes: &[u8]) -> Self {
        todo!()
    }
}
