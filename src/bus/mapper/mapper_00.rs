use crate::bus::mapper::{utils, Mapper};
use crate::bus::cpu_bus::CPUBus;
use crate::bus::ppu_bus::PPUBus;

pub struct Mapper00 {
    horizontal_mirror: bool,
    prg_rom: Box<[u8]>,
    chr_rom: Box<[u8]>,
    prg_ram: Box<[u8]>,
    prg_rom_mask: u16,
    prg_ram_mask: u16,
}

impl Mapper00 {
    pub fn new(horizontal_mirror: bool, prg_rom: Box<[u8]>, chr_rom: Box<[u8]>, prg_ram_size: usize) -> Result<Self, String> {
        let prg_rom_mask = match prg_rom.len() {
            0x4000 => 0x3FFF,
            0x8000 => 0x7FFF,
            _ => return Err("Invalid PRG ROM size".to_string()),
        };
        
        if chr_rom.len() != 0x2000 {
            return Err("Invalid CHR ROM size".to_string());
        }

        let prg_ram_mask = match prg_ram_size {
            0 => 0,
            0x800 => 0x7FF,
            0x1000 => 0xFFF,
            _ => return Err("Invalid PRG RAM size".to_string()),
        };

        Ok(Self {
            horizontal_mirror: horizontal_mirror,
            prg_rom,
            chr_rom,
            prg_ram: vec![0; prg_ram_size].into_boxed_slice(),
            prg_rom_mask,
            prg_ram_mask,
        })
    }
}

impl Mapper for Mapper00 {
    fn cpu_read(&mut self, bus: &mut CPUBus, address: u16) -> u8 {
        match address >> 14 {
            0 => bus.read(self, address),
            1 => if self.prg_ram.len() > 0 { self.prg_ram[(address & self.prg_ram_mask) as usize] } else { 0 }
            _ => self.prg_rom[(address & self.prg_rom_mask) as usize],
        }
    }

    fn cpu_write(&mut self, bus: &mut CPUBus, address: u16, value: u8) {
        match address >> 14 {
            0 => bus.write(self, address, value),
            1 => if self.prg_ram.len() > 0 { self.prg_ram[(address & self.prg_ram_mask) as usize] = value }
            _ => {},
        }
    }

    fn ppu_read(&mut self, bus: &mut PPUBus, address: u16) -> u8 {
        if address < 0x2000 {
            self.chr_rom[address as usize]
        } else {
            bus.read(utils::mirror_namespace(address, self.horizontal_mirror, !self.horizontal_mirror))
        }
    }

    fn ppu_write(&mut self, bus: &mut PPUBus, address: u16, value: u8) {
        if address >= 0x2000 {
            bus.write(utils::mirror_namespace(address, self.horizontal_mirror, !self.horizontal_mirror), value)
        }
    }
}
