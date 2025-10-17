use crate::{bit_field, bit_range};
use crate::ppu::oam::OAM;
use crate::ppu::{memory, PPUMemory};
use crate::ppu::registers::Registers;
use crate::ppu::utils::flip_byte;

#[derive(Clone, Copy)]
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

    pub fn get_attribute(&self) -> SpriteAttribute {
        self.attribute
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    fn get_pattern_table_index(&self, scanline: usize, registers: &Registers) -> (bool, u16, u16) {
        let y_dif = (scanline as u16).checked_sub(self.get_y() as u16).unwrap_or_default();

        if !registers.control.get_sprite_size() {
            if !self.get_attribute().get_flip_vertical() {
                (registers.control.get_sprite_pattern_table(), self.get_id() as u16, y_dif)
            } else {
                (registers.control.get_sprite_pattern_table(), self.get_id() as u16, 7 - y_dif)
            }
        } else {
            if !self.get_attribute().get_flip_vertical() {
                if y_dif < 8 {
                    (self.get_id() & 0x01 != 0, self.get_id() as u16 & 0xFE, y_dif)
                } else {
                    (self.get_id() & 0x01 != 0, self.get_id() as u16 | 0x01, y_dif & 0x07)
                }
            } else {
                if y_dif < 8 {
                    (self.get_id() & 0x01 != 0, self.get_id() as u16 | 0x01, 7 - y_dif)
                } else {
                    (self.get_id() & 0x01 != 0, self.get_id() as u16 & 0xFE, 15 - y_dif)
                }
            }
        }
    }

    pub fn get_pattern_low(&self, scanline: usize, registers: &Registers, memory: &mut dyn PPUMemory) -> u8 {
        let (table, tile, y) = self.get_pattern_table_index(scanline, registers);
        let pattern = memory::read_pattern_table_tile_low(memory, table, tile, y);

        if self.get_attribute().get_flip_horizontal() {
            flip_byte(pattern)
        } else {
            pattern
        }
    }

    pub fn get_pattern_high(&self, scanline: usize, registers: &Registers, memory: &mut dyn PPUMemory) -> u8 {
        let (table, tile, y) = self.get_pattern_table_index(scanline, registers);
        let pattern = memory::read_pattern_table_tile_high(memory, table, tile, y);

        if self.get_attribute().get_flip_horizontal() {
            flip_byte(pattern)
        } else {
            pattern
        }
    }
}

enum SpriteEvaluationState {
    LoadY,
    LoadSprite,
    Overflow,
    End
}

pub struct Sprites {
    sprite_evaluation_state: SpriteEvaluationState,
    oam_primary: OAM<64>,
    oam_secondary: OAM<8>,
    oam_index: usize,
    sprite_index: usize,
    sprite_count: usize,
    sprite_zero_active: bool,
    overflow: bool,
    scanline: usize
}

impl Sprites {
    pub fn new() -> Self {
        Self {
            sprite_evaluation_state: SpriteEvaluationState::LoadY,
            oam_primary: OAM::new(),
            oam_secondary: OAM::new(),
            oam_index: 0,
            sprite_index: 0,
            sprite_count: 0,
            sprite_zero_active: false,
            overflow: false,
            scanline: 0
        }
    }

    pub fn reset_evaluation(&mut self, scanline: usize) {
        self.sprite_evaluation_state = SpriteEvaluationState::LoadY;
        self.oam_index = 0;
        self.sprite_index = 0;
        self.sprite_count = 0;
        self.sprite_zero_active = false;
        self.overflow = false;
        self.scanline = scanline
    }

    pub fn get_oam_primary(&mut self) -> &mut OAM<64> {
        &mut self.oam_primary
    }

    pub fn get_oam_secondary(&mut self) -> &mut OAM<8> {
        &mut self.oam_secondary
    }

    pub fn is_sprite_zero_active(&self) -> bool {
        self.sprite_zero_active
    }

    pub fn is_overflowing(&self) -> bool {
        self.overflow
    }

    pub fn evaluate(&mut self, sprite_size: usize) {
        self.sprite_evaluation_state = match self.sprite_evaluation_state {
            SpriteEvaluationState::LoadY => {
                let y = self.oam_primary
                    .get_sprite(self.oam_index)
                    .get(self.sprite_index);

                if self.sprite_in_scanline(y, sprite_size) {
                    self.oam_secondary
                        .get_sprite_mut(self.sprite_count)
                        .set(self.sprite_index, y);

                    if self.oam_index == 0 {
                        self.sprite_zero_active = true
                    }

                    SpriteEvaluationState::LoadSprite
                } else {
                    self.increment_oam_index()
                }
            },
            SpriteEvaluationState::LoadSprite => {
                let data = self.oam_primary
                    .get_sprite(self.oam_index)
                    .get(self.sprite_index);

                self.oam_secondary
                    .get_sprite_mut(self.sprite_count)
                    .set(self.sprite_index, data);

                if self.sprite_index == 3 {
                    self.sprite_index = 0;
                    self.sprite_count += 1;
                    self.increment_oam_index()
                } else {
                    self.sprite_index += 1;
                    SpriteEvaluationState::LoadSprite
                }
            },
            SpriteEvaluationState::Overflow => {
                let y = self.oam_primary
                    .get_sprite(self.oam_index)
                    .get(self.sprite_index);

                if self.sprite_in_scanline(y, sprite_size) {
                    self.overflow = true;
                    SpriteEvaluationState::End
                } else {
                    self.oam_index += 1;
                    self.sprite_index = (self.sprite_index + 1) & 3;

                    if self.oam_index > 63 {
                        SpriteEvaluationState::End
                    } else {
                        SpriteEvaluationState::Overflow
                    }
                }
            },
            SpriteEvaluationState::End => SpriteEvaluationState::End
        }
    }

    fn sprite_in_scanline(&self, sprite_y: u8, sprite_size: usize) -> bool {
        let y = sprite_y as usize;
        y <= self.scanline && y + sprite_size > self.scanline
    }

    fn increment_oam_index(&mut self) -> SpriteEvaluationState {
        self.oam_index += 1;

        if self.oam_index >= 64 {
            SpriteEvaluationState::End
        } else if self.sprite_count < 8 {
            SpriteEvaluationState::LoadY
        } else {
            SpriteEvaluationState::Overflow
        }
    }
}