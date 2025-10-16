use crate::ppu::sprites::Sprite;

pub struct OAM<const SIZE: usize> {
    data: [Sprite; SIZE]
}

impl<const SIZE: usize> OAM<SIZE> {
    pub fn new() -> Self {
        Self {
            data: [0; SIZE].map(|_| Sprite::new()),
        }
    }

    pub fn get_sprite(&self, index: usize) -> &Sprite {
        &self.data[index]
    }

    pub fn get_sprite_mut(&mut self, index: usize) -> &mut Sprite {
        &mut self.data[index]
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        self.data[index >> 2].get(index & 3)
    }

    pub fn set_byte(&mut self, index: usize, value: u8) {
        self.data[index >> 2].set(index & 3, value)
    }
}