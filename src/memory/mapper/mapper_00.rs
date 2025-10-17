use crate::memory::mapper::Mapper;
use crate::memory::memory::Memory;

struct Mapper00 {
    horizontal_flip: bool,
}

impl Mapper00 {
    pub fn new(horizontal_flip: bool) -> Self {
        Self {
            horizontal_flip,
        }
    }
}

impl Mapper for Mapper00 {
    fn read_cpu(&mut self, memory: &mut Memory, addr: u16) -> u8 {
        todo!()
    }

    fn write_cpu(&mut self, memory: &mut Memory, addr: u16, value: u8) {
        todo!()
    }

    fn read_ppu(&mut self, memory: &mut Memory, addr: u16) -> u8 {
        todo!()
    }

    fn write_ppu(&mut self, memory: &mut Memory, addr: u16, value: u8) {
        todo!()
    }
}
