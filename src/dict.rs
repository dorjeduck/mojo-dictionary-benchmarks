

#[path="./keys_container.rs"]
mod keys_container;
use keys_container::KeysContainer;

use std::convert::{TryFrom, TryInto};

#[path="./ahash.rs"]
pub mod ahash;
// use ahash::{StrHash, FxStrHash, MojoAHashStrHash, AHashStrHash};

/// Open-addressing dictionary with linear probing and 1-based slot_to_index like your Mojo Dict.
/// - V: Copy (to mirror Copyable & Movable)
/// - H: BuildHasher/StrHash (default aHash RandomState)
/// - KC: key-count integer (u32 default)
/// - KO: key-offset integer for KeysContainer (u32 default)
pub struct Dict<
    V: Copy,
    H: ahash::StrHash = ahash::FxStrHash,
    KC: TryInto<usize> + From<u8> + From<u16> + From<u32> + TryFrom<usize> + Copy = u32,
    KO: TryFrom<usize> + Copy + TryInto<usize> = u32,
    const DESTRUCTIVE: bool = true,
    const CACHING_HASHES: bool = true,
> {
    keys: KeysContainer<KO>,
    key_hashes: Option<Vec<KC>>,      // present if CACHING_HASHES
    values: Vec<V>,
    slot_to_index: Vec<KC>,           // 0 = empty, else index+1
    deleted_mask: Option<Vec<u8>>,    // bit per key index if DESTRUCTIVE
    count: usize,                     // active (non-deleted) entries
    capacity: usize,                  // power of two, >= 8
    hasher: H,
}

