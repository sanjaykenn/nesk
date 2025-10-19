const HIGH_PASS_K1: f64 = 0.996039;
const HIGH_PASS_K2: f64 = 0.999835;
const LOW_PASS_K1: f64 = 0.815686;

struct HighPassFilter {
    k: f64,
    previous_input: f64,
    previous_output: f64
}

impl HighPassFilter {
    pub fn new(k: f64) -> Self {
        Self {
            k,
            previous_input: 0.0,
            previous_output: 0.0,
        }
    }

    pub fn apply(&mut self, input: f64) -> f64 {
        self.previous_output = self.previous_output * self.k + input - self.previous_input;
        self.previous_input = input;
        self.previous_output
    }
}

struct LowPassFilter {
    k: f64,
    previous_input: f64,
    previous_output: f64
}

impl LowPassFilter {
    pub fn new(k: f64) -> Self {
        Self {
            k,
            previous_input: 0.0,
            previous_output: 0.0,
        }
    }

    pub fn apply(&mut self, input: f64) -> f64 {
        self.previous_output = (input - self.previous_output) * self.k;
        self.previous_input = input;
        self.previous_output
    }
}

struct Filter {
    high_pass1: HighPassFilter,
    high_pass2: HighPassFilter,
    low_pass1: LowPassFilter,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            high_pass1: HighPassFilter::new(HIGH_PASS_K1),
            high_pass2: HighPassFilter::new(HIGH_PASS_K2),
            low_pass1: LowPassFilter::new(LOW_PASS_K1),
        }
    }

    pub fn apply(&mut self, input: f64) -> f64 {
        let mut out = self.high_pass1.apply(input);
        out = self.high_pass2.apply(out);
        self.low_pass1.apply(out)
    }
}

pub struct Mixer {
    filter: Filter
}

impl Mixer {
    pub fn new() -> Self {
        Self { filter: Filter::new() }
    }

    pub fn get_output(&mut self, pulse1: f64, pulse2: f64, triangle: f64, noise: f64, dmc: f64) -> f64 {
        let pulse_out = 95.88 / (8128.0 / (pulse1 + pulse2) + 100.0);
        let tnd_out = 159.79 / (1.0 / (triangle / 8227.0 + noise / 12241.0 + dmc / 22638.0) + 100.0);

        self.filter.apply(pulse_out + tnd_out)
    }
}