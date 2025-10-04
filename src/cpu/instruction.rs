pub enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

pub enum ALUOperation {
    // TODO
}

pub fn is_read(opcode: u8) -> bool {
    opcode & 0b11100000 != 0b10000000
}

pub fn is_write(opcode: u8) -> bool {
    opcode & 0b11 == 0b11 || opcode & 0b110 == 0b110 || opcode & 0b11100000 == 0b10000000
}

pub fn is_branch(opcode: u8) -> bool {
    opcode & 0b1111 == 0b0000
}

pub fn is_jmp(opcode: u8) -> bool {
    opcode == 0x4C || opcode == 0x6C
}

pub fn get_addressing_mode(opcode: u8) -> AddressingMode {
    match opcode & 0b00011101 {
        0b00000000 => if opcode & 0b10011111 != 0b10000010 { AddressingMode::Implied } else { AddressingMode::Immediate },
        0b00000001 => AddressingMode::IndirectX,
        0b00000100 => AddressingMode::ZeroPage,
        0b00000101 => AddressingMode::ZeroPage,
        0b00001001 => AddressingMode::Immediate,
        0b00001100 => AddressingMode::Absolute,
        0b00001101 => AddressingMode::Absolute,
        0b00010001 => AddressingMode::IndirectY,
        0b00010100 | 0b00010101 => if opcode & 0b11010010 != 0b10010010 { AddressingMode::ZeroPageX } else { AddressingMode::ZeroPageY },
        0b00011001 => AddressingMode::AbsoluteY,
        0b00011100 | 0b00011101 => if opcode & 0b11010010 != 0b10010010 { AddressingMode::AbsoluteX } else { AddressingMode::AbsoluteY },
        _ => AddressingMode::Implied
    }
}

pub fn get_alu_operation(opcode: u8) -> Option<ALUOperation> {
    None // TODO
}