#[allow(dead_code)]
impl<
        V: Copy,
        H: ahash::StrHash + Default,
        KC: TryInto<usize> + From<u8> + From<u16> + From<u32> + TryFrom<usize> + Copy,
        KO: TryFrom<usize> + Copy + TryInto<usize>,
        const DESTRUCTIVE: bool,
        const CACHING_HASHES: bool,
    > Dict<V, H, KC, KO, DESTRUCTIVE, CACHING_HASHES>
{
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.max(8).next_power_of_two();

        let slot_to_index = vec![KC::try_from(0usize).ok().expect("usize -> KeyEndType conversion failed"); capacity];
        let key_hashes = if CACHING_HASHES {
            Some(vec![KC::try_from(0usize).ok().expect("usize -> KeyEndType conversion failed"); capacity])
        } else {
            None
        };

        let deleted_mask = if DESTRUCTIVE {
            // one bit per key index; we size by capacity/8 like your code (mask for keys)
            Some(vec![0u8; capacity >> 3])
        } else {
            None
        };

        Self {
            keys: KeysContainer::<KO>::new(capacity),
            key_hashes,
            values: Vec::with_capacity(capacity),
            slot_to_index,
            deleted_mask,
            count: 0,
            capacity,
            hasher: H::default(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.count
    }

    #[inline]
    pub fn contains(&self, key: &str) -> bool {
        self.find_key_index(key) != 0
    }

    #[inline]
    fn load_slot(&self, slot: usize) -> usize {
        self.slot_to_index[slot]
                .try_into()
                .ok()
                .expect("KeyEndType -> usize conversion failed")
    }

    #[inline]
    fn store_slot(&mut self, slot: usize, val: usize) {
        self.slot_to_index[slot] = KC::try_from(val).ok().expect("usize -> KeyEndType conversion failed");
    }

    #[inline]
    fn is_deleted(&self, index: usize) -> bool {
        if !DESTRUCTIVE {
            return false;
        }
        let dm = self.deleted_mask.as_ref().unwrap();
        let byte = index >> 3;
        let bit = index & 7;
        (dm[byte] & (1 << bit)) != 0
    }

    #[inline]
    fn set_deleted(&mut self, index: usize) {
        if !DESTRUCTIVE {
            return;
        }
        let dm = self.deleted_mask.as_mut().unwrap();
        let byte = index >> 3;
        let bit = index & 7;
        dm[byte] |= 1 << bit;
    }

    #[inline]
    fn clear_deleted(&mut self, index: usize) {
        if !DESTRUCTIVE {
            return;
        }
        let dm = self.deleted_mask.as_mut().unwrap();
        let byte = index >> 3;
        let bit = index & 7;
        dm[byte] &= !(1 << bit);
    }

    fn maybe_rehash(&mut self) {
        // Mojo used: if self.count / self.capacity >= 0.87 -> rehash
        // We'll emulate with >= 87% load factor.
        if self.count * 100 >= self.capacity * 87 {
            self.rehash();
        }
    }

    fn rehash(&mut self) {
        let old_cap = self.capacity;
        let old_slots = std::mem::take(&mut self.slot_to_index);
        let old_hashes = if CACHING_HASHES {
            std::mem::take(&mut self.key_hashes)
        } else {
            None
        };

        self.capacity <<= 1;
        self.slot_to_index = vec![KC::try_from(0usize).ok().expect("usize -> KeyEndType conversion failed"); self.capacity];

        if DESTRUCTIVE {
            let mut new_mask = vec![0u8; self.capacity >> 3];
            if let Some(old_mask) = self.deleted_mask.as_ref() {
                let to_copy = old_mask.len().min(new_mask.len());
                new_mask[..to_copy].copy_from_slice(&old_mask[..to_copy]);
            }
            self.deleted_mask = Some(new_mask);
        }

        let modulo_mask = self.capacity - 1;
        let mut new_hashes = if CACHING_HASHES {
            Some(vec![KC::try_from(0usize).ok().expect("usize -> KeyEndType conversion failed"); self.capacity])
        } else {
            None
        };

        for i in 0..old_cap {
            let key_index: usize = old_slots[i]
                .try_into()
                .ok()
                .expect("KeyEndType -> usize conversion failed");
            if key_index == 0 {
                continue;
            }

            let idx0 = key_index - 1;
            let k = self.keys.get(idx0).unwrap();

            // pull cached (already truncated) or recompute and truncate
            let key_hash_truncated: usize = if CACHING_HASHES {
                old_hashes.as_ref().unwrap()[i]
                    .try_into()
                    .ok()
                    .expect("KC -> usize conversion failed")
            } else {
                let h = self.hasher.hash(k);
                (h as usize) & ((1 << (std::mem::size_of::<KC>() * 8)) - 1)
            };

            let mut slot = key_hash_truncated & modulo_mask;
            loop {
                if self.load_slot(slot) == 0 {
                    self.store_slot(slot, key_index);
                    if CACHING_HASHES {
                        new_hashes.as_mut().unwrap()[slot] =
                            KC::try_from(key_hash_truncated).ok().expect("usize -> KeyEndType conversion failed");
                    }
                    break;
                }
                slot = (slot + 1) & modulo_mask;
            }
        }

        if CACHING_HASHES { self.key_hashes = new_hashes; }
        let _ = old_hashes;
    }


    pub fn put(&mut self, key: &str, value: V) {
        self.maybe_rehash();

        let key_hash_u64 = self.hasher.hash(key);
        let key_hash_truncated =
            (key_hash_u64 as usize) & ((1 << (std::mem::size_of::<KC>() * 8)) - 1);

        let modulo_mask = self.capacity - 1;
        let mut slot = key_hash_truncated & modulo_mask;

        loop {
            let key_index = self.load_slot(slot);
            if key_index == 0 {
                // insert fresh
                self.keys.add(key);
                if CACHING_HASHES {
                    self.key_hashes.as_mut().unwrap()[slot] =
                        KC::try_from(key_hash_truncated).ok().expect("usize -> KeyEndType conversion failed");
                }
                self.values.push(value);
                self.store_slot(slot, self.keys.len()); // 1-based
                self.count += 1;
                return;
            }

            // collision path
            if CACHING_HASHES {
                let other_hash: usize = self.key_hashes.as_ref().unwrap()[slot]
                    .try_into().ok().expect("KC -> usize conversion failed");
                if other_hash == key_hash_truncated {
                    let other_key = self.keys.get(key_index - 1).unwrap();
                    if other_key == key {
                        // replace value
                        let idx0 = key_index - 1;
                        self.values[idx0] = value;
                        if DESTRUCTIVE && self.is_deleted(idx0) {
                            self.count += 1;
                            self.clear_deleted(idx0);
                        }
                        return;
                    }
                }
            } else {
                let other_key = self.keys.get(key_index - 1).unwrap();
                if other_key == key {
                    let idx0 = key_index - 1;
                    self.values[idx0] = value;
                    if DESTRUCTIVE && self.is_deleted(idx0) {
                        self.count += 1;
                        self.clear_deleted(idx0);
                    }
                    return;
                }
            }

            slot = (slot + 1) & modulo_mask;
        }
    }

    pub fn get_or(&self, key: &str, default: V) -> V {
        let key_index = self.find_key_index(key);
        if key_index == 0 {
            return default;
        }
        if DESTRUCTIVE {
            if self.is_deleted(key_index - 1) {
                return default;
            }
        }
//         key_index -= 1;
//         println!("g {} / {}", key, key_index);
        self.values[key_index - 1]
    }

    pub fn calc(&mut self, key: &str, f: impl Fn(V) -> V) {
        let key_index = self.find_key_index(key);
        if key_index != 0 {
            let idx0 = key_index - 1;
            self.values[idx0] = f(self.values[idx0]);
        }
    }

    pub fn delete(&mut self, key: &str) {
        if !DESTRUCTIVE {
            return;
        }
        let key_index = self.find_key_index(key);
        if key_index == 0 {
            return;
        }
        let idx0 = key_index - 1;
        if !self.is_deleted(idx0) {
            self.count -= 1;
        }
        self.set_deleted(idx0);
    }

    pub fn upsert(&mut self, key: &str, update: impl Fn(Option<V>) -> V) {
        let mut key_index = self.find_key_index(key);
        if key_index == 0 {
            let v = update(None);
            self.put(key, v);
            return;
        }
        key_index -= 1;

        if DESTRUCTIVE && self.is_deleted(key_index) {
            self.values[key_index] = update(None);
            return;
        }
        self.values[key_index] = update(Some(self.values[key_index]));
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.keys.clear();
        for x in &mut self.slot_to_index {
            *x = KC::try_from(0usize).ok().expect("usize -> KeyEndType conversion failed");
        }
        if DESTRUCTIVE {
            for b in self.deleted_mask.as_mut().unwrap().iter_mut() {
                *b = 0;
            }
        }
        self.count = 0;
    }

    #[inline]
    fn find_key_index(&self, key: &str) -> usize {
        let key_hash_u64 = self.hasher.hash(key);
        // Truncate to KC width, just like in put()
        let key_hash_truncated = (key_hash_u64 as usize) & ((1 << (std::mem::size_of::<KC>() * 8)) - 1);

        let modulo_mask = self.capacity - 1;
        let mut slot = key_hash_truncated & modulo_mask;

        loop {
            let key_index = self.load_slot(slot);
            if key_index == 0 {
                return 0;
            }

            if CACHING_HASHES {
                let other_hash: usize = self.key_hashes.as_ref().unwrap()[slot]
                    .try_into().ok().expect("KC -> usize conversion failed");
                if other_hash == key_hash_truncated {
                    let other_key = self.keys.get(key_index - 1).unwrap();
                    if other_key == key {
                        return key_index;
                    }
                }
            } else {
                let other_key = self.keys.get(key_index - 1).unwrap();
                if other_key == key {
                    return key_index;
                }
            }

            slot = (slot + 1) & modulo_mask;
        }
    }


    /// Debug print similar to your `debug()` method.
    pub fn debug(&self) {
        println!("Dict count: {} and capacity: {}", self.count, self.capacity);
        println!("KeyMap:");
        for i in 0..self.capacity {
            print!(
                "{}{}",
                self.slot_to_index[i]
                 .try_into().ok().expect("KC -> usize conversion failed"),
                if i + 1 < self.capacity { ", " } else { "\n" }
            );
        }
        println!("Keys:");
        print!("({})[", self.keys.len());
        for i in 0..self.keys.len() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", self.keys.get(i).unwrap());
        }
        println!("]");
        if CACHING_HASHES {
            println!("KeyHashes:");
            for i in 0..self.capacity {
                let v = if self.load_slot(i) > 0 {
                    (self.key_hashes.as_ref().unwrap()[i]
                     .try_into().ok().expect("KC -> usize conversion failed")) as usize
                } else {
                    0
                };
                print!(
                    "{}{}",
                    v,
                    if i + 1 < self.capacity { ", " } else { "\n" }
                );
            }
        }
    }
}
