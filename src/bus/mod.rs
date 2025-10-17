use crate::bus::mapper::Mapper;
use crate::bus::cpu_bus::CPUBus;
use crate::bus::ppu_bus::PPUBus;
use crate::cpu::CPU;

mod bus;
mod cpu_bus;
mod mapper;
mod ppu_bus;
mod cpu_memory_map;
mod ppu_memory_map;

pub struct Bus {
    cpu: CPU,
    cpu_memory: CPUMemoryMap,
}

struct CPUMemoryMap {
    mapper: Box<dyn Mapper>,
    bus: CPUBus
}

struct PPUMemoryMap<'a> {
    mapper: &'a mut dyn Mapper,
    bus: &'a mut PPUBus
}