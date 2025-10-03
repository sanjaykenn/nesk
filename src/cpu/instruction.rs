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

pub struct Instruction {
    read: bool,
    write: bool,
    branch: bool,
    jmp: bool,
    addressing_mode : AddressingMode,
    alu_operation: Option<ALUOperation>,
}
