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

pub enum ALUOperation {
}

pub fn is_read(opcode: u8) -> bool {
    opcode & 0b11100000 != 0b10000000
}

pub fn is_write(opcode: u8) -> bool {
    opcode & 0b11 == 0b11 || opcode & 0b110 == 0b110 || opcode & 0b11100000 == 0b10000000
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

pub fn get_addressing_mode(opcode: u8) -> AddressingMode {
    match opcode {
        0x96 | 0x97 | 0xB6 | 0xB7 => ZeroPage(Some(Y)),
        0x9E | 0x9F | 0xBE | 0xBF => Absolute(Some(Y)),
        _ => ADDRESS_TABLE[(opcode & 0x1F) as usize]
    }
}

pub fn get_alu_operation(opcode: u8) -> Option<ALUOperation> {
    todo!()
}
