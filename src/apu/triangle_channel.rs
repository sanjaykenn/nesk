use crate::apu::components::{LengthCounter, Timer};
use crate::apu::registers::TriangleRegister;

const SEQUENCE: [u8; 32] = [
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
];

pub struct LinearCounter {
    counter: u8,
    reload_value: u8,
    reload: bool,
    control: bool
}

impl LinearCounter {
    pub fn new() -> Self {
        Self {
            counter: 0,
            reload_value: 0,
            reload: false,
            control: false
        }
    }

    pub fn get(&self) -> u8 {
        self.counter
    }

    pub fn reload(&mut self) {
        self.counter = self.reload_value
    }

    pub fn set_reload_value(&mut self, value: u8) {
        self.reload_value = value
    }

    pub fn set_reload(&mut self) {
        self.reload = true
    }

    pub fn set_control(&mut self, value: bool) {
        self.control = value
    }

    pub fn tick(&mut self) {
        if self.reload {
            self.reload()
        } else if self.counter != 0 {
            self.counter -= 1
        }

        if !self.control {
            self.reload = false
        }
    }
}

struct TriangleSequencer {
    index: usize
}

impl TriangleSequencer {
    pub fn new() -> Self {
        Self {
            index: 0
        }
    }

    pub fn get_output(&self) -> u8 {
        SEQUENCE[self.index]
    }

    pub fn tick(&mut self) {
        self.index = (self.index + 1) & 31
    }
}

pub struct TriangleChannel {
    enabled: bool,
    timer: Timer,
    linear_counter: LinearCounter,
    length_counter: LengthCounter,
    sequencer: TriangleSequencer
}

impl TriangleChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            timer: Timer::new(),
            linear_counter: LinearCounter::new(),
            length_counter: LengthCounter::new(),
            sequencer: TriangleSequencer::new()
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value
    }

    pub fn get_linear_counter(&mut self) -> &mut LinearCounter {
        &mut self.linear_counter
    }

    pub fn get_length_counter(&mut self) -> &mut LengthCounter {
        &mut self.length_counter
    }

    pub fn get_output(&self) -> u8 {
        self.sequencer.get_output()
    }

    pub fn write(&mut self, register: TriangleRegister, value: u8) {
        match register {
            TriangleRegister::Linear => {
                self.linear_counter.set_control(value & 0x80 != 0);
                self.length_counter.set_halt(value & 0x80 != 0);
                self.linear_counter.set_reload_value(value & 0x7F)
            }
            TriangleRegister::Low => {
                self.timer.set_period(self.timer.get_period() & 0x700 | value as u16)
            }
            TriangleRegister::High => {
                if self.enabled {
                    self.length_counter.set(value >> 3)
                }

                self.timer.set_period(self.timer.get_period() & 0xFF | (value as u16 & 7) << 8);
                self.linear_counter.set_reload()
            }
        }
    }

    pub fn tick_timer(&mut self) {
        if self.timer.tick() && self.linear_counter.get() != 0 && self.length_counter.get() != 0 {
            self.sequencer.tick()
        }
    }
}