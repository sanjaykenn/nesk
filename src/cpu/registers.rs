use crate::cpu::instruction::Instruction;
use crate::cpu::status::StatusRegister;

pub struct Registers {
    pub instruction: Instruction,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: StatusRegister,
    pub stack_pointer: u8,
    pub program_counter: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            instruction: Instruction::new(0),
            a: 0,
            x: 0,
            y: 0,
            status: StatusRegister::new(),
            stack_pointer: 0xFD,
            program_counter: 0,
        }
    }

    pub fn get_pcl(&self) -> u8 {
        (self.program_counter & 0xFF) as u8
    }

    pub fn set_pcl(&mut self, pcl: u8) {
        self.program_counter = (self.program_counter & 0xFF00) | pcl as u16;
    }

    pub fn get_pch(&self) -> u8 {
        (self.program_counter >> 8) as u8
    }

    pub fn set_pch(&mut self, pch: u8) {
        self.program_counter = (self.program_counter & 0xFF) | (pch as u16) << 8;
    }

    pub fn set_pc(&mut self, pcl: u8, pch: u8) {
        self.program_counter = pcl as u16 | (pch as u16) << 8;
    }

    pub fn increment_pc(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }
}
