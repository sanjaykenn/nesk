pub struct PPUBus {
    vram: [u8; 0x800],
}

impl PPUBus {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x800],
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        if address >> 11 == 0b100 {
            return self.vram[address as usize & 0x7FF];
        }

        unreachable!("Invalid PPU address map: {:04X}", address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address >> 11 == 0b100 {
            self.vram[address as usize & 0x7FF] = value;
        }

        unreachable!("Invalid PPU address map: {:04X}", address)
    }
}
