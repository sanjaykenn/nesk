pub struct StatusRegister(u8);

impl StatusRegister {
    pub fn new() -> Self {
        Self(0b00110110)
    }

    pub fn get(&self) -> u8 {
        self.0
    }

    pub fn get_b_clear(&self) -> u8 {
        self.0 & 0b11101111
    }

    pub fn set(&mut self, sr: u8) {
        self.0 = sr | 0b00110000;
    }

    pub fn get_negative(&self) -> bool {
        self.get() & 0b10000000 != 0
    }

    pub fn set_negative(&mut self, value: bool) {
        self.set(self.get() & 0b01111111 | (value as u8) << 7);
    }

    pub fn get_overflow(&self) -> bool {
        self.get() & 0b01000000 != 0
    }

    pub fn set_overflow(&mut self, value: bool) {
        self.set(self.get() & 0b10111111 | (value as u8) << 6);
    }

    pub fn get_decimal(&self) -> bool {
        self.get() & 0b00001000 != 0
    }

    pub fn set_decimal(&mut self, value: bool) {
        self.set(self.get() & 0b11110111 | (value as u8) << 3);
    }

    pub fn get_interrupt(&self) -> bool {
        self.get() & 0b00000100 != 0
    }

    pub fn set_interrupt(&mut self, value: bool) {
        self.set(self.get() & 0b11111011 | (value as u8) << 2);
    }

    pub fn get_zero(&self) -> bool {
        self.get() & 0b00000010 != 0
    }

    pub fn set_zero(&mut self, value: bool) {
        self.set(self.get() & 0b11111101 | (value as u8) << 1);
    }

    pub fn get_carry(&self) -> bool {
        self.get() & 0b00000001 != 0
    }

    pub fn set_carry(&mut self, value: bool) {
        self.set(self.get() & 0b11111110 | (value as u8));
    }
}
