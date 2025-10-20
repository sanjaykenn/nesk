use crate::ppu::background::Background;
use crate::ppu::foreground::Foreground;
use crate::ppu::{colors, PPUMemory, PPURegister, PPU};
use crate::ppu::registers::{Registers, VRAMAddress};
use crate::{HEIGHT, PIXEL_SIZE, WIDTH};

impl PPU {
    pub fn new() -> Self {
        Self {
            register: Registers::new(),
            palette_ram: [0; 0x20],
            nmi: false,
            dma: None,
            ppu_data_buffer: 0,
            transfer_address: VRAMAddress::new(),
            address_latch: false,
            scanline: 261,
            cycle: 0,
            odd_frame: false,
            background: Background::new(),
            foreground: Foreground::new(),
            pixels: [[[0; PIXEL_SIZE]; WIDTH]; HEIGHT],
            render: false,
        }
    }

    pub fn pull_nmi(&mut self) -> bool {
        if self.nmi {
            self.nmi = false;
            true
        } else {
            false
        }
    }

    fn send_nmi(&mut self) {
        self.nmi = true
    }

    pub fn pull_dma(&mut self) -> Option<u8> {
        self.dma.take()
    }

    pub fn read_register(&mut self, register: PPURegister, memory: &mut dyn PPUMemory) -> u8 {
        match register {
            PPURegister::Status => {
                let data = (self.register.status.get() & 0xE0) | (self.ppu_data_buffer & 0x1F);
                self.register.status.set_started_vertical_blank(false);
                self.address_latch = false;
                data
            },
            PPURegister::OAMData => if self.foreground.oam_return_ff() {
                0xFF
            } else {
                self.foreground.get_sprites().get_oam_primary().get_byte(self.register.oam_address as usize)
            },
            PPURegister::VRAMData => {
                let data = if self.register.vram_address.get() >= 0x3F00 {
                    self.palette_ram[Self::get_palette_index(self.register.vram_address.get())]
                } else {
                    self.ppu_data_buffer
                };
                self.ppu_data_buffer = memory.read(self.register.vram_address.get());

                self.register.vram_address.set(self.register.vram_address.get() +
                    if self.register.control.get_vram_increment() {
                        32
                    } else {
                        1
                    }
                );

                data
            },
            _ => self.address_latch as u8
        }
    }

    pub fn write_register(&mut self, register: PPURegister, memory: &mut dyn PPUMemory, value: u8) {
        match register {
            PPURegister::Control => {
                self.register.control.set(value);
                self.transfer_address.set_nametable_x(self.register.control.get_nametable_x());
                self.transfer_address.set_nametable_y(self.register.control.get_nametable_y());

                if self.register.control.get_generate_nmi() && self.register.status.get_started_vertical_blank() {
                    self.send_nmi()
                }
            },
            PPURegister::Mask => self.register.mask.set(value),
            PPURegister::OAMAddress => self.register.oam_address = value,
            PPURegister::OAMData => self.foreground.get_sprites().get_oam_primary().set_byte(self.register.oam_address as usize, value),
            PPURegister::Scroll =>
                if self.address_latch == false {
                    self.register.fine_x = value & 0x07;
                    self.transfer_address.set_tile_x(value as u16 >> 3);
                    self.address_latch = true
                } else {
                    self.transfer_address.set_fine_y(value as u16 & 0x07);
                    self.transfer_address.set_tile_y(value as u16 >> 3);
                    self.address_latch = false
                },
            PPURegister::VRAMAddress => if self.address_latch == false {
                self.transfer_address.set(
                    self.transfer_address.get() & 0x00FF | (value as u16 & 0x3F) << 8
                );
                self.address_latch = true
            } else {
                self.transfer_address.set(
                    value as u16 | self.transfer_address.get() & 0xFF00
                );
                self.register.vram_address.set(self.transfer_address.get());
                self.address_latch = false
            },
            PPURegister::VRAMData => {
                if self.register.vram_address.get() >= 0x3F00 {
                    self.palette_ram[Self::get_palette_index(self.register.vram_address.get())] = value;
                } else {
                    memory.write(self.register.vram_address.get(), value);
                }
                self.register.vram_address.set(self.register.vram_address.get() +
                    if self.register.control.get_vram_increment() {
                        32
                    } else {
                        1
                    }
                );
            },
            PPURegister::DMA => {
                self.dma = Some(value)
            },
            _ => {}
        }
    }

