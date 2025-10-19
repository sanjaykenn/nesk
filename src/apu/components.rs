const LENGTH_COUNTER_TABLE: [u8; 32] = [
	  9, 253,  19,   1,  39,   3,  79,   5,
	159,   7,  59,   9,  13,  11,  25,  13,
	 11,  15,  23,  17,  47,  19,  95,  21,
	191,  23,  71,  25,  15,  27,  31,  29
];

pub struct Divider {
	counter: u8,
	reload: u8
}

impl Divider {
	pub fn new() -> Self {
		Self {
			counter: 0,
			reload: 0
		}
	}

	pub fn get_reload(&self) -> u8 {
		self.reload
	}

	pub fn set_reload(&mut self, reload: u8) {
		self.reload = reload
	}

	pub fn reload(&mut self) {
		self.counter = self.reload
	}

	pub fn tick(&mut self) -> bool {
		if self.counter == 0 {
			self.reload();
			true
		} else {
			self.counter -= 1;
			false
		}
	}
}

pub struct Timer {
	period: u16,
	time: u16
}

impl Timer {
	pub fn new() -> Self {
		Self {
			period: 0,
			time: 0,
		}
	}

	pub fn get_period(&self) -> u16 {
		self.period
	}

	pub fn set_period(&mut self, value: u16) {
		self.period = value
	}

	pub fn reset(&mut self) {
		self.time = self.period
	}

	pub fn tick(&mut self) -> bool {
		if self.time == 0 {
			self.reset();
			true
		} else {
			self.time -= 1;
			false
		}
	}
}

pub struct Envelope {
	start: bool,
	loop_flag: bool,
	constant_volume: bool,
	divider: Divider,
	decay_level: u8
}

impl Envelope {
	pub fn new() -> Self {
		Self {
			start: false,
			loop_flag: false,
			constant_volume: false,
			divider: Divider::new(),
			decay_level: 0
		}
	}

	pub fn set_start(&mut self) {
		self.start = true
	}

	pub fn set_loop(&mut self, value: bool) {
		self.loop_flag = value
	}

	pub fn set_constant_volume(&mut self, value: bool) {
		self.constant_volume = value
	}

	pub fn get_divider(&mut self) -> &mut Divider {
		&mut self.divider
	}

	pub fn restart(&mut self) {
		self.start = false;
		self.decay_level = 15;
		self.divider.reload()
	}

	pub fn get_output(&self) -> u8 {
		if self.constant_volume {
			self.divider.get_reload()
		} else {
			self.decay_level
		}
	}

	pub fn tick(&mut self) {
		if !self.start {
			if self.divider.tick() {
				if self.decay_level > 0 {
					self.decay_level -= 1
				} else if self.loop_flag {
					self.decay_level = 15
				}
			}
		} else {
			self.restart()
		}
	}

}

pub struct LengthCounter {
	counter: u8,
	halt: bool
}

impl LengthCounter {
	pub fn new() -> Self {
		Self {
			counter: 0,
			halt: false
		}
	}

	pub fn get(&self) -> u8 {
		self.counter
	}

	pub fn set(&mut self, value: u8) {
		self.counter = LENGTH_COUNTER_TABLE[value as usize] + 1
	}

	pub fn clear(&mut self) {
		self.counter = 0
	}

	pub fn set_halt(&mut self, value: bool) {
		self.halt = value
	}

	pub fn is_muting(&self) -> bool {
		self.counter == 0
	}

	pub fn tick(&mut self) {
		if self.counter != 0 && !self.halt {
			self.counter -= 1
		}
	}
}
