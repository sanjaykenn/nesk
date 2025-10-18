use crate::controller::Controller;

impl Controller {
    pub fn new() -> Self {
        Self { value: 0, strobe: 0 }
    }
    
    pub fn load_buttons(&mut self, buttons: [bool; 8]) {
        if self.strobe & 1 != 0 {
            self.value = buttons.iter()
                .enumerate()
                .fold(0, |acc, (i, &pressed)| {
                    acc | ((pressed as u8) << i)
                });
        }
    }

    pub fn read(&mut self) -> u8 {
        let result = self.value & 1;
        if self.strobe & 1 == 0 {
            self.value = self.value >> 1 | 0x80;
        }
        result
    }

    pub fn write(&mut self, value: u8) {
        self.strobe = value;
    }
}