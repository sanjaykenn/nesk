use crate::bus::{mapper, Bus, CPUMemoryMap};
use crate::controller::Controller;
use crate::cpu::{CPUMemory, CPU};
use crate::ppu::PPU;

impl Bus {
    pub fn from_ines(binary: &[u8]) -> Self {
        let mut cpu_memory = CPUMemoryMap::new(mapper::from_ines(binary));
        let cpu = CPU::new(&mut cpu_memory);

        Self {
            cpu,
            cpu_memory,
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

    pub fn get_controller_1(&mut self) -> &mut Controller {
        self.cpu_memory.bus.get_controller_1()
    }

    pub fn get_controller_2(&mut self) -> &mut Controller {
        self.cpu_memory.bus.get_controller_2()
    }
}