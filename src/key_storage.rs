const KEY: &[u8] = &[
    0xDA, 0xDF, 0xDD, 0x05, 0x53, 0x40, 0x45, 0xb3, 0xEE, 0xCC, 0x26, 0x5E, 0xB8, 0x0B, 0x25, 0xDB,
    0xA2, 0xE6, 0xEC, 0x63, 0xF2, 0xE1, 0x19, 0x76, 0x08, 0x43, 0x38, 0x6F, 0xC5, 0xC1, 0x85, 0x46,
];

pub trait AbstractKeyStorage: Default {
    fn next_key(&mut self) -> u8;
}

pub struct SimpleKeyStorage {
    index: usize,
    data: &'static [u8],
}

impl SimpleKeyStorage {
    pub fn new(data: &'static [u8]) -> Self {
        SimpleKeyStorage { index: 0, data }
    }
}

impl Default for SimpleKeyStorage {
    fn default() -> Self {
        Self::new(&KEY)
    }
}

impl AbstractKeyStorage for SimpleKeyStorage {
    fn next_key(&mut self) -> u8 {
        let item = self.data[self.index];

        if self.index + 1 == self.data.len() {
            self.index = 0;
        } else {
            self.index += 1;
        }

        item
    }
}
