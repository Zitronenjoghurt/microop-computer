use crate::computer::components::bus::Bus;
use crate::computer::components::cpu::CPU;
use crate::computer::components::ram::RAM;
use crate::computer::components::MMC;

pub mod components;
pub mod instructions;

#[derive(Debug, Default, PartialEq)]
pub struct Computer {
    bus: Bus,
    cpu: CPU,
    ram: RAM,
}

impl Computer {
    pub fn new() -> Computer {
        Computer::default()
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);

        if let Some(mmc) = self.bus.get_active_mmc() {
            match mmc {
                MMC::RAM => self.ram.process_bus(&mut self.bus),
                MMC::ROM => unimplemented!(),
            }
        }
    }
}
