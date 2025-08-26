use std::str;
use std::convert::{TryFrom, TryInto};

/// Container that stores strings in a contiguous `u8` buffer
/// and keeps track of the end offsets in a separate array.
pub struct KeysContainer<KeyEndType = u32> {
    keys: Vec<u8>,
    keys_end: Vec<KeyEndType>,
    count: usize,
}

#[allow(dead_code)]
impl<KeyEndType> KeysContainer<KeyEndType>
where
    KeyEndType: Copy
        + TryFrom<usize>
        + TryInto<usize>,
{
    pub fn new(capacity: usize) -> Self {
        KeysContainer {
            keys: Vec::with_capacity(capacity * 8),
            keys_end: Vec::with_capacity(capacity),
            count: 0,
        }
    }

    pub fn add(&mut self, key: &str) {
        let prev_end: usize = if self.count == 0 {
            0
        } else {
            self.keys_end[self.count - 1]
                .try_into()
                .ok()
                .expect("KeyEndType -> usize conversion failed")
        };

        let key_bytes = key.as_bytes();
        let new_end = prev_end + key_bytes.len();

        if new_end > self.keys.capacity() {
            let new_cap = self.keys.capacity() + (self.keys.capacity() >> 1).max(8);
            self.keys.reserve(new_cap - self.keys.capacity());
        }

        self.keys.extend_from_slice(key_bytes);

        if self.count >= self.keys_end.capacity() {
            let new_cap = self.keys_end.capacity() + (self.keys_end.capacity() >> 1).max(1);
            self.keys_end.reserve(new_cap - self.keys_end.capacity());
        }

        self.keys_end.push(
            KeyEndType::try_from(new_end).ok().expect("usize -> KeyEndType conversion failed"),
        );
        self.count += 1;
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        if index >= self.count {
            return None;
        }
        let start: usize = if index == 0 {
            0
        } else {
            self.keys_end[index - 1]
                .try_into()
                .ok()
                .expect("KeyEndType -> usize conversion failed")
        };
        let end: usize = self.keys_end[index]
            .try_into()
            .ok()
            .expect("KeyEndType -> usize conversion failed");

        let slice = &self.keys[start..end];
        Some(unsafe { str::from_utf8_unchecked(slice) })
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.keys_end.clear();
        self.count = 0;
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn keys_vec(&self) -> Vec<&str> {
//         (0..self.count).map(|i| Some(str::from_utf8(self.get(i)).unwrap()).unwrap()).collect()
        (0..self.count).map(|i| self.get(i).unwrap()).collect()
    }

    pub fn print_keys(&self) {
        print!("({})[", self.count);
        for (i, k) in self.keys_vec().iter().enumerate() {
            if i < self.count - 1 {
                print!("{}, ", k);
            } else {
                print!("{}", k);
            }
        }
        println!("]");
    }
}
// &[u8]         81.54128ms
// safe &str    689.82759ms
// unsafe &str   86.77004ms