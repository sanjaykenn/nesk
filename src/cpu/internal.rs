use crate::cpu::alu::ALU;
use crate::cpu::instruction::{AddressingMode, IndexMode, Instruction};
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
}

enum CPUState {
    FetchInstruction,
    FetchOperand,
    JumpAbsolute,
    JumpIndirect(i32),
    FetchOperandHigh,
    ReadIndirect(i32),
    Read(bool),
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
}

impl CPUInternal {
    pub fn tick(&mut self) {
        let buffer = match self.state {
            CPUState::FetchInstruction | CPUState::FetchOperand | CPUState::FetchOperandHigh => {
                self.read(self.registers.pc)
            }
            CPUState::JumpIndirect(cycle) => match cycle {
                0 => self.read(self.get_pc()),
                _ => self.read(self.registers.pc),
            },
            CPUState::Write => 0,
            _ => self.read(self.get_pc()),
        };

        self.state = self.next(buffer);

        if matches!(self.state, CPUState::Write) {
            self.write(self.registers.pc, self.latch);
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
                } else if self.fix_pch {
                    self.registers
                        .set_pch(self.registers.get_pch().wrapping_add(1));
                    self.fix_pch = false;
                    CPUState::FetchInstruction
                } else {
                    self.registers.pc = self.registers.pc.wrapping_add(1);
                    self.registers.ir = Instruction::new(buffer);
                    CPUState::FetchOperand
                }
            }
            CPUState::FetchOperand => {
                if !matches!(
                    self.registers.ir.get_addressing_mode(),
                    AddressingMode::Implied
                ) {
                    self.registers.pc = self.registers.pc.wrapping_add(1);
                }

                match self.registers.ir.get_addressing_mode() {
                    AddressingMode::Implied => CPUState::FetchInstruction,
                    AddressingMode::Immediate => self.load_alu(todo!(), buffer),
                    AddressingMode::Branch => {
                        self.latch = buffer;
                        todo!(/* self.branch = ? */);
                        CPUState::FetchInstruction
                    }
                    addressing_mode => {
                        self.pcl = buffer;
                        self.pch = 0;
                        match self.registers.ir.get_opcode() {
                            0x4C => CPUState::JumpAbsolute,
                            0x6C => CPUState::JumpIndirect(0),
                            _ => todo!(),
                        }
                    }
                }
            }
            CPUState::JumpAbsolute => {
                self.registers.set_pcl(self.pcl);
                self.registers.set_pch(buffer);
                CPUState::FetchInstruction
            }
            CPUState::JumpIndirect(cycle) => match cycle {
                0 => {
                    self.registers.set_pch(buffer);
                    CPUState::JumpIndirect(1)
                }
                1 => {
                    self.latch = buffer;
                    self.pcl = self.pcl.wrapping_add(1);
                    CPUState::JumpIndirect(2)
                }
                2 => {
                    self.registers.set_pcl(self.latch);
                    self.registers.set_pch(buffer);
                    CPUState::FetchInstruction
                }
                _ => unreachable!("Invalid cycle for jump indirect"),
            },
            CPUState::FetchOperandHigh => {
                self.registers.pc = self.registers.pc.wrapping_add(1);
                match self.registers.ir.get_addressing_mode() {
                    AddressingMode::Absolute(mode) => match mode {
                        None => self.read_or_write_state(),
                        Some(index) => {
                            (self.pcl, self.fix_pch) = self.pcl.overflowing_add(match index {
                                IndexMode::X => self.registers.x,
                                IndexMode::Y => self.registers.y,
                            });
                            CPUState::Read(true)
                        }
                    },
                    _ => unreachable!("Invalid addressing mode for given state"),
                }
            }
            CPUState::ReadIndirect(cycle) => match cycle {
                0 => {
                    self.pcl = buffer.wrapping_add(self.registers.x);
                    CPUState::ReadIndirect(1)
                }
                1 => {
                    self.latch = buffer;
                    self.pcl = self.pcl.wrapping_add(1);

                    match self.registers.ir.get_addressing_mode() {
                        AddressingMode::Indirect(mode) => match mode {
                            IndexMode::X => CPUState::ReadIndirect(2),
                            IndexMode::Y => CPUState::ReadIndirect(3),
                        },
                        _ => unreachable!("Invalid addressing mode for given state"),
                    }
                }
                2 => {
                    self.pcl = self.latch;
                    self.pch = buffer;
                    CPUState::Read(false)
                },
                3 => {
                    self.pcl = self.latch + self.registers.y;
                    self.pch = buffer;
                    CPUState::Read(true)
                }
                _ => unreachable!("Invalid cycle for read indirect"),
            },
            CPUState::Read(reread) => {
                if self.fix_pch {
                    self.pch = self.pch.wrapping_add(1);
                    self.fix_pch = false;
                    CPUState::Read(false)
                } else if reread {
                    CPUState::Read(false)
                } else if self.registers.ir.is_write() {
                    //self.alu.a = todo!();
                    //self.alu.b = buffer;
                    CPUState::Write
                } else {
                    self.load_alu(todo!(), buffer)
                }
            }
            CPUState::Write => {
                todo!(/*self.latch = ...*/);

                if self.registers.ir.is_read() {
                    CPUState::Write
                } else {
                    CPUState::FetchInstruction
                }
            }
        }
    }

    fn load_alu(&mut self, a: u8, buffer: u8) -> CPUState {
        match self.registers.ir.get_alu_operation() {
            None => self.latch = buffer,
            Some(operation) => self.alu.set(a, buffer, operation),
        }

        CPUState::FetchInstruction
    }

    fn read_or_write_state(&mut self) -> CPUState {
        if self.registers.ir.is_read() {
            CPUState::Read(false)
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
