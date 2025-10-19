use crate::bus::cpu_bus::CPUBus;
use crate::bus::mapper::Mapper;
use crate::cpu::CPUMemory;

pub struct CPUMemoryMap<'a> {
    mapper: &'a mut dyn Mapper,
    bus: CPUBus<'a>,
}

impl<'a> CPUMemoryMap<'a> {
    pub fn new(mapper: &'a mut dyn Mapper, bus: CPUBus<'a>) -> Self {
        Self {
            mapper,
            bus,
        }
    }
}

impl CPUMemory for CPUMemoryMap<'_> {
    fn read(&mut self, address: u16) -> u8 {
        self.mapper.cpu_read(&mut self.bus, address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.mapper.cpu_write(&mut self.bus, address, value);
    }
}
