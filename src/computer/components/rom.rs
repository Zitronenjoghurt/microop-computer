use crate::computer::components::bus::status::BusStatus;
use crate::computer::components::bus::Bus;
use crate::utils::paged_memory::PagedMemory;

#[derive(Debug, Default, PartialEq)]
pub struct ROM {
    memory: PagedMemory,
}

impl ROM {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_bus(&mut self, bus: &mut Bus) {
        match bus.get_status() {
            BusStatus::Read => self.output_data(bus),
            _ => {}
        }
    }

    pub fn output_data(&mut self, bus: &mut Bus) {
        let data = self.memory.read_dw(bus.get_address().value());
        bus.force_put_data(data);
    }

    pub fn force_write(&mut self, data: Vec<u8>, address: u64) {
        data.iter().enumerate().for_each(|(i, byte)| {
            self.memory
                .write_byte(address.wrapping_add(i as u64), *byte);
        })
    }
}
