use crate::ppu::PPUMemory;

pub struct Background {
    shifter_pattern_low: u16,
    shifter_pattern_high: u16,
    shifter_attribute_low: u16,
    shifter_attribute_high: u16,
    pattern_table_tile: u8,
    palette: u8,
    pattern_table_tile_low: u8,
    pattern_table_tile_high: u8,
}

impl Background {
    pub fn new() -> Self {
        Self {
            shifter_pattern_low: 0,
            shifter_pattern_high: 0,
            shifter_attribute_low: 0,
            shifter_attribute_high: 0,
            pattern_table_tile: 0,
            palette: 0,
            pattern_table_tile_low: 0,
            pattern_table_tile_high: 0,
        }
    }

    fn load_shift_registers(&mut self) {
        self.shifter_pattern_low = self.shifter_pattern_low & 0xFF00 | self.pattern_table_tile_low as u16;
        self.shifter_pattern_high = self.shifter_pattern_high & 0xFF00 | self.pattern_table_tile_high as u16;

        self.shifter_attribute_low = self.shifter_attribute_low & 0xFF00 | if self.palette & 1 != 0 { 0xFF } else { 0x00 };
        self.shifter_attribute_high = self.shifter_attribute_high & 0xFF00 | if self.palette & 2 != 0 { 0xFF } else { 0x00 };
    }

    fn shift_registers(&mut self, memory: &mut dyn PPUMemory) {
        if memory.get_registers().mask.get_show_background() {
            self.shifter_pattern_low <<= 1;
            self.shifter_pattern_high <<= 1;

            self.shifter_attribute_low <<= 1;
            self.shifter_attribute_high <<= 1;
        }
    }

    fn load_tile(&mut self, cycle: i32, memory: &mut dyn PPUMemory) {
        match cycle & 7 {
            1 => {
                if cycle >= 9 {
                    self.load_shift_registers();
                }

                let address = memory.get_registers().vram_address.get();
                self.pattern_table_tile = memory.read_nametable(address);
            }
            3 => {
                let nametable = memory.get_registers().vram_address.get_nametable();
                let attribute_index = memory.get_registers().vram_address.get_attribute_index();
                let attribute = memory.read_attribute_table(nametable, attribute_index);
                self.palette = memory.get_registers().get_palette_from_attribute(attribute)
            }
            5 => {
                let background_pattern_table = memory.get_registers().control.get_background_pattern_table();
                let fine_y = memory.get_registers().vram_address.get_fine_y();

                self.pattern_table_tile_low = memory.read_pattern_table_tile_low(
                    background_pattern_table,
                    self.pattern_table_tile as u16,
                    fine_y,
                )
            }
            7 => {
                let background_pattern_table = memory.get_registers().control.get_background_pattern_table();
                let fine_y = memory.get_registers().vram_address.get_fine_y();

                self.pattern_table_tile_high = memory.read_pattern_table_tile_high(
                    background_pattern_table,
                    self.pattern_table_tile as u16,
                    fine_y,
                )
            }
            _ => {}
        }
    }
}