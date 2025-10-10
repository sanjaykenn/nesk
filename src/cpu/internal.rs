use crate::cpu::alu::{ALUOperation, ALU};
use crate::cpu::instruction::{AddressingMode, IndexMode, Instruction, TargetRegister};
use crate::cpu::status::StatusRegister;

struct Registers {
    ir: Instruction,
    a: u8,
    x: u8,
    y: u8,
    sr: StatusRegister,
    sp: u8,
    pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ir: Instruction::new(0),
            a: 0,
            x: 0,
            y: 0,
            sr: StatusRegister::new(),
            sp: 0,
            pc: 0,
        }
    }

    pub fn get_pcl(&self) -> u8 {
        (self.pc & 0xFF) as u8
    }

    pub fn set_pcl(&mut self, pcl: u8) {
        self.pc = (self.pc & 0xFF00) | pcl as u16;
    }

    pub fn get_pch(&self) -> u8 {
        (self.pc >> 8) as u8
    }

    pub fn set_pch(&mut self, pch: u8) {
        self.pc = (self.pc & 0xFF) | (pch as u16) << 8;
    }

    pub fn set_pc(&mut self, pcl: u8, pch: u8) {
        self.pc = pcl as u16 | (pch as u16) << 8;
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
}

enum CPUState {
    FetchInstruction,
    FetchOperand,
    JumpAbsolute,
    JumpIndirect(i32),
    IndexedRead(IndexMode),
    FetchOperandHigh(Option<IndexMode>),
    Indirect(i32, IndexMode),
    DummyRead,
    Read,
    DummyWrite,
    Write,
    Break(i32),
    JumpSubroutine(i32),
    ReturnFromInterrupt(i32),
    ReturnSubroutine(i32),
    PushRegister(i32),
    PullRegister(i32),
}

struct CPUInternal {
    state: CPUState,
    registers: Registers,
    alu: ALU,
    pcl: u8,
    pch: u8,
    latch: u8,
    fix_pch: bool,
    branch: bool,
    output: Option<TargetRegister>,
    result: u8,
}

impl CPUInternal {
    pub fn tick(&mut self) {
        let buffer = match self.state {
            CPUState::FetchInstruction | CPUState::FetchOperand | CPUState::FetchOperandHigh(_) => self.read(self.registers.pc),
            CPUState::JumpIndirect(cycle) => match cycle {
                0 => self.read(self.get_pc()),
                _ => self.read(self.registers.pc),
            },
            CPUState::Write | CPUState::Break(0) | CPUState::Break(1) | CPUState::Break(2) => 0,
            CPUState::Break(3) => self.read(0xFFFE),
            CPUState::Break(4) =>  self.read(0xFFFF),
            _ => self.read(self.get_pc()),
        };

        if let Some(value) = self.alu.get_result(&mut self.registers.sr) {
            self.result = value
        } else if let Some(output) = self.output.take() {
            self.set_register_value(output);
        }

        self.state = self.next(buffer);

        if matches!(self.state, CPUState::Write | CPUState::DummyWrite | CPUState::Break(0) | CPUState::Break(1) | CPUState::Break(2)) {
            self.write(self.registers.pc, self.latch)
        }
    }

