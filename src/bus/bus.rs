use crate::bus::cpu_bus::CPUBus;
use crate::bus::cpu_memory_map::CPUMemoryMap;
use crate::bus::mapper;
use crate::bus::mapper::Mapper;
use crate::bus::ppu_bus::PPUBus;
use crate::bus::ppu_memory_map::PPUMemoryMap;
use crate::controller::Controller;
use crate::cpu::{CPUMemory, CPU};
use crate::ppu::PPU;

pub struct Bus {
    mapper: Box<dyn Mapper>,
    cpu: CPU,
    ppu: PPU,
    controller_1: Controller,
    controller_2: Controller,
    ram: [u8; 0x800],
    vram: [u8; 0x800],
}

impl Bus {
    pub fn from_ines(binary: &[u8]) -> Result<Self, String> {
        let mut result = Self {
            mapper: mapper::from_ines(binary)?,
            cpu: CPU::new(),
            ppu: PPU::new(),
            controller_1: Controller::new(),
            controller_2: Controller::new(),
            ram: [0; 0x800],
            vram: [0; 0x800],
        };

        result.init_cpu_program_counter();

        Ok(result)
    }

    fn init_cpu_program_counter(&mut self) {
        self.cpu.init_program_counter(&mut CPUMemoryMap::new(
            self.mapper.as_mut(),
            CPUBus::new(
                &mut self.ppu,
                &mut self.controller_1,
                &mut self.controller_2,
                &mut self.ram,
                &mut self.vram
            )
        ));
    }

    pub fn get_cpu(&mut self) -> &mut CPU {
        &mut self.cpu
    }

    pub fn tick_cpu(&mut self) {
        self.cpu.tick(&mut CPUMemoryMap::new(
            self.mapper.as_mut(),
            CPUBus::new(
                &mut self.ppu,
                &mut self.controller_1,
                &mut self.controller_2,
                &mut self.ram,
                &mut self.vram
            )
        ))
    }

    pub fn get_cpu_memory<'a>(&'a mut self) -> Box<dyn CPUMemory + 'a> {
        Box::new(CPUMemoryMap::new(
            self.mapper.as_mut(),
            CPUBus::new(
                &mut self.ppu,
                &mut self.controller_1,
                &mut self.controller_2,
                &mut self.ram,
                &mut self.vram
            )
        ))
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        &mut self.ppu
    }

    pub fn tick_ppu(&mut self) {
        self.ppu.tick(&mut PPUMemoryMap::new(
            self.mapper.as_mut(),
            PPUBus::new(&mut self.vram)
        ))
    }

    pub fn tick_apu(&mut self) {

    }

    pub fn get_controller_1(&mut self) -> &mut Controller {
        &mut self.controller_1
    }

    pub fn get_controller_2(&mut self) -> &mut Controller {
        &mut self.controller_2
    }
}