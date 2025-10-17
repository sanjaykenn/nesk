use crate::memory::memory::Memory;

mod mapper_00;

pub trait Mapper {
    fn read_cpu(&mut self, memory: &mut Memory, addr: u16) -> u8;
    fn write_cpu(&mut self, memory: &mut Memory, addr: u16, value: u8);
    fn read_ppu(&mut self, memory: &mut Memory, addr: u16) -> u8;
    fn write_ppu(&mut self, memory: &mut Memory, addr: u16, value: u8);
}