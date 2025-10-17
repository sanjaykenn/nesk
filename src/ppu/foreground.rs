use crate::ppu::PPUMemory;
use crate::ppu::registers::Registers;
use crate::ppu::sprites::{SpriteAttribute, Sprites};

pub struct Foreground {
    shifter_patterns_low: [u8; 8],
    shifter_patterns_high: [u8; 8],
    sprite_attribute_bytes: [SpriteAttribute; 8],
    sprite_x: [u8; 8],
    oam_return_ff: bool,
    sprites: Sprites,
    show_sprite_zero: bool,
    sprite_zero_active: bool
}

impl Foreground {
    pub fn new() -> Self {
        Self {
            shifter_patterns_low: [0; 8],
            shifter_patterns_high: [0; 8],
            sprite_attribute_bytes: [SpriteAttribute::new(); 8],
            sprite_x: [0; 8],
            oam_return_ff: false,
            sprites: Sprites::new(),
            show_sprite_zero: false,
            sprite_zero_active: false
        }
    }

    fn tick(&mut self, cycle: usize, scanline: usize, registers: &mut Registers, memory: &mut dyn PPUMemory) {
        if cycle >= 2 && cycle < 258 {
            self.shift_registers(registers)
        }

        if cycle == 0 {
            self.sprite_zero_active = self.sprites.is_sprite_zero_active();
            self.sprites.reset_evaluation(scanline)
        } else if cycle <= 64 {
            self.oam_return_ff = true;

            if cycle & 1 != 0 {
                self.sprites.get_oam_secondary().set_byte(cycle - 1 >> 1, 0xFF)
            }
        } else if cycle <= 256 {
            self.oam_return_ff = false;
            self.sprites.evaluate(if registers.control.get_sprite_size() { 16 } else { 8 });
            registers.status.set_sprite_overflow(self.sprites.is_overflowing())
        } else if cycle <= 320 {
            self.load_sprites(cycle, scanline, registers, memory)
        }
    }

    fn shift_registers(&mut self, registers: &Registers, ) {
        if registers.mask.get_show_sprites() {
            for i in 0..self.sprite_x.len() {
                if self.sprite_x[i] > 0 {
                    self.sprite_x[i] -= 1;
                } else {
                    self.shifter_patterns_low[i] <<= 1;
                    self.shifter_patterns_high[i] <<= 1;
                }
            }
        }
    }

    fn load_sprites(&mut self, cycle: usize, scanline: usize, registers: &Registers, memory: &mut dyn PPUMemory) {
        let index = (cycle - 257) / 8;
        let sprite = self.sprites.get_oam_secondary().get_sprite(index);

        match cycle & 7 {
            0 => self.sprite_x[index] = sprite.get_x(),
            3 => self.sprite_attribute_bytes[index] = sprite.get_attribute(),
            5 => self.shifter_patterns_low[index] = sprite.get_pattern_low(scanline, registers, memory),
            7 => self.shifter_patterns_high[index] = sprite.get_pattern_high(scanline, registers, memory),
            _ => {}
        }
    }

    fn load_next_pixel(&mut self, registers: &Registers) -> (u8, u8, bool) {
        if registers.mask.get_show_sprites() {
            for i in 0..self.sprite_x.len() {
                if self.sprite_x[i] == 0 {
                    let pixel = self.shifter_patterns_high[i] >> 6 & 2 |
                        self.shifter_patterns_low[i] >> 7 & 1;

                    if pixel != 0 {
                        if i == 0 {
                            self.show_sprite_zero = true;
                        }

                        let sprite_palette = self.sprite_attribute_bytes[i].get_palette();
                        let sprite_priority = !self.sprite_attribute_bytes[i].get_priority();
                        return (pixel, sprite_palette, sprite_priority);
                    }
                }
            }
        }

        (0, 0, false)
    }

    pub fn oam_return_ff(&self) -> bool {
        self.oam_return_ff
    }

    pub fn get_sprites(&mut self) -> &mut Sprites {
        &mut self.sprites
    }
}