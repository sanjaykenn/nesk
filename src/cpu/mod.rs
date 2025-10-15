use crate::cpu::alu::ALU;
use crate::cpu::instruction::TargetRegister;
use crate::cpu::registers::Registers;
use crate::cpu::state::CPUState;

mod alu;
mod instruction;
mod status;
mod cpu;
mod registers;
mod state;

pub struct CPU {
    state: CPUState,
    registers: Registers,
    alu: ALU,
    pcl: u8,
    pch: u8,
    latch: u8,
    fix_pch: bool,
    branch: bool,
    output: Option<TargetRegister>,
}

pub trait CPUMemory {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
