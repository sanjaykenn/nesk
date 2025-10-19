use crate::apu::components::{Divider, Envelope, LengthCounter, Timer};
use crate::apu::registers::PulseRegister;

const WAVEFORMS: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 1, 1],
    [0, 0, 0, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 0, 0]
];

pub struct Sweep<const CHANNEL: u8> {
    enabled: bool,
    divider: Divider,
    timer: Timer,
    negate: bool,
    shift: u8,
    reload: bool
}

impl<const CHANNEL: u8> Sweep<CHANNEL> {
    pub fn new() -> Self {
        Self {
            enabled: false,
            divider: Divider::new(),
            timer: Timer::new(),
            negate: false,
            shift: 0,
            reload: false
        }
    }

    fn get_target_period(&self) -> u16 {
        let change = self.timer.get_period() >> self.shift;

        if self.negate {
            self.timer.get_period().checked_sub(if CHANNEL == 1 { change + 1 } else { change })
                .unwrap_or_default()
        } else {
            self.timer.get_period() + change
        }
    }

    pub fn is_muting(&self) -> bool {
        self.timer.get_period() < 8 || self.get_target_period() > 0x7FF
    }

    pub fn tick(&mut self) {
        if self.divider.tick() && self.enabled && !self.is_muting() {
            self.timer.set_period(self.get_target_period())
        }

        if self.reload {
            self.divider.reload();
            self.reload = false
        }
    }
}

struct PulseSequencer {
    duty: usize,
    counter: usize
}

impl PulseSequencer {
    pub fn new() -> Self {
        Self {
            duty: 0,
            counter: 0
        }
    }

    pub fn tick(&mut self) {
        if self.counter == 0 {
            self.counter = 7
        } else {
            self.counter -= 1
        }
    }

    pub fn set_duty(&mut self, value: u8) {
        self.duty = value as usize
    }

    pub fn restart(&mut self) {
        self.counter = 0;
    }

    pub fn get_output(&self) -> u8 {
        WAVEFORMS[self.duty][self.counter]
    }
}

pub struct PulseChannel<const CHANNEL: u8> {
    enabled: bool,
    envelope: Envelope,
    sweep: Sweep<CHANNEL>,
    sequencer: PulseSequencer,
    length_counter: LengthCounter
}

impl<const CHANNEL: u8> PulseChannel<CHANNEL> {
    pub fn new() -> Self {
        Self {
            enabled: false,
            envelope: Envelope::new(),
            sweep: Sweep::new(),
            sequencer: PulseSequencer::new(),
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

    pub fn get_sweep(&mut self) -> &mut Sweep<CHANNEL> {
        &mut self.sweep
    }

    pub fn get_length_counter(&mut self) -> &mut LengthCounter {
        &mut self.length_counter
    }

    pub fn get_output(&self) -> u8 {
        if self.enabled && !self.sweep.is_muting() && !self.length_counter.is_muting() && self.sequencer.get_output() != 0 {
            self.envelope.get_output()
        } else {
            0
        }
    }

    pub fn write(&mut self, register: PulseRegister, value: u8) {
        match register {
            PulseRegister::Volume => {
                self.sequencer.set_duty(value >> 6);
                self.length_counter.set_halt(value & 0x20 != 0);
                self.envelope.set_loop(value & 0x20 != 0);
                self.envelope.set_constant_volume(value & 0x10 != 0);
                self.envelope.get_divider().set_reload(value & 0x0F)
            },
            PulseRegister::Sweep => {
                self.sweep.enabled = value & 0x80 != 0;
                self.sweep.divider.set_reload(value >> 4 & 7);
                self.sweep.negate = value & 8 != 0;
                self.sweep.shift = value & 7;
                self.sweep.reload = true;
            },
            PulseRegister::Low => {
                self.sweep.timer.set_period(self.sweep.timer.get_period() & 0x700 | value as u16)
            },
            PulseRegister::High => {
                if self.enabled {
                    self.length_counter.set(value >> 3)
                }

                self.sweep.timer.set_period(self.sweep.timer.get_period() & 0xFF | (value as u16 & 7) << 8);
                self.sequencer.counter = 0;
                self.sweep.reload = true;

                self.envelope.restart();
                self.sequencer.restart()
            }
        }
    }

    pub fn tick_timer(&mut self) {
        if self.sweep.timer.tick() {
            self.sequencer.tick()
        }
    }
}

