use crate::computer::components::bus::status::BusStatus;
use crate::computer::components::bus::Bus;
use crate::utils::paged_memory::PagedMemory;
use log::debug;

#[derive(Debug, Default, PartialEq)]
pub struct RAM {
    memory: PagedMemory,
}

/// Bus Operations
impl RAM {
    pub fn process_bus(&mut self, bus: &mut Bus) {
        debug!(target: "ram", "RAM active");
        match bus.get_status() {
            BusStatus::Read => self.output_data(bus),
            BusStatus::WriteByte => self.input_data_byte(bus),
            BusStatus::WriteHalfWord => self.input_data_hw(bus),
            BusStatus::WriteWord => self.input_data_w(bus),
            BusStatus::WriteDoubleWord => self.input_data_dw(bus),
            _ => {}
        }
    }

    pub fn input_data_byte(&mut self, bus: &mut Bus) {
        let address = bus.get_address().value();
        let value = bus.get_data() as u8;
        debug!(target: "ram", "[{:016x}] Input Byte {value} ", address);
        self.memory.write_byte(address, value);
    }

    pub fn input_data_hw(&mut self, bus: &mut Bus) {
        let address = bus.get_address().value();
        let value = bus.get_data() as u16;
        debug!(target: "ram", "[{:016x}] Input HW: {value}", address);
        self.memory
            .write_hw(bus.get_address().value(), bus.get_data() as u16);
    }

    pub fn input_data_w(&mut self, bus: &mut Bus) {
        let address = bus.get_address().value();
        let value = bus.get_data() as u32;
        debug!(target: "ram", "[{:016x}] Input W: {value}", address);
        self.memory
            .write_w(bus.get_address().value(), bus.get_data() as u32);
    }

    pub fn input_data_dw(&mut self, bus: &Bus) {
        let address = bus.get_address().value();
        let value = bus.get_data();
        debug!(target: "ram", "[{:016x}] Input DW: {value}", address);
        self.memory
            .write_dw(bus.get_address().value(), bus.get_data());
    }

    pub fn output_data(&mut self, bus: &mut Bus) {
        let address = bus.get_address().value();
        let data = self.memory.read_dw(address);
        debug!(target: "ram", "[{:016x}] Output: {data}", address);
        bus.force_put_data(data);
    }
}
