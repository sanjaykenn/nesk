mod alu;
mod instruction;
mod internal;
mod status;

pub struct CPU {}

pub trait CPUMemory {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

