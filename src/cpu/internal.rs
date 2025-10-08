use crate::cpu::alu::ALU;
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
    ReadIndirect(i32),
    DummyRead,
    Read,
    DummyWrite,
    Write,
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
            CPUState::Write => 0,
            _ => self.read(self.get_pc()),
        };

        if let Some(output) = self.output.take() {
            match output {
                TargetRegister::A => self.registers.a = self.latch,
                TargetRegister::X => self.registers.x = self.latch,
                TargetRegister::Y => self.registers.y = self.latch,
            }
        }

        if let Some(value) = self.alu.get_result(&mut self.registers.sr) {
            self.result = value
        }

        self.state = self.next(buffer);

        if matches!(self.state, CPUState::Write) {
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
                    AddressingMode::Implied => CPUState::FetchInstruction,
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
                            _ => match addressing_mode {
                                AddressingMode::ZeroPage(index) => match index {
                                    None => self.read_or_write_state(),
                                    Some(index) => CPUState::IndexedRead(index)
                                }
                                AddressingMode::Absolute(index) => CPUState::FetchOperandHigh(index),
                                AddressingMode::Indirect(index) => match index {
                                    IndexMode::X => CPUState::ReadIndirect(0),
                                    IndexMode::Y => CPUState::ReadIndirect(1),
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
                match index {
                    None => self.read_or_write_state(),
                    Some(index) => {
                        (self.pcl, self.fix_pch) = self.pcl.overflowing_add(match index {
                            IndexMode::X => self.registers.x,
                            IndexMode::Y => self.registers.y,
                        });
                        CPUState::Read
                    }
                }
            }
            CPUState::ReadIndirect(cycle) => match cycle {
                0 => {
                    self.pcl = buffer.wrapping_add(self.registers.x);
                    CPUState::ReadIndirect(1)
                }
                1 => {
                    self.latch = buffer;
                    self.increment_pcl();
                    CPUState::ReadIndirect(2)
                }
                2 => {
                    self.pch = buffer;
                    self.pcl = self.latch;

                    match self.registers.ir.get_addressing_mode() {
                        AddressingMode::Indirect(mode) => match mode {
                            IndexMode::X => CPUState::Read,
                            IndexMode::Y => {
                                self.pcl = self.pcl.wrapping_add(self.registers.y);
                                CPUState::DummyRead
                            },
                        },
                        _ => unreachable!("Invalid addressing mode for given state"),
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
                    CPUState::DummyWrite
                } else {
                    CPUState::FetchInstruction
                }
            }
            CPUState::DummyWrite => CPUState::Write,
            CPUState::Write => {
                self.latch = self.result;
                CPUState::FetchInstruction
            }
        }
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
        }
    }

    fn load_alu(&mut self, a: u8, buffer: u8) -> CPUState {
        match self.registers.ir.get_alu_operation() {
            None => self.result = buffer,
            Some(operation) => self.alu.set(a, buffer, operation),
        }

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

    fn read(&mut self, addr: u16) -> u8 {
        0
    }

    fn write(&mut self, addr: u16, data: u8) {}
}
