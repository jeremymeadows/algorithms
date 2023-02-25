#![feature(slice_flatten)]

use std::iter;

mod constants;
use constants::{P_ARRAY, S_BOXES};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockCipherMode {
    Counter(u64),
    // OutputFeedback,
    // CipherFeedback,
    // CipherBlockChaining,
}

#[derive(Clone, Copy, Debug)]
pub struct BlowFish {
    p: [u32; 18],
    s: [[u32; 256]; 4],
    mode: BlockCipherMode,
}

trait Block {
    fn to_block(self) -> u64;
    fn to_halves(self) -> (u32, u32);
    fn to_bytes(self) -> [u8; 8];
}

impl BlowFish {
    pub fn new(key: &[u8]) -> BlowFish {
        let mut bf = BlowFish {
            p: P_ARRAY,
            s: S_BOXES,
            mode: BlockCipherMode::Counter(0),
        };

        let key: Vec<u32> = key
            .chunks_exact(4)
            .map(|e| u32::from_be_bytes(e.try_into().unwrap()))
            .collect();

        for i in 0..18 {
            bf.p[i] = bf.p[i] ^ key[i % key.len()];
        }

        let (mut l, mut r) = (0, 0);
        for i in (0..18).step_by(2) {
            (l, r) = bf.encrypt_block((l, r));
            bf.p[i] = l;
            bf.p[i + 1] = r
        }

        for i in 0..4 {
            for j in (0..256).step_by(2) {
                (l, r) = bf.encrypt_block((l, r));
                bf.s[i][j] = l;
                bf.s[i][j + 1] = r
            }
        }

        bf
    }

    pub fn with_mode(mut self, mode: BlockCipherMode) -> Self {
        self.mode = mode;
        self
    }

    fn f(&self, val: u32) -> u32 {
        let indexes = val.to_be_bytes();
        let mut x = [0u32; 4];

        for i in 0..4 {
            x[i] = self.s[i][indexes[i] as usize];
        }

        (x[0].wrapping_add(x[1]) ^ x[2]).wrapping_add(x[3])
    }

    fn encrypt_block<T: Block>(&self, block: T) -> (u32, u32) {
        let (mut l, mut r) = block.to_halves();

        for i in 0..16 {
            l ^= self.p[i];
            r ^= self.f(l);

            std::mem::swap(&mut l, &mut r);
        }

        l ^= self.p[16];
        r ^= self.p[17];
        (r, l)
    }

    #[allow(dead_code)]
    // counter mode doesn't need this
    fn decrypt_block<T: Block>(&self, block: T) -> (u32, u32) {
        let (mut l, mut r) = block.to_halves();

        for i in (2..18).rev() {
            l ^= self.p[i];
            r ^= self.f(l);

            std::mem::swap(&mut l, &mut r);
        }

        l ^= self.p[1];
        r ^= self.p[0];
        (r, l)
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let chunks = data.chunks_exact(8);

        let data = chunks
            .clone()
            .map(|e| u64::from_be_bytes(e.try_into().unwrap()).to_halves())
            .collect::<Vec<(u32, u32)>>();
        let remainder = chunks.remainder();

        let BlockCipherMode::Counter(mut counter) = self.mode;
        for (data_l, data_r) in data {
            let (counter_l, counter_r) = self.encrypt_block(counter.to_halves());
            counter += 1;

            for i in (data_l ^ counter_l, data_r ^ counter_r).to_bytes() {
                bytes.push(i);
            }
        }

        for (b, n) in iter::zip(remainder, self.encrypt_block(counter.to_halves()).to_bytes()) {
            bytes.push(b ^ n);
        }

        bytes
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        match self.mode {
            BlockCipherMode::Counter(_) => self.encrypt(data),
        }
    }
}

impl Block for u64 {
    fn to_block(self) -> u64 {
        self
    }

    fn to_halves(self) -> (u32, u32) {
        ((self >> 32) as u32, self as u32)
    }

    fn to_bytes(self) -> [u8; 8] {
        self.to_be_bytes()
    }
}

impl Block for (u32, u32) {
    fn to_block(self) -> u64 {
        (self.0 as u64) << 32 | self.1 as u64
    }

    fn to_halves(self) -> (u32, u32) {
        self
    }

    fn to_bytes(self) -> [u8; 8] {
        [self.0.to_be_bytes(), self.1.to_be_bytes()]
            .flatten()
            .try_into()
            .unwrap()
    }
}

impl Block for [u8; 8] {
    fn to_block(self) -> u64 {
        u64::from_be_bytes(self)
    }

    fn to_halves(self) -> (u32, u32) {
        (
            u32::from_be_bytes(self[0..4].try_into().unwrap()),
            u32::from_be_bytes(self[4..8].try_into().unwrap()),
        )
    }

    fn to_bytes(self) -> [u8; 8] {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_block() {
        let data = "abcdefgh".as_bytes();
        let bf = BlowFish::new(&[0, 0, 0, 0, 0, 0, 0, 0]);

        let data_block = (
            u32::from_be_bytes(data[0..4].try_into().unwrap()),
            u32::from_be_bytes(data[4..8].try_into().unwrap()),
        );

        let enc = bf.encrypt_block(data_block);
        let dec = bf.decrypt_block(enc);

        let enc = [enc.0.to_be_bytes(), enc.1.to_be_bytes()].concat();
        let dec = [dec.0.to_be_bytes(), dec.1.to_be_bytes()].concat();

        assert_eq!(enc, [26, 10, 94, 236, 107, 46, 20, 81]);
        assert_eq!(dec, data);
    }

    #[test]
    fn single_block_counter() {
        let data = "abcdefgh".as_bytes();
        let bf = BlowFish::new(&[0, 0, 0, 0, 0, 0, 0, 0]).with_mode(BlockCipherMode::Counter(0));

        let enc = bf.encrypt(data);
        let dec = bf.decrypt(&enc);

        assert_eq!(enc, [47, 155, 244, 33, 4, 254, 186, 16]);
        assert_eq!(dec, data);
    }

    #[test]
    fn smaller_block_counter() {
        let data = "abcdef".as_bytes();
        let bf = BlowFish::new(&[0, 0, 0, 0, 0, 0, 0, 0]).with_mode(BlockCipherMode::Counter(0));

        let enc = bf.encrypt(data);
        let dec = bf.decrypt(&enc);

        assert_eq!(enc, [47, 155, 244, 33, 4, 254]);
        assert_eq!(dec, data);
    }

    #[test]
    fn larger_block_counter() {
        let data = "the quick brown fox jumped over the lazy dog".as_bytes();
        let bf = BlowFish::new(&[0, 0, 0, 0, 0, 0, 0, 0]).with_mode(BlockCipherMode::Counter(0));

        let enc = bf.encrypt(data);
        let dec = bf.decrypt(&enc);

        assert_eq!(
            enc,
            [
                58, 145, 242, 101, 16, 237, 180, 27, 15, 205, 100, 37, 56, 38, 113, 135, 203, 12,
                110, 76, 199, 140, 236, 110, 22, 120, 134, 219, 101, 126, 12, 209, 216, 77, 150, 8,
                67, 118, 137, 203, 79, 10, 114, 155
            ]
        );
        assert_eq!(dec, data);
    }
}

