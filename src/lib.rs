use crate::bus::Bus;
use crate::nes::DMA;

mod cpu;
mod ppu;
mod bus;
mod nes;
mod controller;
mod apu;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;
pub const PIXEL_SIZE: usize = 3;

pub const TICK_DURATION_NS: f64 = 1100000.0 / 23625.0;

pub struct NES {
    bus: Bus,
    dma: DMA,
    cycle: usize,
}