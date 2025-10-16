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
            sprite_attribute_bytes: [0; 8].map(|_| SpriteAttribute::new()),
            sprite_x: [0; 8],
            oam_return_ff: false,
            sprites: Sprites::new(),
            show_sprite_zero: false,
            sprite_zero_active: false
        }
    }
}