use crate::apu::components::Timer;
use crate::apu::registers::DMCRegister;
use crate::cpu::CPUMemory;

const PITCH_TABLE: [u16; 16] = [
    214, 190, 170, 160, 143, 127, 113, 107,
    95,  80,  71,  64,  53,  42,  36,  27
];

struct Reader {
    address: u16,
    sample_buffer: Option<u8>,
    bytes_remaining: u16,
    sample_address: u16,
    sample_length: u16,
    loop_flag: bool,
    irq_enabled: bool,
    interrupt: bool,
    dma: bool
}

impl Reader {
    pub fn new() -> Self {
        Self {
            address: 0,
            sample_buffer: None,
            bytes_remaining: 0,
            sample_address: 0,
            sample_length: 0,
            loop_flag: false,
            irq_enabled: false,
            interrupt: false,
            dma: false
        }
    }

    pub fn are_bytes_remaining(&self) -> bool {
        self.bytes_remaining > 0
    }

    pub fn set_bytes_remaining(&mut self, bytes_remaining: u16) {
        self.bytes_remaining = bytes_remaining;

        if self.are_bytes_remaining() && self.buffer_is_empty() {
            self.fetch_sample()
        }
    }

    pub fn buffer_is_empty(&self) -> bool {
        self.sample_buffer.is_none()
    }

    pub fn empty_buffer(&mut self) -> u8 {
        if self.are_bytes_remaining() {
            self.fetch_sample()
        }

        self.sample_buffer.take().expect("Buffer is already empty")
    }

    pub fn load_buffer(&mut self, memory: &mut dyn CPUMemory) {
        self.sample_buffer = Some(memory.read(self.address));

        if self.address == 0xFFFF {
            self.address = 0x8000
        } else {
            self.address += 1
        }

        self.bytes_remaining -= 1;
        if !self.are_bytes_remaining() {
            if self.loop_flag {
                self.restart()
            } else if self.irq_enabled {
                self.interrupt = true
            }
        }
    }

    pub fn fetch_sample(&mut self) {
        self.dma = true
    }

    fn restart(&mut self) {
        self.address = self.sample_address;
        self.set_bytes_remaining(self.sample_length)
    }
}

pub struct DeltaModulationChannel {
    shift_register: u8,
    bits_remaining: u8,
    output_level: u8,
    silence: bool,
    timer: Timer,
    reader: Reader
}

impl DeltaModulationChannel {
    pub fn new() -> Self {
        Self {
            shift_register: 0,
            bits_remaining: 8,
            output_level: 0,
            silence: false,
            timer: Timer::new(),
            reader: Reader::new()
        }
    }

    pub fn write(&mut self, register: DMCRegister, value: u8) {
        match register {
            DMCRegister::Frequency => {
                self.reader.irq_enabled = value & 0x80 != 0;
                if !self.reader.irq_enabled {
                    self.reader.interrupt = false
                }

                self.reader.loop_flag = value & 0x40 != 0;
                self.timer.set_period(PITCH_TABLE[value as usize & 0x0F])
            }
            DMCRegister::Raw => self.output_level = value & 0x7F,
            DMCRegister::Start => self.reader.sample_address = 0xC000 | (value as u16) << 6,
            DMCRegister::Length => self.reader.sample_length = (value as u16) << 4 | 0x01
        }
    }

    pub fn are_bytes_remaining(&self) -> bool {
        self.reader.are_bytes_remaining()
    }

    pub fn set_bytes_remaining(&mut self, value: u16) {
        self.reader.set_bytes_remaining(value)
    }

    pub fn restart_sample(&mut self) {
        self.reader.restart()
    }

    pub fn get_irq(&self) -> bool {
        self.reader.interrupt
    }

    pub fn reset_irq(&mut self) {
        self.reader.interrupt = false
    }

    pub fn get_dma(&self) -> bool {
        self.reader.dma
    }

    pub fn reset_dma(&mut self) {
        self.reader.dma = false
    }

    pub fn load_buffer(&mut self, memory: &mut dyn CPUMemory) {
        self.reader.load_buffer(memory)
    }

    pub fn get_output(&self) -> u8 {
        self.output_level
    }

    pub fn tick_timer(&mut self) {
        if self.timer.tick() {
            if !self.silence {
                if self.shift_register & 1 != 0 {
                    if self.output_level <= 125 {
                        self.output_level += 2
                    }
                } else {
                    if self.output_level >= 2 {
                        self.output_level -= 2
                    }
                }
            }

            self.shift_register >>= 1;
            self.bits_remaining -= 1;

            if self.bits_remaining == 0 {
                self.bits_remaining = 8;
                self.silence = self.reader.buffer_is_empty();

                if !self.reader.buffer_is_empty() {
                    self.shift_register = self.reader.empty_buffer()
                }
            }
        }
    }
}
