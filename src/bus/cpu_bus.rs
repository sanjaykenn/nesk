use crate::apu::{APURegister, APU};
use crate::bus::mapper::Mapper;
use crate::bus::ppu_bus::PPUBus;
use crate::bus::ppu_memory_map::PPUMemoryMap;
use crate::controller::Controller;
use crate::ppu::{PPURegister, PPU};

pub struct CPUBus<'a> {
    ppu: &'a mut PPU,
    apu: &'a mut APU,
    controller_1: &'a mut Controller,
    controller_2: &'a mut Controller,
    ram: &'a mut [u8; 0x800],
    vram: &'a mut [u8; 0x800],
    buffer: u8,
}

impl<'a> CPUBus<'a> {
    pub fn new(
        ppu: &'a mut PPU,
        apu: &'a mut APU,
        controller_1: &'a mut Controller,
        controller_2: &'a mut Controller,
        ram: &'a mut [u8; 0x800],
        vram: &'a mut [u8; 0x800]
    ) -> Self {
        Self {
            ppu,
            apu,
            controller_1,
            controller_2,
            ram,
            vram,
            buffer: 0
        }
    }

    pub fn read<'b>(&mut self, mapper: &'b mut dyn Mapper, address: u16) -> u8 {
        let buffer = match address >> 13 {
            0 => self.ram[address as usize & 0x7FF],
            1 => self.ppu.read_register(PPURegister::from(address as u8 & 7), &mut PPUMemoryMap::new(mapper, PPUBus::new(self.vram))),
            _ => if address == 0x4014 {
                self.ppu.read_register(PPURegister::DMA, &mut PPUMemoryMap::new(mapper, PPUBus::new(self.vram)))
            } else if address < 0x4020 {
                match address {
                    0x4015 => self.apu.read_sound_channels_enable(),
                    0x4016 => self.controller_1.read() | self.buffer & 0xE0,
                    0x4017 => self.controller_2.read() | self.buffer & 0xE0,
                    _ => self.buffer,
                }
            } else {
                unreachable!("Invalid CPU address map: {:04X}", address)
            }
        };
        self.buffer = buffer;
        buffer
    }

    pub fn write<'b>(&mut self, mapper: &'b mut dyn Mapper, address: u16, value: u8) {
        match address >> 13 {
            0 => self.ram[address as usize & 0x7FF] = value,
            1 => self.ppu.write_register(PPURegister::from(address as u8 & 7), &mut PPUMemoryMap::new(mapper, PPUBus::new(self.vram)), value),
            _ => if address == 0x4014 {
                self.ppu.write_register(PPURegister::DMA, &mut PPUMemoryMap::new(mapper, PPUBus::new(self.vram)), value)
            } else if address < 0x4020 {
                match address {
                    0x4016 => { self.controller_1.write(value);self.controller_2.write(value) },
                    _ => self.apu.write(APURegister::from_address(address), value)
                }
            } else {
                unreachable!("Invalid CPU address map: {:04X}", address)
            }
        }
    }
}