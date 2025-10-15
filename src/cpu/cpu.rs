use crate::cpu::alu::{ALUOperation, ALU};
use crate::cpu::{CPUMemory, CPU};
use crate::cpu::instruction::{AddressingMode, IndexMode, Instruction, TargetRegister};
use crate::cpu::registers::Registers;
use crate::cpu::state::{CPUState, CycleMode};

impl CPU {
    pub fn new() -> Self {
        Self {
            state: CPUState::FetchInstruction,
            registers: Registers::new(),
            alu: ALU::new(),
            low: 0,
            high: 0,
            value: 0,
            fix_pch: false,
            branch: false,
            output: None,
        }
    }

    pub fn tick(&mut self, memory: &mut dyn CPUMemory) {
        let buffer = self.read(memory);

        if let Some(value) = self.alu.get_output(&mut self.registers.status) {
            match self.output.take() {
                Some(TargetRegister::SR) => self.set_register_value(TargetRegister::SR, value),
                Some(TargetRegister::SP) => self.set_register_value(TargetRegister::SP, value),
                None => {
                    self.registers.status.set_negative(value & 0b10000000 != 0);
                    self.registers.status.set_zero(value == 0);
                    self.value = value
                },
                Some(output) => {
                    self.registers.status.set_negative(value & 0b10000000 != 0);
                    self.registers.status.set_zero(value == 0);
                    self.set_register_value(output, value)
                }
            }
        }

        let state = self.next(buffer);

        self.write(memory);

        self.state = state;
    }

    fn read(&mut self, memory: &mut dyn CPUMemory) -> u8 {
        match &self.state.get_mode() {
            CycleMode::Fetch => memory.read(self.registers.program_counter),
            CycleMode::Read => memory.read(self.get_address()),
            CycleMode::Pop => self.pop_stack(memory),
            CycleMode::Peak => self.peak_stack(memory),
            _ => 0
        }
    }

    fn write(&mut self, memory: &mut dyn CPUMemory) {
        match self.state.get_mode() {
            CycleMode::Write => memory.write(self.get_address(), self.value),
            CycleMode::Push => self.push_to_stack(memory, self.value),
            _ => {}
        }
    }

    fn peak_stack(&self, memory: &mut dyn CPUMemory) -> u8 {
        memory.read(0x0100 | (self.registers.stack_pointer as u16))
    }

    fn pop_stack(&mut self, memory: &mut dyn CPUMemory) -> u8 {
        let result = self.peak_stack(memory);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        result
    }

