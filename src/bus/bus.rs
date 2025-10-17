use crate::bus::mapper::Mapper;
use crate::bus::{Bus, CPUMemoryMap, PPUMemoryMap};
use crate::cpu::CPU;
use crate::ppu::PPU;

impl Bus {
    pub fn new(mapper: Box<dyn Mapper>) -> Self {
        Self {
            cpu: CPU::new(),
            cpu_memory: CPUMemoryMap::new(mapper),
        }
    }

    pub fn get_cpu(&mut self) -> &mut CPU {
        &mut self.cpu
    }

    pub fn get_cpu_memory(&mut self) -> &mut CPUMemoryMap {
        &mut self.cpu_memory
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        self.get_cpu_memory().bus.get_ppu()
    }

    pub fn get_ppu_memory_map(&'_ mut self) -> PPUMemoryMap<'_> {
        self.cpu_memory.get_ppu_memory()
    }
}