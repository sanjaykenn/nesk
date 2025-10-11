use crate::cpu::status::StatusRegister;

#[derive(Clone, Copy)]
pub enum ALUOperation {
    LOAD, OR, AND, EOR, ADC, SBC, CMP, ASL, ROL, LSR, ROR, INC, DEC, BIT
}

pub struct ALU {
    a: u8,
    b: u8,
    carry: bool,
    operator: Option<ALUOperation>,
    output: Option<u8>,
}

impl ALU {
    pub fn new() -> Self {
        Self { a: 0, b: 0, carry: false, operator: None, output: None }
    }

    pub fn set(&mut self, a: u8, b: u8, operator: ALUOperation) {
        self.a = a;
        self.b = b;
        self.operator = Some(operator);
    }

    pub fn get_output(&mut self, status: &mut StatusRegister) -> Option<u8> {
        match self.output.take() {
            None => match self.operator {
                None => None,
                Some(operation) => self.compute_operation(operation, status),
            },
            Some(value) =>  {
                let operator = self.operator.take().unwrap();
                match operator {
                    ALUOperation::ADC => status.set_overflow((self.a ^ value) & (self.b ^ value) & 0b10000000 != 0),
                    ALUOperation::SBC => status.set_overflow((self.a ^ value) & (!self.b ^ value) & 0b10000000 != 0),
                    _ => {}
                }

                status.set_negative(value & 0b10000000 != 0);
                status.set_zero(value == 0);
                status.set_carry(self.carry);

                if matches!(operator, ALUOperation::CMP) {
                    None
                } else {
                    Some(value)
                }
            }
        }
    }

    fn compute_operation(&mut self, operation: ALUOperation, status: &mut StatusRegister) -> Option<u8> {
        match operation {
            ALUOperation::LOAD => {
                status.set_negative(self.b & 0b10000000 != 0);
                status.set_zero(self.b == 0);
                self.operator = None;
                return Some(self.b);
            }
            ALUOperation::BIT => {
                status.set_negative(self.b & 0b10000000 != 0);
                status.set_overflow(self.b & 0b01000000 != 0);
                status.set_zero(self.a & self.b == 0);
                self.operator = None;
                return None;
            }
            _ => {}
        }

        let output;
        (output, self.carry) = match operation {
            ALUOperation::OR => (self.a | self.b, status.get_carry()),
            ALUOperation::AND => (self.a & self.b, status.get_carry()),
            ALUOperation::EOR => (self.a ^ self.b, status.get_carry()),
            ALUOperation::ADC => Self::adc(self.a, self.b, status),
            ALUOperation::SBC => Self::adc(self.a, !self.b, status),
            ALUOperation::CMP => (self.a.wrapping_sub(self.b), self.a >= self.b),
            ALUOperation::ASL => (self.b << 1, self.b & 0b10000000 != 0),
            ALUOperation::ROL => (self.b << 1 | status.get_carry() as u8, self.b & 0b10000000 != 0),
            ALUOperation::LSR => (self.b >> 1, self.b & 1 != 0),
            ALUOperation::ROR => (self.b >> 1 | (status.get_carry() as u8) << 7, self.b & 1 != 0),
            ALUOperation::INC => (self.b.wrapping_add(1), status.get_carry()),
            ALUOperation::DEC => (self.b.wrapping_sub(1), status.get_carry()),
            ALUOperation::LOAD | ALUOperation::BIT => unreachable!("Invalid operation"),
        };

        self.output = Some(output);

        None
    }

    fn adc(a: u8, b: u8, status: &mut StatusRegister) -> (u8, bool) {
        let (result1, carry1) = a.overflowing_add(b);
        let (result2, carry2) = result1.overflowing_add(status.get_carry() as u8);
        (result2, carry1 || carry2)
    }
}