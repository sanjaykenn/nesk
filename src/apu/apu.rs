use crate::apu::mixer::Mixer;
use crate::apu::noise_channel::NoiseChannel;
use crate::apu::pulse_channel::PulseChannel;
use crate::apu::registers::APURegister;
use crate::apu::triangle_channel::TriangleChannel;

const SAMPLE_NUMERATOR: u32 = 77;
const SAMPLE_DENOMINATOR: u32 = 3125;

struct FrameCounter {
    sequencer: u16,
    sequence_mode: bool,
    interrupt_inhibit: bool,
    next_sequence_mode: bool,
    next_interrupt_inhibit: bool,
    next_write: Option<u8>
}

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            sequencer: 0,
            sequence_mode: false,
            interrupt_inhibit: false,
            next_sequence_mode: false,
            next_interrupt_inhibit: false,
            next_write: None
        }
    }
}

pub struct APU {
    frame_counter: FrameCounter,
    pulse_channel1: PulseChannel<1>,
    pulse_channel2: PulseChannel<2>,
    triangle_channel: TriangleChannel,
    noise_channel: NoiseChannel,
    apu_mixer: Mixer,
    apu_tick: bool,
    irq: bool,
    ticks: u32,
    sample: Vec<f64>,
}

impl APU {
    pub fn new() -> Self {
        Self {
            frame_counter: FrameCounter::new(),
            pulse_channel1: PulseChannel::new(),
            pulse_channel2: PulseChannel::new(),
            triangle_channel: TriangleChannel::new(),
            noise_channel: NoiseChannel::new(),
            apu_mixer: Mixer::new(),
            apu_tick: false,
            irq: false,
            ticks: 0,
            sample: Vec::new()
        }
    }

    pub fn reset(&mut self) {
        self.frame_counter = FrameCounter::new();
        self.pulse_channel1 = PulseChannel::new();
        self.pulse_channel2 = PulseChannel::new();
        self.triangle_channel = TriangleChannel::new();
        self.noise_channel = NoiseChannel::new();
        self.apu_mixer = Mixer::new();
        self.apu_tick = false;
        self.irq = false;
        self.ticks = 0
    }

    fn send_irq(&mut self) {
        self.irq = true
    }

    pub fn reset_irq(&mut self) {
        self.irq = false
    }

    pub fn get_irq(&self) -> bool {
        self.irq
    }

    pub fn read_sound_channels_enable(&mut self) -> u8 {
        let mut data = 0x20;

        if self.pulse_channel1.get_length_counter().get() > 0 { data |= 0x01 }
        if self.pulse_channel2.get_length_counter().get() > 0 { data |= 0x02 }
        if self.triangle_channel.get_length_counter().get() > 0 { data |= 0x04 }
        if self.noise_channel.get_length_counter().get() > 0 { data |= 0x08 }
        // TODO: DMC Channel

        if self.get_irq() {
            data |= 0x40;
            self.reset_irq()
        }

        data
    }

    pub fn write(&mut self, register: APURegister, value: u8) {
        match register {
            APURegister::Pulse1(register) => self.pulse_channel1.write(register, value),
            APURegister::Pulse2(register) => self.pulse_channel2.write(register, value),
            APURegister::Triangle(register) => self.triangle_channel.write(register, value),
            APURegister::Noise(register) => self.noise_channel.write(register, value),
            APURegister::DMC(_) => {
                // TODO: DMC Channel
            },
            APURegister::SoundChannelsEnable => {
                // TODO: DMC Channel

                self.noise_channel.set_enabled(value & 0x08 != 0);
                self.triangle_channel.set_enabled(value & 0x04 != 0);
                self.pulse_channel2.set_enabled(value & 0x02 != 0);
                self.pulse_channel1.set_enabled(value & 0x01 != 0);

                if !self.pulse_channel1.is_enabled() { self.pulse_channel1.get_length_counter().clear() }
                if !self.pulse_channel2.is_enabled() { self.pulse_channel2.get_length_counter().clear() }
                if !self.triangle_channel.is_enabled() { self.triangle_channel.get_length_counter().clear() }
                if !self.noise_channel.is_enabled() { self.noise_channel.get_length_counter().clear() }
            },
            APURegister::FrameCounter => {
                self.frame_counter.next_write = Some(if self.apu_tick { 3 } else { 4 });
                self.frame_counter.next_sequence_mode = value & 0x80 != 0;
                self.frame_counter.next_interrupt_inhibit = value & 0x40 != 0;

                if self.frame_counter.next_interrupt_inhibit {
                    self.reset_irq()
                }

                if self.frame_counter.next_sequence_mode {
                    self.tick_quarter_frame();
                    self.tick_half_frame()
                }
            },
            APURegister::Unused => {}
        }
    }

