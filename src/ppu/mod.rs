use crate::ppu::background::Background;
use crate::ppu::foreground::Foreground;
use crate::ppu::registers::{Registers, VRAMAddress};
use crate::{Screen, HEIGHT, PIXEL_SIZE, WIDTH};

mod utils;
mod registers;
mod background;
mod sprites;
mod oam;
mod foreground;
mod ppu;

pub struct PPU {
    screen: Box<dyn Screen>,
    register: Registers,
    nmi: bool,
    dma: Option<u8>,
    scanline: usize,
    cycle: usize,
    odd_frame: bool,
    ppu_data_buffer: u8,
    transfer_address: VRAMAddress,
    address_latch: bool,
    background: Background,
    foreground: Foreground,
    pixels: [[[u8; PIXEL_SIZE]; WIDTH]; HEIGHT]
}

pub trait PPUMemory {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);

    fn read_nametable(&mut self, address: u16) -> u8 {
        self.read(0x2000 | address & 0x0FFF)
    }

    fn read_attribute_table(&mut self, nametable: u16, address: u16) -> u8 {
        self.read(0x23C0 | nametable << 10 | address)
    }

    fn read_pattern_table_tile_low(&mut self, pattern_table: bool, tile: u16, y: u16) -> u8 {
        self.read(if !pattern_table { 0 } else { 0x1000 } | tile << 4 | y)
    }

    fn read_pattern_table_tile_high(&mut self, pattern_table: bool, tile: u16, y: u16) -> u8 {
        self.read(if !pattern_table { 0 } else { 0x1000 } | tile << 4 | y | 8)
    }
}
