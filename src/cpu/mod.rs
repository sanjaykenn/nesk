use crate::cpu::operations::OperationUnit;
use crate::cpu::instruction::TargetRegister;
use crate::cpu::registers::Registers;
use crate::cpu::state::CPUState;

mod operations;
mod instruction;
mod status;
mod cpu;
mod registers;
mod state;

pub struct CPU {
    state: CPUState,
    registers: Registers,
    operation_unit: OperationUnit,
    low: u8,
    high: u8,
    value: u8,
    fix_pch: bool,
    branch: bool,
    output: Option<TargetRegister>,
    nmi: bool,
}

pub trait CPUMemory {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