    pub fn write_oam(&mut self, address: u8, value: u8) {
        self.foreground.get_sprites().get_oam_primary().set_byte(address as usize, value)
    }

    fn load_pixel(&mut self, bg_pattern: u8, bg_palette: u8, fg_pattern: u8, fg_palette: u8, fg_priority: bool) -> (u8, u8) {
        if bg_pattern == 0 {
            if fg_pattern == 0 {
                (0, 0)
            } else {
                (fg_pattern, fg_palette)
            }
        } else {
            if fg_pattern == 0 {
                (bg_pattern, bg_palette)
            } else {
                if self.foreground.sprite_zero_active() && self.foreground.show_sprite_zero() {
                    if self.register.mask.get_show_background() && self.register.mask.get_show_sprites() {
                        if !self.register.mask.get_show_background_leftmost_pixels() && !self.register.mask.get_show_sprites_leftmost_pixels() {
                            if self.cycle >= 9 && self.cycle < 258 {
                                self.register.status.set_sprite_0_hit(true)
                            }
                        } else if self.cycle >= 1 && self.cycle < 258 {
                            self.register.status.set_sprite_0_hit(true)
                        }
                    }
                }

                if fg_priority {
                    (fg_pattern, fg_palette)
                } else {
                    (bg_pattern, bg_palette)
                }
            }
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        self.pixels[y][x] = colors::get_color(color, &self.register.mask);
    }

    fn get_palette_index(address: u16) -> usize {
        if address & 3 == 0 {
            address as usize & 0x0F
        } else {
            address as usize & 0x1F
        }
    }

    fn read_palette_ram_index(&self, address: u8) -> u8 {
        let value = self.palette_ram[Self::get_palette_index(address as u16)];

        if self.register.mask.get_grayscale() {
            value & 0x30
        } else {
            value
        }
    }

    pub fn tick(&mut self, memory: &mut dyn PPUMemory) {
        self.foreground.clear_show_sprite_zero();

        if self.scanline == 261 {
            if self.cycle == 1 {
                self.register.status.set_started_vertical_blank(false);
                self.register.status.set_sprite_0_hit(false);
                self.register.status.set_sprite_overflow(false)
            } else if self.cycle >= 280 && self.cycle <= 304 {
                if self.register.mask.is_rendering_enabled() {
                    self.register.vram_address.set_fine_y(self.transfer_address.get_fine_y());
                    self.register.vram_address.set_nametable_y(self.transfer_address.get_nametable_y());
                    self.register.vram_address.set_tile_y(self.transfer_address.get_tile_y())
                }
            }

            self.background.tick(self.cycle, &mut self.register, memory, &self.transfer_address);
        } else if self.scanline < 240 {
            self.background.tick(self.cycle, &mut self.register, memory, &self.transfer_address);
            self.foreground.tick(self.cycle, self.scanline, &mut self.register, memory);
        } else if self.scanline == 241 {
            if self.cycle == 1 {
                self.register.status.set_started_vertical_blank(true);

                if self.register.control.get_generate_nmi() {
                    self.send_nmi()
                }
            }
        }

        if self.cycle >= 1 && self.cycle <= WIDTH && self.scanline < HEIGHT {
            let (bg_pattern, bg_palette) = self.background.load_next_pixel(&mut self.register);
            let (fg_pattern, fg_palette, fg_priority) = self.foreground.load_next_pixel(&mut self.register);

            let (pattern, palette) = self.load_pixel(bg_pattern, bg_palette, fg_pattern, fg_palette | 4, fg_priority);

            self.set_pixel(self.cycle - 1, self.scanline, self.read_palette_ram_index(
                palette << 2 | pattern
            ))
        }

        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline > 261 {
                self.scanline = 0;
                self.render = true;
                self.odd_frame = !self.odd_frame;

                if self.register.mask.is_rendering_enabled() && self.odd_frame {
                    self.cycle = 1
                }
            }
        }
    }

    pub fn get_output(&mut self) -> Option<&[[[u8; PIXEL_SIZE]; WIDTH]; HEIGHT]> {
        if self.render {
            self.render = false;
            Some(&self.pixels)
        } else {
            None
        }
    }
}