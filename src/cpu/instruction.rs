use crate::cpu::alu::ALUOperation;
use crate::cpu::instruction::AddressingMode::*;
use crate::cpu::instruction::IndexMode::*;

#[derive(Clone, Copy)]
pub enum IndexMode {
    X,
    Y
}

#[derive(Clone, Copy)]
pub enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage(Option<IndexMode>),
    Absolute(Option<IndexMode>),
    Indirect(IndexMode),
    Branch
}

const ADDRESS_TABLE: [AddressingMode; 0x20] = [
    Implied, Indirect(X), Immediate, Indirect(X),
    ZeroPage(None), ZeroPage(None), ZeroPage(None), ZeroPage(None),
    Implied, Immediate, Implied, Immediate,
    Absolute(None), Absolute(None), Absolute(None), Absolute(None), 
    Branch, Indirect(Y), Immediate, Indirect(Y),
    ZeroPage(Some(X)), ZeroPage(Some(X)), ZeroPage(Some(X)), ZeroPage(Some(X)),
    Implied, Absolute(Some(Y)), Implied, Absolute(Some(Y)),
    Absolute(Some(X)), Absolute(Some(X)), Absolute(Some(X)), Absolute(Some(X)),
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
            0x96 | 0x97 | 0xB6 | 0xB7 => ZeroPage(Some(Y)),
            0x9E | 0x9F | 0xBE | 0xBF => Absolute(Some(Y)),
            _ => ADDRESS_TABLE[(self.get_opcode() & 0x1F) as usize]
        }
    }

    pub fn get_alu_operation(&self) -> Option<ALUOperation> {
        todo!()
    }
}