    fn next(&mut self, buffer: u8) -> CPUState {
        match self.state {
            CPUState::FetchInstruction => {
                if self.branch {
                    let pcl;
                    (pcl, self.fix_pch) = self.registers.get_pcl().overflowing_add(self.latch);
                    self.registers.set_pcl(pcl);
                    CPUState::FetchInstruction
                } else if self.fix_pch() {
                    CPUState::FetchInstruction
                } else {
                    self.registers.increment_pc();
                    self.registers.ir = Instruction::new(buffer);
                    CPUState::FetchOperand
                }
            }
            CPUState::FetchOperand => {
                if !matches!(self.registers.ir.get_addressing_mode(), AddressingMode::Implied) {
                    self.registers.increment_pc();
                }

                match self.registers.ir.get_addressing_mode() {
                    AddressingMode::Implied => self.implied_instructions(),
                    AddressingMode::Immediate => self.load_alu(self.get_register_value(self.registers.ir.get_input()), buffer),
                    AddressingMode::Branch => {
                        self.latch = buffer;
                        self.branch = self.registers.ir.branch(&self.registers.sr);
                        CPUState::FetchInstruction
                    }
                    addressing_mode => {
                        self.pcl = buffer;
                        self.pch = 0;
                        match self.registers.ir.get_opcode() {
                            0x4C => CPUState::JumpAbsolute,
                            0x6C => CPUState::JumpIndirect(0),
                            0x20 => CPUState::JumpSubroutine(0),
                            _ => match addressing_mode {
                                AddressingMode::ZeroPage => self.read_or_write_state(),
                                AddressingMode::ZeroPageIndexed(index) => CPUState::IndexedRead(index),
                                AddressingMode::Absolute => CPUState::FetchOperandHigh(None),
                                AddressingMode::AbsoluteIndexed(index) => CPUState::FetchOperandHigh(Some(index)),
                                AddressingMode::Indirect(index) => match index {
                                    IndexMode::X => CPUState::Indirect(0, IndexMode::X),
                                    IndexMode::Y => CPUState::Indirect(1, IndexMode::Y),
                                },
                                _ => unreachable!("Invalid addressing mode for given state"),
                            },
                        }
                    }
                }
            }
            CPUState::JumpAbsolute => {
                self.registers.set_pc(self.pcl, buffer);
                CPUState::FetchInstruction
            }
            CPUState::JumpIndirect(cycle) => match cycle {
                0 => {
                    self.registers.set_pch(buffer);
                    CPUState::JumpIndirect(1)
                }
                1 => {
                    self.latch = buffer;
                    self.increment_pcl();
                    CPUState::JumpIndirect(2)
                }
                2 => {
                    self.registers.set_pc(self.latch, buffer);
                    CPUState::FetchInstruction
                }
                _ => unreachable!("Invalid cycle for jump indirect"),
            }
            CPUState::IndexedRead(index) => {
                self.pcl += match index {
                    IndexMode::X => self.registers.x,
                    IndexMode::Y => self.registers.y,
                };
                self.read_or_write_state()
            }
            CPUState::FetchOperandHigh(index) => {
                self.registers.increment_pc();
                self.pch = buffer;
                match index {
                    None => self.read_or_write_state(),
                    Some(index) => {
                        (self.pcl, self.fix_pch) = self.pcl.overflowing_add(match index {
                            IndexMode::X => self.registers.x,
                            IndexMode::Y => self.registers.y,
                        });
                        if self.registers.ir.is_write() {
                            CPUState::DummyRead
                        } else {
                            CPUState::Read
                        }
                    }
                }
            }
            CPUState::Indirect(cycle, index) => match cycle {
                0 => {
                    self.pcl = buffer.wrapping_add(self.registers.x);
                    CPUState::Indirect(1, index)
                }
                1 => {
                    self.latch = buffer;
                    self.increment_pcl();
                    CPUState::Indirect(2, index)
                }
                2 => {
                    self.pch = buffer;
                    self.pcl = self.latch;

                    match index {
                        IndexMode::X => self.read_or_write_state(),
                        IndexMode::Y => {
                            self.pcl = self.pcl.wrapping_add(self.registers.y);
                            if self.registers.ir.is_write() {
                                CPUState::DummyRead
                            } else {
                               CPUState::Read
                            }
                        },
                    }
                }
                _ => unreachable!("Invalid cycle for read indirect"),
            }
            CPUState::DummyRead => {
                self.fix_pch();
                self.read_or_write_state()
            }
            CPUState::Read => {
                if self.fix_pch() {
                    return CPUState::Read
                }

                self.load_alu(self.get_register_value(self.registers.ir.get_input()), buffer);
                if self.registers.ir.is_write() {
                    self.latch = buffer;
                    self.output = None;
                    CPUState::DummyWrite
                } else {
                    CPUState::FetchInstruction
                }
            }
            CPUState::DummyWrite => CPUState::Write,
            CPUState::Write => {
                if self.registers.ir.is_read() {
                    self.latch = self.result;
                } else {
                    self.latch = self.get_register_value(self.registers.ir.get_input());
                }
                CPUState::FetchInstruction
            }
            CPUState::Break(cycle) => match cycle {
                0 => { self.push_to_stack(self.registers.get_pch()); CPUState::Break(1) },
                1 => { self.push_to_stack(self.registers.get_pcl()); CPUState::Break(2) },
                2 => { self.push_to_stack(self.registers.sr.get()); CPUState::Break(3) },
                3 => { self.registers.set_pcl(self.latch); CPUState::Break(4) },
                4 => {
                    self.registers.set_pch(self.latch);
                    self.registers.sr.set_interrupt(true);
                    CPUState::FetchInstruction
                },
                _ => unreachable!("Invalid cycle for break"),
            },
            CPUState::JumpSubroutine(cycle) => match cycle {
                0 => CPUState::JumpSubroutine(1),
                1 => { self.push_to_stack(self.registers.get_pch()); CPUState::JumpSubroutine(1) },
                2 => { self.push_to_stack(self.registers.get_pcl()); CPUState::JumpSubroutine(2) },
                3 => {
                    self.registers.set_pcl(self.latch);
                    self.registers.set_pcl(buffer);
                    CPUState::FetchInstruction
                },
                _ => unreachable!("Invalid cycle for jump subroutine"),
            },
            CPUState::ReturnFromInterrupt(_) => CPUState::FetchInstruction,
            CPUState::ReturnSubroutine(_) => CPUState::FetchInstruction,
            CPUState::PushRegister(_) => CPUState::FetchInstruction,
            CPUState::PullRegister(_) => CPUState::FetchInstruction,
        }
    }