    fn tick_quarter_frame(&mut self) {
        self.pulse_channel1.get_envelope().tick();
        self.pulse_channel2.get_envelope().tick();
        self.triangle_channel.get_linear_counter().tick();
        self.noise_channel.get_envelope().tick();
        // TODO: DMC Channel
    }

    fn tick_half_frame(&mut self) {
        self.pulse_channel1.get_length_counter().tick();
        self.pulse_channel2.get_length_counter().tick();
        self.triangle_channel.get_length_counter().tick();
        self.noise_channel.get_length_counter().tick();

        self.pulse_channel1.get_sweep().tick();
        self.pulse_channel2.get_sweep().tick();

        // TODO: DMC Channel
    }

    fn tick_frame_counter(&mut self) {
        if let Some(value) = &mut self.frame_counter.next_write {
            if *value > 0 {
                *value -= 1
            } else {
                self.frame_counter.sequencer = 0;
                self.frame_counter.sequence_mode = self.frame_counter.next_sequence_mode;
                self.frame_counter.interrupt_inhibit = self.frame_counter.next_interrupt_inhibit;
                self.frame_counter.next_write = None;
            }
        }

        if !self.apu_tick {
            match self.frame_counter.sequencer {
                3728 => self.tick_quarter_frame(),
                7456 => {
                    self.tick_quarter_frame();
                    self.tick_half_frame()
                },
                11185 => self.tick_quarter_frame(),
                14914 => if !self.frame_counter.sequence_mode {
                    self.tick_quarter_frame();
                    self.tick_half_frame();

                    if !self.frame_counter.interrupt_inhibit {
                        self.send_irq()
                    }
                },
                18640 => if self.frame_counter.sequence_mode {
                    self.tick_quarter_frame();
                    self.tick_half_frame()
                },
                _ => {}
            }

            self.frame_counter.sequencer += 1;

            if self.frame_counter.sequence_mode {
                if self.frame_counter.sequencer == 18641 {
                    self.frame_counter.sequencer = 0
                }
            } else if self.frame_counter.sequencer == 14915 {
                self.frame_counter.sequencer = 0
            }
        }
    }

    pub fn tick(&mut self) {
        if self.apu_tick {
            self.pulse_channel1.tick_timer();
            self.pulse_channel2.tick_timer();
            self.noise_channel.tick_timer();
            // TODO: DMC Channel
        }

        self.triangle_channel.tick_timer();

        self.tick_frame_counter();

        self.ticks += 1;
        if (self.ticks - 1) * SAMPLE_NUMERATOR / SAMPLE_DENOMINATOR < self.ticks * SAMPLE_NUMERATOR / SAMPLE_DENOMINATOR {
            self.sample.push(self.apu_mixer.get_output(
                self.pulse_channel1.get_output() as f64,
                self.pulse_channel2.get_output() as f64,
                self.triangle_channel.get_output() as f64,
                self.noise_channel.get_output() as f64,
                0.0 // TODO: DMC Channel
            ));
        }

        if self.ticks >= 3125 {
            self.ticks = 0
        }

        self.apu_tick = !self.apu_tick
    }

    pub fn get_output(&mut self) -> Vec<f64> {
        std::mem::replace(&mut self.sample, Vec::new())
    }
}
