use crate::memory::mapper::Mapper;
use crate::memory::memory::Memory;

mod bus;
mod memory;
mod mapper;

pub struct Bus {
    memory: Memory,
    mapper: Box<dyn Mapper>,
}