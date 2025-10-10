use crate::cpu::{CPUMemory, CPU};
use crate::cpu::internal::CPUInternal;

impl CPU {
    pub fn new() -> Self {
        Self {
            internal: CPUInternal::new()
        }
    }
    
    pub fn tick(&mut self, memory: &mut dyn CPUMemory) {
        self.internal.tick(memory);
    }
}