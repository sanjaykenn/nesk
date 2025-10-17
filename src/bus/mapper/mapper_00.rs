use crate::bus::mapper::Mapper;
use crate::bus::cpu_bus::CPUBus;
use crate::bus::ppu_bus::PPUBus;

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
    fn cpu_read(&mut self, bus: &mut CPUBus, address: u16) -> u8 {
        todo!()
    }

    fn cpu_write(&mut self, bus: &mut CPUBus, address: u16, value: u8) {
        todo!()
    }

    fn ppu_read(&mut self, bus: &mut PPUBus, address: u16) -> u8 {
        todo!()
    }

    fn ppu_write(&mut self, bus: &mut PPUBus, address: u16, value: u8) {
        todo!()
    }
}
