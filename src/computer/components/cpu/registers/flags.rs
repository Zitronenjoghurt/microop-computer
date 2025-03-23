use crate::utils::bit_operations::{get_bit_u64, set_bit_u64};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct CPUFlags(u64);

impl CPUFlags {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CPUFlagsAccessTrait for CPUFlags {
    fn get_flags(&self) -> u64 {
        self.0
    }

    fn set_flags(&mut self, value: u64) {
        self.0 = value;
    }
}

pub trait CPUFlagsAccessTrait {
    fn get_flags(&self) -> u64;
    fn set_flags(&mut self, value: u64);

    fn get_zero(&self) -> bool {
        get_bit_u64(self.get_flags(), 0)
    }

    fn set_zero(&mut self, value: bool) {
        self.set_flags(set_bit_u64(self.get_flags(), 0, value))
    }

    fn get_carry(&self) -> bool {
        get_bit_u64(self.get_flags(), 1)
    }

    fn set_carry(&mut self, value: bool) {
        self.set_flags(set_bit_u64(self.get_flags(), 1, value))
    }

    fn get_subtract(&self) -> bool {
        get_bit_u64(self.get_flags(), 2)
    }

    fn set_subtract(&mut self, value: bool) {
        self.set_flags(set_bit_u64(self.get_flags(), 2, value))
    }
}
