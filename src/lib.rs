mod cpu;
mod ppu;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;
pub const PIXEL_SIZE: usize = 3;

pub trait Screen {
    fn render(&mut self, pixels: &[[[u8; PIXEL_SIZE]; WIDTH]; HEIGHT]);
}
