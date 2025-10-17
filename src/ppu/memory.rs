use crate::ppu::PPUMemory;

pub fn read_nametable(memory: &mut dyn PPUMemory, address: u16) -> u8 {
    memory.read(0x2000 | address & 0x0FFF)
}

pub fn read_attribute_table(memory: &mut dyn PPUMemory, nametable: u16, address: u16) -> u8 {
    memory.read(0x23C0 | nametable << 10 | address)
}

pub fn read_pattern_table_tile_low(memory: &mut dyn PPUMemory, pattern_table: bool, tile: u16, y: u16) -> u8 {
    memory.read(if !pattern_table { 0 } else { 0x1000 } | tile << 4 | y)
}

pub fn read_pattern_table_tile_high(memory: &mut dyn PPUMemory, pattern_table: bool, tile: u16, y: u16) -> u8 {
    memory.read(if !pattern_table { 0 } else { 0x1000 } | tile << 4 | y | 8)
}