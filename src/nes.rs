use crate::bus::Bus;
use crate::{HEIGHT, NES, PIXEL_SIZE, WIDTH};

#[derive(Clone, Copy)]
pub enum DMAState {
    INACTIVE,
    WAIT,
    TRANSFER
}

#[derive(Clone, Copy)]
pub struct DMA {
    state: DMAState,
    value: u8,
    address: u16
}

impl DMA {
    pub fn new() -> Self {
        Self {
            state: DMAState::INACTIVE,
            value: 0,
            address: 0
        }
    }
}

impl NES {
    pub fn from_ines(binary: &[u8]) -> Result<Self, String> {
        Ok(Self {
            bus: Bus::from_ines(binary)?,
            dma: DMA::new(),
            cycle: 0,
        })
    }

    pub fn load_buttons(&mut self, controller1: [bool; 8], controller2: [bool; 8]) {
        self.bus.get_controller_1().load_buttons(controller1);
        self.bus.get_controller_2().load_buttons(controller2)
    }

    pub fn tick(&mut self) {
        if self.bus.get_ppu().pull_nmi() {
            self.bus.get_cpu().send_nmi()
        }

        // TODO: if apu sends irq, send it to cpu

        if let Some(page) = self.bus.get_ppu().pull_dma() {
            self.dma.state = DMAState::WAIT;
            self.dma.address = (page as u16) << 8
        }

        match self.cycle {
            0 => match self.dma.state {
                DMAState::INACTIVE => {
                    self.bus.tick_cpu();
                    self.bus.tick_apu();
                },
                DMAState::WAIT => {},
                DMAState::TRANSFER => {
                    let value = self.bus.get_cpu_memory().read(self.dma.address);
                    self.dma.value = value;
                }
            },
            12 => match self.dma.state {
                DMAState::INACTIVE => {
                    self.bus.tick_cpu();
                    self.bus.tick_apu()
                },
                DMAState::WAIT => self.dma.state = DMAState::TRANSFER,
                DMAState::TRANSFER => {
                    let address = self.dma.address as u8;
                    let value = self.dma.value;
                    self.bus.get_ppu().write_oam(address, value);

                    if self.dma.address & 0xFF == 0xFF {
                        self.dma.state = DMAState::INACTIVE
                    } else {
                        self.dma.address += 1
                    }
                }
            },
            1 | 5 | 9 | 13 | 17 | 21 => self.bus.tick_ppu(),
            _ => {}
        }

        self.cycle = (self.cycle + 1) % 24;
    }
    
    pub fn get_screen_output(&mut self) -> Option<&[[[u8; PIXEL_SIZE]; WIDTH]; HEIGHT]> {
        self.bus.get_ppu().get_output()
    }
}