use crate::memory::mapper::Mapper;
use crate::ppu::PPU;

pub struct Memory {
    ppu: PPU,
    ram: [u8; 0x800],
    vram: [u8; 0x800],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ppu: PPU::new(),
            ram: [0; 0x800],
            vram: [0; 0x800],
        }
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        &mut self.ppu
    }
}