use crate::computer::address::BOOT_ROM_START;
use crate::computer::components::bus::Bus;
use crate::computer::components::cpu::CPU;
use crate::computer::components::ram::RAM;
use crate::computer::components::rom::ROM;
use crate::computer::components::MMC;

pub mod address;
pub mod components;
pub mod instructions;

#[derive(Debug, Default, PartialEq)]
pub struct Computer {
    pub bus: Bus,
    pub cpu: CPU,
    pub ram: RAM,
    pub rom: ROM,
}

impl Computer {
    pub fn new() -> Computer {
        Computer::default()
    }

    pub fn tick(&mut self) -> bool {
        let do_continue = self.cpu.tick(&mut self.bus);

        if let Some(mmc) = self.bus.get_active_mmc() {
            match mmc {
                MMC::RAM => self.ram.process_bus(&mut self.bus),
                MMC::ROM => self.rom.process_bus(&mut self.bus),
            }
        };

        do_continue
    }

    pub fn set_boot_rom(&mut self, data: Vec<u8>) {
        self.rom.force_write(data, BOOT_ROM_START)
    }
}
