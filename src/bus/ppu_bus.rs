pub struct PPUBus<'a> {
    vram: &'a mut [u8; 0x800],
}

impl<'a> PPUBus<'a> {
    pub fn new(vram: &'a mut [u8; 0x800]) -> Self {
        Self {
            vram,
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        if 0x2000 <= address && address < 0x2800 {
            self.vram[address as usize & 0x7FF]
        } else {
            unreachable!("Invalid PPU address map: {:04X}", address)
        }

    }

    pub fn write(&mut self, address: u16, value: u8) {
        if 0x2000 <= address && address < 0x2800 {
            self.vram[address as usize & 0x7FF] = value;
        } else {
            unreachable!("Invalid PPU address map: {:04X}", address)
        }
    }
}
