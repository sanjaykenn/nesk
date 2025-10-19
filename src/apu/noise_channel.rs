use crate::apu::components::{Envelope, LengthCounter, Timer};
use crate::apu::registers::NoiseRegister;

const NOISE_TIMER_PERIODS: [u16; 16] = [
    4,    8,   16,   32,   64,   96,  128,  160,
    202,  254,  380,  508,  762, 1016, 2034, 4068
];

struct LinearFeedbackShiftRegister {
    value: u16,
    mode: bool
}

impl LinearFeedbackShiftRegister {
    pub fn new() -> Self {
        Self {
            value: 1,
            mode: false
        }
    }

    pub fn set_mode(&mut self, value: bool) {
        self.mode = value
    }

    pub fn is_muting(&self) -> bool {
        self.value & 1 != 0
    }

    pub fn tick(&mut self) {
        let feedback = (self.value & 1 != 0) ^ (self.value & if self.mode { 0x20 } else { 0x02 } != 0);
        self.value >>= 1;

        if feedback {
            self.value |= 0x4000
        }
    }
}

pub struct NoiseChannel {
    enabled: bool,
    envelope: Envelope,
    timer: Timer,
    shift_register: LinearFeedbackShiftRegister,
    length_counter: LengthCounter
}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            envelope: Envelope::new(),
            timer: Timer::new(),
            shift_register: LinearFeedbackShiftRegister::new(),
            length_counter: LengthCounter::new()
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value
    }

    pub fn get_envelope(&mut self) -> &mut Envelope {
        &mut self.envelope
    }

    pub fn get_length_counter(&mut self) -> &mut LengthCounter {
        &mut self.length_counter
    }

    pub fn get_output(&self) -> u8 {
        if self.enabled && !self.shift_register.is_muting() && !self.length_counter.is_muting() {
            self.envelope.get_output()
        } else {
            0
        }
    }

    pub fn write(&mut self, register: NoiseRegister, value: u8) {
        match register {
            NoiseRegister::Volume => {
                self.length_counter.set_halt(value & 0x20 != 0);
                self.envelope.set_constant_volume(value & 0x10 != 0);
                self.envelope.get_divider().set_reload(value & 0x0F)
            }
            NoiseRegister::Low => {
                self.shift_register.set_mode(value & 0x80 != 0);
                self.timer.set_period(NOISE_TIMER_PERIODS[value as usize & 0x0F])
            }
            NoiseRegister::High => {
                if self.enabled {
                    self.length_counter.set(value >> 3)
                }

                self.envelope.set_start()
            }
        }
    }

    pub fn tick_timer(&mut self) {
        if self.timer.tick() {
            self.shift_register.tick()
        }
    }
}