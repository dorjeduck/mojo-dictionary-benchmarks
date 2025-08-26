use rustc_hash::FxHasher;
use std::hash::{ Hasher};
use ahash::AHasher;

const MULTIPLE: u64 = 6364136223846793005;
const ROT: u32 = 23;

#[inline]
fn folded_multiply(s: u64, by: u64) -> u64 {
    let b1 = s.wrapping_mul(by.swap_bytes());
    let b2 = s.swap_bytes().wrapping_mul(!by);
    b1 ^ b2.swap_bytes()
}

#[derive(Clone, Copy)]
struct U128(u64, u64);

#[inline]
fn read_small(data: &[u8]) -> U128 {
    let len = data.len();
    if len >= 2 {
        if len >= 4 {
            // len 4–8: (u32 at start, u32 at end)
            let a = u32::from_le_bytes(data[0..4].try_into().unwrap()) as u64;
            let b = u32::from_le_bytes(data[len - 4..len].try_into().unwrap()) as u64;
            U128(a, b)
        } else {
            // len 2–3: (u16 at start, last byte)
            let a = u16::from_le_bytes([data[0], data[1]]) as u64;
            let b = data[len - 1] as u64;
            U128(a, b)
        }
    } else {
        if len > 0 {
            // len 1: (byte, byte)
            let a = data[0] as u64;
            U128(a, a)
        } else {
            // len 0
            U128(0, 0)
        }
    }
}

struct MojoAHasher {
    buffer: u64,
    pad: u64,
    extra_keys: U128,
}

impl MojoAHasher {
    fn new(key: [u64; 4]) -> Self {
        let pi_key = [
            key[0] ^ 0x243f_6a88_85a3_08d3,
            key[1] ^ 0x1319_8a2e_0370_7344,
            key[2] ^ 0xa409_3822_299f_31d0,
            key[3] ^ 0x082e_fa98_ec4e_6c89,
        ];
        Self {
            buffer: pi_key[0],
            pad: pi_key[1],
            extra_keys: U128(pi_key[2], pi_key[3]),
        }
    }

    #[inline]
    fn large_update(&mut self, new_data: U128) {
        let combined = folded_multiply(
            new_data.0 ^ self.extra_keys.0,
            new_data.1 ^ self.extra_keys.1,
        );
        // rotate_bits_left[ROT]((buffer + pad) ^ combined)
        self.buffer = (self.buffer.wrapping_add(self.pad) ^ combined).rotate_left(ROT);
    }

    #[inline]
    fn finish(&self) -> u64 {
        let rot = self.buffer & 63;
        let folded = folded_multiply(self.buffer, self.pad);
        folded.rotate_left(rot as u32)
    }

    #[inline]
    fn write(&mut self, data: &[u8]) {
        // self.buffer = (self.buffer + length) * MULTIPLE
        self.buffer = self
            .buffer
            .wrapping_add(data.len() as u64)
            .wrapping_mul(MULTIPLE);

        let len = data.len();
        if len > 8 {
            if len > 16 {
                // tail = last 16 bytes
                let tail_a = u64::from_le_bytes(data[len - 16..len - 8].try_into().unwrap());
                let tail_b = u64::from_le_bytes(data[len - 8..len].try_into().unwrap());
                self.large_update(U128(tail_a, tail_b));

                // process 16-byte blocks from start
                let mut offset = 0usize;
                while len - offset > 16 {
                    let a = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
                    let b =
                        u64::from_le_bytes(data[offset + 8..offset + 16].try_into().unwrap());
                    self.large_update(U128(a, b));
                    offset += 16;
                }
            } else {
                // len 9–16: first 8 + last 8
                let a = u64::from_le_bytes(data[0..8].try_into().unwrap());
                let b = u64::from_le_bytes(data[len - 8..len].try_into().unwrap());
                self.large_update(U128(a, b));
            }
        } else {
            // Mojo write() does large_update(read_small) for <= 8,
            // but your top-level ahash() short path bypasses write().
            let U128(a, b) = read_small(data);
            self.large_update(U128(a, b));
        }
    }
}

pub fn ahash(s: &str) -> u64 {
    let mut hasher = MojoAHasher::new([0, 0, 0, 0]);
    let data = s.as_bytes();

//     println!("data.len(): {}, buff: {}", data.len(), hasher.buffer);

    if data.len() > 8 {
        hasher.write(data);
    } else {
        // EXACT Mojo short path:
        let U128(a, b) = read_small(data);
        hasher.buffer = folded_multiply(a ^ hasher.buffer, b ^ hasher.extra_keys.1);
        hasher.pad = hasher.pad.wrapping_add(data.len() as u64);
    }

//     println!("buff {} pad {}", hasher.buffer, hasher.pad);
    hasher.finish()
}

pub trait StrHash {
    fn hash(&self, s: &str) -> u64;
}

#[derive(Default)]
pub struct MojoAHashStrHash;

impl StrHash for MojoAHashStrHash {
    #[inline]
    fn hash(&self, s: &str) -> u64 {
        ahash(s) // call your custom ahash function
    }
}

#[derive(Default)]
pub struct FxStrHash;

impl StrHash for FxStrHash {
    #[inline]
    fn hash(&self, s: &str) -> u64 {
        let mut hasher = FxHasher::default();
        hasher.write(s.as_bytes());
        hasher.finish()
    }
}

#[derive(Default)]
pub struct AHashStrHash;

impl StrHash for AHashStrHash {
    #[inline]
    fn hash(&self, s: &str) -> u64 {
        let mut hasher = AHasher::default();
        hasher.write(s.as_bytes());
        hasher.finish()
    }
}

