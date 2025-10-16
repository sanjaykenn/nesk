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

pub struct Sprite {
    y: u8,
    id: u8,
    attribute: SpriteAttribute,
    x: u8
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            y: 0,
            id: 0,
            attribute: SpriteAttribute::new(),
            x: 0,
        }
    }

    pub fn get(&self, index: usize) -> u8 {
        match index {
            0 => self.y,
            1 => self.id,
            2 => self.attribute.get(),
            3 => self.x,
            _ => panic!("Sprite has only 4 attributes!")
        }
    }

    pub fn set(&mut self, index: usize, value: u8) {
        match index {
            0 => self.y = value,
            1 => self.id = value,
            2 => self.attribute.set(value),
            3 => self.x = value,
            _ => panic!("Sprite has only 4 attributes!")
        }
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_attribute(&self) -> &SpriteAttribute {
        &self.attribute
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }
}