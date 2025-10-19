pub enum PulseRegister {
    Volume,
    Sweep,
    Low,
    High
}

pub enum TriangleRegister {
	Linear,
	Low,
	High
}

pub enum NoiseRegister {
    Volume,
    Low,
    High
}

pub enum DMCRegister {
    Frequency,
    Raw,
    Start,
    Length
}

pub enum APURegister {
    Pulse1(PulseRegister),
    Pulse2(PulseRegister),
    Triangle(TriangleRegister),
    Noise(NoiseRegister),
    DMC(DMCRegister),
    SoundChannelsEnable,
    FrameCounter,
    Unused
}

impl APURegister {
    pub fn from_address(address: u16) -> APURegister {
        match address {
            0x4000 => APURegister::Pulse1(PulseRegister::Volume),
            0x4001 => APURegister::Pulse1(PulseRegister::Sweep),
            0x4002 => APURegister::Pulse1(PulseRegister::Low),
            0x4003 => APURegister::Pulse1(PulseRegister::High),
            0x4004 => APURegister::Pulse2(PulseRegister::Volume),
            0x4005 => APURegister::Pulse2(PulseRegister::Sweep),
            0x4006 => APURegister::Pulse2(PulseRegister::Low),
            0x4007 => APURegister::Pulse2(PulseRegister::High),
            0x4008 => APURegister::Triangle(TriangleRegister::Linear),
            0x4009 => APURegister::Unused,
            0x400A => APURegister::Triangle(TriangleRegister::Low),
            0x400B => APURegister::Triangle(TriangleRegister::High),
            0x400C => APURegister::Noise(NoiseRegister::Volume),
            0x400D => APURegister::Unused,
            0x400E => APURegister::Noise(NoiseRegister::Low),
            0x400F => APURegister::Noise(NoiseRegister::High),
            0x4010 => APURegister::DMC(DMCRegister::Frequency),
            0x4011 => APURegister::DMC(DMCRegister::Raw),
            0x4012 => APURegister::DMC(DMCRegister::Start),
            0x4013 => APURegister::DMC(DMCRegister::Length),
            0x4015 => APURegister::SoundChannelsEnable,
            0x4017 => APURegister::FrameCounter,
            _ => APURegister::Unused
        }
    }
}
