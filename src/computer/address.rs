use std::fmt::Display;

// Inclusive ranges
pub const BOOT_ROM_START: u64 = 0x0000_0000_0000_0000;
pub const BOOT_ROM_END: u64 = 0x0000_0000_0000_1000;
pub const BOOT_ROM_SIZE: u64 = BOOT_ROM_END - BOOT_ROM_START;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Address(u64);

impl Address {
    pub fn new(address: u64) -> Address {
        Address(address)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

// SPECIFIC ADDRESSES
impl Address {
    pub fn boot_rom(offset: u64) -> Self {
        if offset > BOOT_ROM_SIZE {
            panic!("Invalid boot rom offset {offset}, boot rom only has a size of {BOOT_ROM_SIZE}")
        };
        Self::new(BOOT_ROM_START + offset)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}
