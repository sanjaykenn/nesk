use crate::ppu::background::Background;
use crate::ppu::foreground::Foreground;
use crate::ppu::{PPUMemory, PPURegister, PPU};
use crate::ppu::registers::{Registers, VRAMAddress};
use crate::{Screen, HEIGHT, PIXEL_SIZE, WIDTH};

impl PPU {
    pub fn new(screen: Box<dyn Screen>) -> Self {
        Self {
            screen,
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
            pixels: [[[0; PIXEL_SIZE]; WIDTH]; HEIGHT]
        }
    }

    pub fn get_nmi(&self) -> bool {
        self.nmi
    }

    fn send_nmi(&mut self) {
        self.nmi = true
    }

    pub fn reset_nmi(&mut self) {
        self.nmi = false
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
                let data;
                if self.register.vram_address.get() >= 0x3F00 {
                    self.ppu_data_buffer = self.palette_ram[self.register.vram_address.get() as usize & 0x1F];
                    data = self.ppu_data_buffer
                } else {
                    data = self.ppu_data_buffer;
                    self.ppu_data_buffer = memory.read(self.register.vram_address.get())
                }

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
                    self.palette_ram[self.register.vram_address.get() as usize & 0x1F] = value;
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
}