use crate::ppu::background::Background;
use crate::ppu::foreground::Foreground;
use crate::ppu::PPU;
use crate::ppu::registers::{Registers, VRAMAddress};
use crate::{Screen, HEIGHT, PIXEL_SIZE, WIDTH};

impl PPU {
    pub fn new(screen: Box<dyn Screen>) -> Self {
        Self {
            screen,
            register: Registers::new(),
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
}