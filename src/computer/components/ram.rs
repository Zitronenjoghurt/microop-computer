use crate::computer::components::bus::status::BusStatus;
use crate::computer::components::bus::Bus;
use crate::utils::bit_operations::construct_u32;
use std::collections::HashMap;

const PAGE_SIZE: usize = 4096;
const PAGE_MASK: u64 = !(PAGE_SIZE as u64 - 1);

#[derive(Debug, Default, PartialEq)]
pub struct RAM {
    pages: HashMap<u64, [u8; PAGE_SIZE]>,
}

impl RAM {
    pub fn new() -> Self {
        Self::default()
    }

    fn page_address(address: u64) -> u64 {
        address & PAGE_MASK
    }

    fn page_offset(address: u64) -> usize {
        (address & !PAGE_MASK) as usize
    }

    pub fn read_byte(&self, address: u64) -> u8 {
        let address = Self::page_address(address);
        let offset = Self::page_offset(address);
        self.pages.get(&address).map_or(0, |p| p[offset])
    }

    pub fn write_byte(&mut self, address: u64, value: u8) {
        let address = Self::page_address(address);
        let offset = Self::page_offset(address);

        let page = self.pages.entry(address).or_insert_with(|| [0; PAGE_SIZE]);
        page[offset] = value;
    }

    pub fn read32(&self, address: u64) -> u32 {
        let b0 = self.read_byte(address);
        let b1 = self.read_byte(address.wrapping_add(1));
        let b2 = self.read_byte(address.wrapping_add(2));
        let b3 = self.read_byte(address.wrapping_add(3));
        construct_u32(b0, b1, b2, b3)
    }

    pub fn read64(&self, address: u64) -> u64 {
        let w0 = self.read_byte(address) as u64;
        let w1 = self.read_byte(address.wrapping_add(4)) as u64;

        w0 | (w1 << 32)
    }

    pub fn write32(&mut self, address: u64, value: u32) {
        let b0 = value as u8;
        let b1 = (value >> 8) as u8;
        let b2 = (value >> 16) as u8;
        let b3 = (value >> 24) as u8;

        self.write_byte(address, b0);
        self.write_byte(address.wrapping_add(1), b1);
        self.write_byte(address.wrapping_add(2), b2);
        self.write_byte(address.wrapping_add(3), b3);
    }

    pub fn write64(&mut self, address: u64, value: u64) {
        let w0 = value as u32;
        let w1 = (value >> 32) as u32;

        self.write32(address, w0);
        self.write32(address.wrapping_add(4), w1);
    }
}

/// Bus Operations
impl RAM {
    pub fn process_bus(&mut self, bus: &mut Bus) {
        match bus.get_status() {
            BusStatus::Read => self.output_data(bus),
            BusStatus::Write => self.input_data(bus),
            _ => {}
        }
    }

    fn input_data(&mut self, bus: &Bus) {
        self.write64(bus.get_address(), bus.get_data());
    }

    fn output_data(&mut self, bus: &mut Bus) {
        let data = self.read64(bus.get_address());
        bus.force_put_data(data);
    }
}
