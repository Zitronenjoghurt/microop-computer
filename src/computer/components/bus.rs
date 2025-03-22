use crate::computer::address::{Address, BOOT_ROM_END, BOOT_ROM_START};
use crate::computer::components::bus::owner::BusOwner;
use crate::computer::components::bus::status::BusStatus;
use crate::computer::components::MMC;

pub mod owner;
pub mod status;

#[derive(Debug, Default, PartialEq)]
pub struct Bus {
    address: Address,
    data: u64,
    owner: BusOwner,
    status: BusStatus,
}

impl Bus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_available(&self) -> bool {
        self.owner == BusOwner::None && self.status == BusStatus::Idle
    }

    pub fn take_ownership(&mut self, source: BusOwner) -> bool {
        if !self.is_available() {
            return false;
        }

        self.owner = source;
        self.status = BusStatus::Idle;
        true
    }

    pub fn release_ownership(&mut self, source: BusOwner) -> bool {
        if source != self.owner {
            return false;
        }

        self.owner = BusOwner::None;
        self.status = BusStatus::Idle;
        true
    }

    pub fn put_address(&mut self, address: Address, source: BusOwner) -> bool {
        if source != self.owner {
            return false;
        }
        self.address = address;
        true
    }

    pub fn put_data(&mut self, data: u64, source: BusOwner) -> bool {
        if source != self.owner {
            return false;
        }
        self.data = data;
        true
    }

    pub fn put_status(&mut self, status: BusStatus, source: BusOwner) -> bool {
        if source != self.owner {
            return false;
        }
        self.status = status;
        true
    }

    pub fn get_address(&self) -> Address {
        self.address
    }

    pub fn get_data(&self) -> u64 {
        self.data
    }

    pub fn get_status(&self) -> BusStatus {
        self.status
    }

    pub fn get_active_mmc(&self) -> Option<MMC> {
        if self.status == BusStatus::Idle {
            return None;
        }

        match self.get_address().value() {
            BOOT_ROM_START..=BOOT_ROM_END => Some(MMC::ROM),
            _ => Some(MMC::RAM),
        }
    }

    pub fn force_put_data(&mut self, data: u64) {
        self.put_data(data, self.owner);
    }
}
