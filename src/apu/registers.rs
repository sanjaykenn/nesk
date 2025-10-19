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
