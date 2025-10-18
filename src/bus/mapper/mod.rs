use crate::bus::cpu_bus::CPUBus;
use crate::bus::ppu_bus::PPUBus;

mod mapper_00;
mod utils;
mod mapper;

pub trait Mapper {
    fn cpu_read(&mut self, bus: &mut CPUBus, address: u16) -> u8;
    fn cpu_write(&mut self, bus: &mut CPUBus, address: u16, value: u8);
    fn ppu_read(&mut self, bus: &mut PPUBus, address: u16) -> u8;
    fn ppu_write(&mut self, bus: &mut PPUBus, address: u16, value: u8);
}

pub fn from_ines(binary: &[u8]) -> Box<dyn Mapper> {
    mapper::from_ines(binary).unwrap()
}
