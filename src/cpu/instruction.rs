use crate::cpu::alu::ALUOperation;
use crate::cpu::instruction::AddressingMode::*;
use crate::cpu::instruction::IndexMode::*;
use crate::cpu::status::StatusRegister;

pub enum TargetRegister {
    A,
    X,
    Y,
    SP
}

#[derive(Clone, Copy)]
pub enum IndexMode {
    X,
    Y
}

#[derive(Clone, Copy)]
pub enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageIndexed(IndexMode),
    Absolute,
    AbsoluteIndexed(IndexMode),
    Indirect(IndexMode),
    Branch
}

const ADDRESS_TABLE: [AddressingMode; 0x20] = [
    Implied, Indirect(X), Immediate, Indirect(X),
    ZeroPage, ZeroPage, ZeroPage, ZeroPage,
    Implied, Immediate, Implied, Immediate,
    Absolute, Absolute, Absolute, Absolute,
    Branch, Indirect(Y), Immediate, Indirect(Y),
    ZeroPageIndexed(X), ZeroPageIndexed(X), ZeroPageIndexed(X), ZeroPageIndexed(X),
    Implied, AbsoluteIndexed(Y), Implied, AbsoluteIndexed(Y),
    AbsoluteIndexed(X), AbsoluteIndexed(X), AbsoluteIndexed(X), AbsoluteIndexed(X),
];

const ALU_OPERATIONS: [Option<ALUOperation>; 0x20] = [
    None, Some(ALUOperation::BIT), None, None, None, None, Some(ALUOperation::CMP), Some(ALUOperation::SBC),
    Some(ALUOperation::OR), Some(ALUOperation::AND), Some(ALUOperation::EOR), Some(ALUOperation::ADC), None, None, Some(ALUOperation::CMP), Some(ALUOperation::SBC),
    Some(ALUOperation::ASL), Some(ALUOperation::ROL), Some(ALUOperation::LSR), Some(ALUOperation::ROR), None, None, Some(ALUOperation::DEC), Some(ALUOperation::INC),
    None, None, None, None, None, None, None, None,
];

pub struct Instruction(u8);

impl Instruction {
    pub fn new(opcode: u8) -> Self {
        Self(opcode)
    }
    
    pub fn get_opcode(&self) -> u8 {
        self.0
    }

    pub fn is_read(&self) -> bool {
        self.get_opcode() & 0b11100000 != 0b10000000
    }

    pub fn is_write(&self) -> bool {
        self.get_opcode() & 0b11 == 0b11 || self.get_opcode() & 0b110 == 0b110 || self.get_opcode() & 0b11100000 == 0b10000000
    }

    pub fn get_addressing_mode(&self) -> AddressingMode {
        match self.get_opcode() {
            0x96 | 0x97 | 0xB6 | 0xB7 => ZeroPageIndexed(Y),
            0x9E | 0x9F | 0xBE | 0xBF => AbsoluteIndexed(Y),
            _ => ADDRESS_TABLE[(self.get_opcode() & 0x1F) as usize]
        }
    }

    pub fn get_alu_operation(&self) -> Option<ALUOperation> {
        let index = (self.get_opcode() >> 1) & 0xFA | self.get_opcode() & 0b11;
        ALU_OPERATIONS[index as usize]
    }

    pub fn get_input(&self) -> TargetRegister {
        match self.get_opcode() {
            0xEB | 0xE0 | 0xE4 | 0xEC | 0x86 | 0x96 | 0x8E | 0x8A | 0x9A | 0xCA => TargetRegister::X,
            0xE8 | 0xC0 | 0xC4 | 0xCC | 0x84 | 0x94 | 0x8C => TargetRegister::Y,
            0xBA => TargetRegister::SP,
            _ => TargetRegister::A,
        }
    }

    pub fn get_output(&self) -> TargetRegister {
        match self.get_opcode() {
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE | 0xAA | 0xBA | 0xCA => TargetRegister::X,
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => TargetRegister::Y,
            0x9A => TargetRegister::SP,
            _ => TargetRegister::A,
        }
    }

    pub fn branch(&self, status: &StatusRegister) -> bool {
        match self.get_opcode() {
            0x10 => !status.get_negative(),
            0x30 => status.get_negative(),
            0x50 => !status.get_overflow(),
            0x70 => status.get_overflow(),
            0x90 => !status.get_carry(),
            0xB0 => status.get_carry(),
            0xD0 => !status.get_zero(),
            0xF0 => status.get_zero(),
            _ => unreachable!("Opcode is not a branch instruction"),
        }
    }
}
