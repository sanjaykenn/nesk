use crate::{bit_field, bit_range};

pub struct SpriteAttribute(u8);

impl SpriteAttribute {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get(&self) -> u8 {
        self.0
    }

    pub fn set(&mut self, value: u8) {
        self.0 = value
    }

    bit_range!(get: get_palette @ 0, 2 => u8);
    bit_field!(get: get_priority @ 5);
    bit_field!(get: get_flip_horizontal @ 6);
    bit_field!(get: get_flip_vertical @ 7);
}
