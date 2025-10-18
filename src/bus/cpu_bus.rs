use crate::bus::mapper::Mapper;
use crate::bus::ppu_bus::PPUBus;
use crate::bus::PPUMemoryMap;
use crate::controller::Controller;
use crate::ppu::{PPURegister, PPU};

pub struct CPUBus {
    ram: [u8; 0x800],
    ppu: PPU,
    ppu_bus: PPUBus,
    controller_1: Controller,
    controller_2: Controller,
}

impl CPUBus {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x800],
            ppu: PPU::new(),
            ppu_bus: PPUBus::new(),
            controller_1: Controller::new(),
            controller_2: Controller::new(),
        }
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        &mut self.ppu
    }

    pub fn tick_ppu(&mut self, mapper: &mut dyn Mapper) {
        let mut memory_map = PPUMemoryMap::new(mapper, &mut self.ppu_bus);
        self.ppu.tick(&mut memory_map);
    }

    pub fn get_controller_1(&mut self) -> &mut Controller {
        &mut self.controller_1
    }

    pub fn get_controller_2(&mut self) -> &mut Controller {
        &mut self.controller_2
    }

    pub fn read(&mut self, mapper: &mut dyn Mapper, address: u16) -> u8 {
        match address >> 13 {
            0 => self.ram[address as usize & 0x7FF],
            1 => self.ppu.read_register(PPURegister::from(address as u8 & 7), &mut PPUMemoryMap::new(mapper, &mut self.ppu_bus)),
            _ => if address == 0x4014 {
                self.ppu.read_register(PPURegister::DMA, &mut PPUMemoryMap::new(mapper, &mut self.ppu_bus))
            } else if address < 0x4020 {
                match address {
                    0x4016 => self.controller_1.read(),
                    0x4017 => self.controller_2.read(),
                    _ => 0, // TODO: APU
                }
            } else {
                unreachable!("Invalid CPU address map: {:04X}", address)
            }
        }
    }

    pub fn write(&mut self, mapper: &mut dyn Mapper, address: u16, value: u8) {
        match address >> 13 {
            0 => self.ram[address as usize & 0x7FF] = value,
            1 => self.ppu.write_register(PPURegister::from(address as u8 & 7), &mut PPUMemoryMap::new(mapper, &mut self.ppu_bus), value),
            _ => if address == 0x4014 {
                self.ppu.write_register(PPURegister::DMA, &mut PPUMemoryMap::new(mapper, &mut self.ppu_bus), value)
            } else if address < 0x4020 {
                match address {
                    0x4016 => {
                        self.controller_1.write(value);
                        self.controller_2.write(value)
                    },
                    _ => {}, // TODO: APU
                }
            } else {
                unreachable!("Invalid CPU address map: {:04X}", address)
            }
        }
    }
}