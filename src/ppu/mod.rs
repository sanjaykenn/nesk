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
}
