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
}