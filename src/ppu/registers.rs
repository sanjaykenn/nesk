use crate::{bit_field, bit_range};

pub struct Registers {
    pub control: Control,
    pub mask: Mask,
    pub status: Status,
    pub oam_address: u8,
    pub fine_x: u8,
    pub vram_address: VRAMAddress
}

impl Registers {
    pub fn new() -> Self {
        Self {
            control: Control::new(),
            mask: Mask::new(),
            status: Status::new(),
            oam_address: 0,
            fine_x: 0,
            vram_address: VRAMAddress::new()
        }
    }

    pub fn increment_horizontal(&mut self) {
        if self.mask.is_rendering_enabled() {
            if self.vram_address.get_tile_x() == 31 {
                self.vram_address.set_tile_x(0);
                self.vram_address.set_nametable_x(!self.vram_address.get_nametable_x())
            } else {
                self.vram_address.set_tile_x(self.vram_address.get_tile_x() + 1)
            }
        }
    }

    pub fn increment_vertical(&mut self) {
        if self.mask.is_rendering_enabled() {
            if self.vram_address.get_fine_y() < 7 {
                self.vram_address.set_fine_y(self.vram_address.get_fine_y() + 1)
            } else {
                self.vram_address.set_fine_y(0);

                match self.vram_address.get_tile_y() {
                    29 => {
                        self.vram_address.set_tile_y(0);
                        self.vram_address.set_nametable_y(!self.vram_address.get_nametable_y())
                    },
                    31 => self.vram_address.set_tile_y(0),
                    _ => self.vram_address.set_tile_y(self.vram_address.get_tile_y() + 1)
                }
            }
        }
    }

    pub fn get_palette_from_attribute(&self, attribute: u8) -> u8 {
        let mut palette = attribute;
        if self.vram_address.get_tile_x() & 2 != 0 {
            palette >>= 2
        }

        if self.vram_address.get_tile_y() & 2 != 0 {
            palette >>= 4
        }

        palette & 3
    }
}

pub struct Control(u8);

impl Control {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn set(&mut self, value: u8) {
        self.0 = value
    }

    bit_field!(get: get_nametable_x @ 0);
    bit_field!(get: get_nametable_y @ 1);
    bit_field!(get: get_vram_increment @ 2);
    bit_field!(get: get_sprite_pattern_table @ 3);
    bit_field!(get: get_background_pattern_table @ 4);
    bit_field!(get: get_sprite_size @ 5);
    bit_field!(get: get_generate_nmi @ 7);
}

pub struct Mask(u8);

impl Mask {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn set(&mut self, value: u8) {
        self.0 = value
    }

    pub fn is_rendering_enabled(&self) -> bool {
        self.get_show_sprites() || self.get_show_background()
    }

    bit_field!(get: get_grayscale @ 0);
    bit_field!(get: get_show_background_leftmost_pixels @ 1);
    bit_field!(get: get_show_sprites_leftmost_pixels @ 2);
    bit_field!(get: get_show_background @ 3);
    bit_field!(get: get_show_sprites @ 4);
    bit_field!(get: get_emphasize_red @ 5);
    bit_field!(get: get_emphasize_green @ 6);
    bit_field!(get: get_emphasize_blue @ 7);
}

pub struct Status(u8);

impl Status {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get(&self) -> u8 {
        self.0
    }

    bit_field!(set: set_sprite_overflow @ 5);
    bit_field!(set: set_sprite_0_hit @ 6);
    bit_field!(get_started_vertical_blank, set_started_vertical_blank @ 7);
}

pub struct VRAMAddress(u16);

impl VRAMAddress {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, value: u16) {
        self.0 = value
    }

    pub fn get_attribute_index(&self) -> u16 {
        self.get_attribute_y() << 3 | self.get_attribute_x()
    }

    bit_range!(get_tile_x, set_tile_x @ 0, 5 => u16);
    bit_range!(get_tile_y, set_tile_y @ 5, 10 => u16);

    bit_range!(get: get_attribute_x @ 2, 5 => u16);
    bit_range!(get: get_attribute_y @ 7, 10 => u16);

    bit_field!(get_nametable_x, set_nametable_x @ 10);
    bit_field!(get_nametable_y, set_nametable_y @ 11);

    bit_range!(get: get_nametable @ 10, 2 => u16);
    bit_range!(get_fine_y, set_fine_y @ 12, 3 => u16);
}
