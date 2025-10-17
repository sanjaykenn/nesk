use crate::cpu::CPUMemory;
use crate::memory::Bus;
use crate::memory::mapper::Mapper;
use crate::memory::memory::Memory;
use crate::ppu::{PPUMemory, PPU};

impl Bus {
    pub fn new(memory: Memory, mapper: Box<dyn Mapper>) -> Self {
        Self {
            memory,
            mapper
        }
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        self.memory.get_ppu()
    }
}

impl CPUMemory for Bus {
    fn read(&mut self, address: u16) -> u8 {
        self.mapper.read_cpu(&mut self.memory, address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.mapper.write_cpu(&mut self.memory, address, value)
    }
}

impl PPUMemory for Bus {
    fn read(&mut self, address: u16) -> u8 {
        self.mapper.read_ppu(&mut self.memory, address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.mapper.write_ppu(&mut self.memory, address, value)
    }
}
