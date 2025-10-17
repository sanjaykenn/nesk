use crate::bus::{mapper, Bus, CPUMemoryMap};
use crate::cpu::{CPUMemory, CPU};
use crate::ppu::PPU;

impl Bus {
    pub fn from_ines(path: &str) -> Self {
        Self {
            cpu: CPU::new(),
            cpu_memory: CPUMemoryMap::new(mapper::from_ines(path)),
        }
    }

    pub fn get_cpu(&mut self) -> &mut CPU {
        &mut self.cpu
    }

    pub fn tick_cpu(&mut self) {
        self.cpu.tick(&mut self.cpu_memory)
    }

    pub fn get_cpu_memory(&mut self) -> &mut dyn CPUMemory {
        &mut self.cpu_memory
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        self.cpu_memory.bus.get_ppu()
    }

    pub fn tick_ppu(&'_ mut self) {
        self.cpu_memory.tick_ppu()
    }
}