use crate::computer::components::bus::Bus;
use crate::computer::components::cpu::CPU;

mod components;

#[derive(Debug, Default, PartialEq)]
pub struct Computer {
    bus: Bus,
    cpu: CPU,
}

impl Computer {
    pub fn new() -> Computer {
        Computer::default()
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
    }
}
