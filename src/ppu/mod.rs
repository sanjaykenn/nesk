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
mod memory;
mod colors;

pub struct PPU {
    screen: Box<dyn Screen>,
    register: Registers,
    palette_ram: [u8; 0x20],
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
}

pub enum PPURegister {
    Control,
    Mask,
    Status,
    OAMAddress,
    OAMData,
    Scroll,
    VRAMAddress,
    VRAMData,
    DMA
}
