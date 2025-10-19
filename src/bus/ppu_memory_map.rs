use crate::bus::mapper::Mapper;
use crate::bus::ppu_bus::PPUBus;
use crate::ppu::PPUMemory;

pub struct PPUMemoryMap<'a> {
    mapper: &'a mut dyn Mapper,
    bus: PPUBus<'a>
}

impl<'a> PPUMemoryMap<'a> {
    pub fn new(mapper: &'a mut dyn Mapper, bus: PPUBus<'a>) -> Self {
        Self {
            mapper,
            bus
        }
    }
}

impl PPUMemory for PPUMemoryMap<'_> {
    fn read(&mut self, address: u16) -> u8 {
        self.mapper.ppu_read(&mut self.bus, address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.mapper.ppu_write(&mut self.bus, address, value)
    }
}
