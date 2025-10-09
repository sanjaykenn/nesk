use crate::cpu::status::StatusRegister;

#[derive(Clone, Copy)]
pub enum ALUOperation {
    OR, AND, EOR, ADC, SBC, CMP, ASL, ROL, LSR, ROR, INC, DEC, BIT
}

pub struct ALU {
    a: u8,
    b: u8,
    operator: Option<ALUOperation>
}

impl ALU {
    pub fn new() -> Self {
        Self { a: 0, b: 0, operator: None }
    }

    pub fn set(&mut self, a: u8, b: u8, operator: ALUOperation) {
        self.a = a;
        self.b = b;
        self.operator = Some(operator);
    }

    pub fn get_result(&mut self, status: &mut StatusRegister) -> Option<u8> {
        let result;
        match self.operator.take() {
            None => return None,
            Some(op) => match op {
                ALUOperation::OR => result = self.a | self.b,
                ALUOperation::AND => result = self.a & self.b,
                ALUOperation::EOR => result = self.a ^ self.b,
                ALUOperation::ADC => result = Self::adc(self.a, self.b, status),
                ALUOperation::SBC => result = Self::adc(self.a, !self.b, status),
                ALUOperation::CMP => {
                    status.set_negative(self.a.wrapping_sub(self.b) & 0b10000000 != 0);
                    status.set_zero(self.a == self.b);
                    status.set_carry(self.a >= self.b);
                    return None;
                },
                ALUOperation::ASL => {
                    result = self.b << 1;
                    status.set_carry(self.b & 0b10000000 != 0);
                },
                ALUOperation::ROL => {
                    result = self.b << 1 | status.get_carry() as u8;
                    status.set_carry(self.b & 0b10000000 != 0);
                },
                ALUOperation::LSR => {
                    result = self.b >> 1;
                    status.set_carry(self.b & 1 != 0);
                },
                ALUOperation::ROR => {
                    result = self.b >> 1 | (status.get_carry() as u8) << 7;
                    status.set_carry(self.b & 1 != 0);
                }
                ALUOperation::INC => result = self.a.wrapping_add(1),
                ALUOperation::DEC => result = self.a.wrapping_sub(1),
                ALUOperation::BIT => {
                    status.set_negative(self.b & 0b10000000 != 0);
                    status.set_overflow(self.b & 0b01000000 != 0);
                    status.set_zero(self.a & self.b == 0);
                    return None;
                }
            }
        }

        status.set_negative(result & 0b10000000 != 0);
        status.set_zero(result == 0);

        Some(result)
    }

    fn adc(a: u8, b: u8, status: &mut StatusRegister) -> u8 {
        let (result1, carry1) = a.overflowing_add(b);
        let (result2, carry2) = result1.overflowing_add(status.get_carry() as u8);
        let carry = carry1 || carry2;
        status.set_overflow(carry ^ status.get_carry());
        status.set_carry(carry);
        result2
    }
}