    fn push_to_stack(&mut self, memory: &mut dyn CPUMemory, value: u8) {
        memory.write(0x0100 | (self.registers.stack_pointer as u16), value);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    fn next(&mut self, buffer: u8) -> CPUState {
        match self.state {
            CPUState::FetchInstruction => {
                if self.branch {
                    let pcl;
                    (pcl, self.fix_pch) = self.registers.get_pcl().overflowing_add(self.value);
                    self.fix_pch ^= self.value >= 0x80;
                    self.registers.set_pcl(pcl);
                    self.branch = false;
                    CPUState::FetchInstruction
                } else if self.fix_pch {
                    if self.registers.get_pcl() < 0x80 {
                        self.registers.set_pch(self.registers.get_pch().wrapping_add(1))
                    } else {
                        self.registers.set_pch(self.registers.get_pch().wrapping_sub(1))
                    }
                    self.fix_pch = false;
                    CPUState::FetchInstruction
                } else {
                    self.registers.increment_pc();
                    self.registers.instruction = Instruction::new(buffer);
                    CPUState::FetchOperand
                }
            }
            CPUState::FetchOperand => {
                if !matches!(self.registers.instruction.get_addressing_mode(), AddressingMode::Implied) {
                    self.registers.increment_pc();
                }

                match self.registers.instruction.get_addressing_mode() {
                    AddressingMode::Implied => self.implied_instructions(),
                    AddressingMode::Immediate => self.load_alu(self.get_register_value(self.registers.instruction.get_input()), buffer),
                    AddressingMode::Branch => {
                        self.value = buffer;
                        self.branch = self.registers.instruction.branch(&self.registers.status);
                        CPUState::FetchInstruction
                    }
                    addressing_mode => {
                        self.low = buffer;
                        self.high = 0;
                        match self.registers.instruction.get_opcode() {
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
                self.registers.set_pc(self.low, buffer);
                CPUState::FetchInstruction
            }
            CPUState::JumpIndirect(cycle) => match cycle {
                0 => {
                    self.high = buffer;
                    CPUState::JumpIndirect(1)
                }
                1 => {
                    self.value = buffer;
                    self.increase_low(1);
                    CPUState::JumpIndirect(2)
                }
                2 => {
                    self.registers.set_pc(self.value, buffer);
                    CPUState::FetchInstruction
                }
                _ => unreachable!("Invalid cycle for jump indirect"),
            }
            CPUState::IndexedRead(index) => {
                self.increase_low(self.get_index_value(index));
                self.read_or_write_state()
            }
            CPUState::FetchOperandHigh(index) => {
                self.registers.increment_pc();
                self.high = buffer;
                match index {
                    None => self.read_or_write_state(),
                    Some(index) => {
                        self.fix_pch = self.increase_low(self.get_index_value(index));
                        if self.registers.instruction.is_write() {
                            CPUState::DummyRead
                        } else {
                            CPUState::Read
                        }
                    }
                }
            }
            CPUState::Indirect(cycle, index) => match cycle {
                0 => {
                    self.low = self.low.wrapping_add(self.registers.x);
                    CPUState::Indirect(1, index)
                }
                1 => {
                    self.value = buffer;
                    self.increase_low(1);
                    CPUState::Indirect(2, index)
                }
                2 => {
                    self.high = buffer;
                    self.low = self.value;

                    match index {
                        IndexMode::X => self.read_or_write_state(),
                        IndexMode::Y => {
                            self.fix_pch = self.increase_low(self.registers.y);
                            if self.registers.instruction.is_write() {
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
                if self.fix_pch {
                    self.fix_pch = false;
                    self.high = self.high.wrapping_add(1);
                }
                self.read_or_write_state()
            }
            CPUState::Read => {
                if self.fix_pch {
                    self.fix_pch = false;
                    self.high = self.high.wrapping_add(1);
                    return CPUState::Read
                }

                self.load_alu(self.get_register_value(self.registers.instruction.get_input()), buffer);
                if self.registers.instruction.is_write() {
                    self.value = buffer;
                    self.output = None;
                    CPUState::DummyWrite
                } else {
                    CPUState::FetchInstruction
                }
            }
            CPUState::DummyWrite => CPUState::Write,
            CPUState::Write => {
                if !self.registers.instruction.is_read() {
                    self.value = self.get_register_value(self.registers.instruction.get_input());
                }
                CPUState::FetchInstruction
            }
            CPUState::Break(cycle) => match cycle {
                0 => { self.value = self.registers.get_pch(); CPUState::Break(1) },
                1 => { self.value = self.registers.get_pcl(); CPUState::Break(2) },
                2 => {
                    self.low = 0xFE;
                    self.high = 0xFF;
                    self.value = self.registers.status.get(); CPUState::Break(3)
                },
                3 => {
                    self.low = 0xFF;
                    self.high = 0xFF;
                    self.registers.set_pcl(buffer); CPUState::Break(4)
                },
                4 => {
                    self.registers.set_pch(buffer);
                    self.registers.status.set_interrupt(true);
                    CPUState::FetchInstruction
                },
                _ => unreachable!("Invalid cycle for break"),
            },
            CPUState::JumpSubroutine(cycle) => match cycle {
                0 => CPUState::JumpSubroutine(1),
                1 => { self.value = self.registers.get_pch(); CPUState::JumpSubroutine(2) },
                2 => { self.value = self.registers.get_pcl(); CPUState::JumpSubroutine(3) },
                3 => {
                    self.registers.set_pcl(self.low);
                    self.registers.set_pch(buffer);
                    CPUState::FetchInstruction
                },
                _ => unreachable!("Invalid cycle for jump subroutine"),
            },
            CPUState::ReturnInterrupt(cycle) => match cycle {
                0 => CPUState::ReturnInterrupt(1),
                1 => { self.registers.status.set(buffer); CPUState::ReturnInterrupt(2) },
                2 => { self.registers.set_pcl(buffer); CPUState::ReturnInterrupt(3) },
                3 => { self.registers.set_pch(buffer); CPUState::FetchInstruction },
                _ => unreachable!("Invalid cycle for return from interrupt"),
            }
            CPUState::ReturnSubroutine(cycle) => match cycle {
                0 => CPUState::ReturnSubroutine(1),
                1 => { self.registers.set_pcl(buffer); CPUState::ReturnSubroutine(2) },
                2 => { self.registers.set_pch(buffer); CPUState::ReturnSubroutine(3) },
                3 => { self.registers.increment_pc(); CPUState::FetchInstruction },
                _ => unreachable!("Invalid cycle for return from interrupt"),
            }
            CPUState::PushRegister(target) => { self.value = self.get_register_value(target); CPUState::FetchInstruction },
            CPUState::PullRegister(1, TargetRegister::SR) => { self.set_register_value(TargetRegister::SR, buffer); CPUState::FetchInstruction },
            CPUState::PullRegister(cycle, target) => match cycle {
                0 => CPUState::PullRegister(1, target),
                1 => self.load_alu_operation(0, buffer, ALUOperation::LOAD, target),
                _ => unreachable!("Invalid cycle for pull register"),
            },
        }
    }

    fn implied_instructions(&mut self) -> CPUState {
        match self.registers.instruction.get_opcode() {
            0x00 => { self.registers.increment_pc(); return CPUState::Break(0) },
            0x20 => return CPUState::JumpSubroutine(0),
            0x40 => return CPUState::ReturnInterrupt(0),
            0x60 => return CPUState::ReturnSubroutine(0),
            0x08 | 0x48 => return CPUState::PushRegister(self.registers.instruction.get_input()),
            0x18 => self.registers.status.set_carry(false),
            0x28 | 0x68 => return CPUState::PullRegister(0, self.registers.instruction.get_output()),
            0x38 => self.registers.status.set_carry(true),
            0x58 => self.registers.status.set_interrupt(false),
            0x78 => self.registers.status.set_interrupt(true),
            0x88 => return self.load_alu_operation(0, self.registers.y, ALUOperation::DEC, TargetRegister::Y),
            0x98 => return self.setup_transfer(TargetRegister::Y, TargetRegister::A),
            0xA8 => return self.setup_transfer(TargetRegister::A, TargetRegister::Y),
            0xB8 => self.registers.status.set_overflow(false),
            0xC8 => return self.load_alu_operation(0, self.registers.y, ALUOperation::INC, TargetRegister::Y),
            0xD8 => self.registers.status.set_decimal(false),
            0xE8 => return self.load_alu_operation(0, self.registers.x, ALUOperation::INC, TargetRegister::X),
            0xF8 => self.registers.status.set_decimal(true),
            0xEA => {},
            _ => return self.load_alu(0, self.get_register_value(self.registers.instruction.get_input())),
        }

        CPUState::FetchInstruction
    }

    fn increase_low(&mut self, value: u8) -> bool {
        let overflow;
        (self.low, overflow) = self.low.overflowing_add(value);
        overflow
    }

    fn get_index_value(&self, index: IndexMode) -> u8 {
        match index {
            IndexMode::X => self.registers.x,
            IndexMode::Y => self.registers.y,
        }
    }

    fn get_register_value(&self, target: TargetRegister) -> u8 {
        match target {
            TargetRegister::A => self.registers.a,
            TargetRegister::X => self.registers.x,
            TargetRegister::Y => self.registers.y,
            TargetRegister::SR => self.registers.status.get(),
            TargetRegister::SP => self.registers.stack_pointer,
        }
    }

    fn set_register_value(&mut self, target: TargetRegister, value: u8) {
        match target {
            TargetRegister::A => self.registers.a = value,
            TargetRegister::X => self.registers.x = value,
            TargetRegister::Y => self.registers.y = value,
            TargetRegister::SR => self.registers.status.set(value),
            TargetRegister::SP => self.registers.stack_pointer = value,
        }
    }

    fn load_alu(&mut self, a: u8, buffer: u8) -> CPUState {
        self.load_alu_operation(a, buffer, self.registers.instruction.get_alu_operation().unwrap(), self.registers.instruction.get_output())
    }

    fn setup_transfer(&mut self, input: TargetRegister, output: TargetRegister) -> CPUState {
        self.load_alu_operation(0, self.get_register_value(input), ALUOperation::LOAD, output)
    }

    fn load_alu_operation(&mut self, a: u8, buffer: u8, operation: ALUOperation, output: TargetRegister) -> CPUState {
        self.alu.set(a, buffer, operation);
        self.output = Some(output);
        CPUState::FetchInstruction
    }

    fn read_or_write_state(&mut self) -> CPUState {
        if self.registers.instruction.is_read() {
            CPUState::Read
        } else if self.registers.instruction.is_write() {
            CPUState::Write
        } else {
            unreachable!("Command must be either read or write")
        }
    }

    fn get_address(&self) -> u16 {
        self.low as u16 | (self.high as u16) << 8
    }
}