    fn implied_instructions(&mut self) -> CPUState {
        match self.registers.ir.get_opcode() {
            0x00 => return CPUState::Break(0),
            0x20 => return CPUState::JumpSubroutine(0),
            0x40 => return CPUState::ReturnFromInterrupt(0),
            0x60 => return CPUState::ReturnSubroutine(0),
            0x08 => return CPUState::PushRegister(0),
            0x18 => self.registers.sr.set_carry(false),
            0x28 => return CPUState::PullRegister(0),
            0x38 => self.registers.sr.set_carry(true),
            0x48 => return CPUState::PushRegister(0),
            0x58 => self.registers.sr.set_interrupt(false),
            0x68 => return CPUState::PullRegister(0),
            0x78 => self.registers.sr.set_interrupt(true),
            0x88 => return self.load_alu_operation(0, self.registers.y, ALUOperation::DEC, TargetRegister::Y),
            0x98 => return self.setup_transfer(TargetRegister::Y, TargetRegister::A),
            0xA8 => return self.setup_transfer(TargetRegister::A, TargetRegister::Y),
            0xB8 => self.registers.sr.set_overflow(false),
            0xC8 => return self.load_alu_operation(0, self.registers.y, ALUOperation::INC, TargetRegister::Y),
            0xD8 => self.registers.sr.set_decimal(false),
            0xE8 => return self.load_alu_operation(0, self.registers.x, ALUOperation::INC, TargetRegister::X),
            0xF8 => self.registers.sr.set_decimal(true),
            0xEA => {},
            _ => return self.load_alu(0, self.get_register_value(self.registers.ir.get_input())),
        }

        CPUState::FetchInstruction
    }

    fn increment_pcl(&mut self) -> bool {
        let overflow;
        (self.pcl, overflow) = self.pcl.overflowing_add(1);
        overflow
    }

    fn fix_pch(&mut self) -> bool {
        if self.fix_pch {
            self.fix_pch = false;
            self.pch = self.pch.wrapping_add(1);
            return true
        }

        false
    }

    fn get_register_value(&self, target: TargetRegister) -> u8 {
        match target {
            TargetRegister::A => self.registers.a,
            TargetRegister::X => self.registers.x,
            TargetRegister::Y => self.registers.y,
            TargetRegister::SP => self.registers.sp,
        }
    }

    fn set_register_value(&mut self, target: TargetRegister) {
        match target {
            TargetRegister::A => self.registers.a = self.result,
            TargetRegister::X => self.registers.x = self.result,
            TargetRegister::Y => self.registers.y = self.result,
            TargetRegister::SP => self.registers.sp = self.result,
        }
    }

    fn load_alu(&mut self, a: u8, buffer: u8) -> CPUState {
        self.output = Some(self.registers.ir.get_output());
        match self.registers.ir.get_alu_operation() {
            None => self.result = buffer,
            Some(operation) => {
                if matches!(operation, ALUOperation::BIT | ALUOperation::CMP) {
                    self.output = None
                }

                self.alu.set(a, buffer, operation)
            },
        }

        CPUState::FetchInstruction
    }

    fn setup_transfer(&mut self, input: TargetRegister, output: TargetRegister) -> CPUState {
        self.result = self.get_register_value(input);
        self.output = Some(output);
        CPUState::FetchInstruction
    }

    fn load_alu_operation(&mut self, a: u8, buffer: u8, operation: ALUOperation, output: TargetRegister) -> CPUState {
        self.alu.set(a, buffer, operation);
        self.output = Some(output);
        CPUState::FetchInstruction
    }

    fn read_or_write_state(&mut self) -> CPUState {
        if self.registers.ir.is_read() {
            CPUState::Read
        } else if self.registers.ir.is_write() {
            CPUState::Write
        } else {
            unreachable!("Command must be either read or write")
        }
    }

    fn get_pc(&self) -> u16 {
        self.pcl as u16 | (self.pch as u16) << 8
    }

    fn push_to_stack(&mut self, value: u8) {
        self.pcl = self.registers.sp;
        self.pch = 0x01;
        self.latch = value;
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    fn peak_stack(&mut self) {
        self.pcl = self.registers.sp;
        self.pch = 0x01
    }

    fn read(&mut self, addr: u16) -> u8 {
        0
    }

    fn write(&mut self, addr: u16, data: u8) {}
}
