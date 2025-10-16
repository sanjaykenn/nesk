use crate::ppu::PPUMemory;
use crate::ppu::sprites::{SpriteAttribute, Sprites};

struct Foreground {
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

    fn shift_registers(&mut self, memory: &mut dyn PPUMemory) {
        if memory.get_registers().mask.get_show_sprites() {
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

    fn load_sprites(&mut self, cycle: usize, scanline: usize, memory: &mut dyn PPUMemory) {
        let index = (cycle - 257) / 8;
        let sprite = self.sprites.get_oam_secondary().get_sprite(index);

        match cycle & 7 {
            0 => self.sprite_x[index] = sprite.get_x(),
            3 => self.sprite_attribute_bytes[index] = sprite.get_attribute(),
            5 => self.shifter_patterns_low[index] = sprite.get_pattern_low(scanline, memory),
            7 => self.shifter_patterns_high[index] = sprite.get_pattern_high(scanline, memory),
            _ => {}
        }
    }
}