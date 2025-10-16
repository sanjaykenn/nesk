use crate::ppu::registers::Registers;

mod utils;
mod registers;
mod background;

pub struct PPU {

}

pub trait PPUMemory {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn get_registers(&mut self) -> &mut Registers;

    fn read_nametable(&mut self, address: u16) -> u8 {
        self.read(0x2000 | address & 0x0FFF)
    }

    fn read_attribute_table(&mut self, nametable: u16, address: u16) -> u8 {
        self.read(0x23C0 | nametable << 10 | address)
    }

    fn read_pattern_table_tile_low(&mut self, pattern_table: bool, tile: u16, y: u16) -> u8 {
        self.read(if !pattern_table { 0 } else { 0x1000 } | tile << 4 | y)
    }

    fn read_pattern_table_tile_high(&mut self, pattern_table: bool, tile: u16, y: u16) -> u8 {
        self.read(if !pattern_table { 0 } else { 0x1000 } | tile << 4 | y | 8)
    }
}
