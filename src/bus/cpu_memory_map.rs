use crate::bus::cpu_bus::CPUBus;
use crate::bus::mapper::Mapper;
use crate::bus::CPUMemoryMap;
use crate::cpu::CPUMemory;

impl CPUMemoryMap {
    pub fn new(mapper: Box<dyn Mapper>) -> Self {
        Self {
            mapper,
            bus: CPUBus::new(),
        }
    }

    pub fn tick_ppu(&'_ mut self) {
        self.bus.tick_ppu(self.mapper.as_mut())
    }
}

impl CPUMemory for CPUMemoryMap {
    fn read(&mut self, address: u16) -> u8 {
        self.mapper.cpu_read(&mut self.bus, address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.mapper.cpu_write(&mut self.bus, address, value);
    }
}
