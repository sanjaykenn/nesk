use crate::cpu::internal::CPUInternal;

mod alu;
mod instruction;
mod internal;
mod status;
mod cpu;
mod registers;
mod state;

pub struct CPU {
    internal: CPUInternal
}

pub trait CPUMemory {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
