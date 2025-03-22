use crate::computer::address::Address;
use crate::computer::components::bus::owner::BusOwner;
use crate::computer::components::bus::status::BusStatus;
use crate::computer::components::bus::Bus;
use crate::computer::components::cpu::decompose::decompose_instruction;
use crate::computer::components::cpu::micro_op::{MicroOp, MicroOpResponse};
use crate::computer::components::cpu::registers::CPUReg::IR;
use crate::computer::components::cpu::registers::{CPUReg, CPURegisters, CPURegistersAccessTrait};
use std::collections::VecDeque;

mod decompose;
mod micro_op;
pub mod registers;

#[derive(Debug, Default, PartialEq)]
pub struct CPU {
    registers: CPURegisters,
    micro_op_queue: VecDeque<MicroOp>,
}

impl CPU {
    pub fn new() -> CPU {
        CPU::default()
    }

    pub fn tick(&mut self, bus: &mut Bus) -> bool {
        if self.micro_op_queue.is_empty() {
            self.micro_op_queue = MicroOp::default_queue()
        }

        let micro_op = self.micro_op_queue.pop_front().unwrap();
        let response = match micro_op {
            MicroOp::Stall => MicroOpResponse::default(),
            MicroOp::Halt => MicroOpResponse::new_halt(),
            MicroOp::BusRelease => self.mo_bus_release(bus),
            MicroOp::BusTake => self.mo_bus_take(bus),
            MicroOp::BusReadByte(register) => self.mo_bus_read_byte(bus, register),
            MicroOp::BusReadHalfWord(register) => self.mo_bus_read_half_word(bus, register),
            MicroOp::BusReadWord(register) => self.mo_bus_read_word(bus, register),
            MicroOp::BusReadDoubleWord(register) => self.mo_bus_read_double_word(bus, register),
            MicroOp::BusWriteAddress(register) => self.mo_bus_write_address(bus, register),
            MicroOp::BusWriteData(register) => self.mo_bus_write_data(bus, register),
            MicroOp::BusSetRead => self.mo_bus_set_read(bus),
            MicroOp::BusSetWriteByte => self.mo_bus_set_write_byte(bus),
            MicroOp::BusSetWriteHalfWord => self.mo_bus_set_write_half_word(bus),
            MicroOp::BusSetWriteWord => self.mo_bus_set_write_word(bus),
            MicroOp::BusSetWriteDoubleWord => self.mo_bus_set_write_double_word(bus),
            MicroOp::Decode => self.mo_decode(),
            MicroOp::ALUAdd(rd, rs1, rs2) => self.mo_alu_add(rd, rs1, rs2),
            MicroOp::RegisterLoadImm(register, imm) => self.mo_register_load_imm(register, imm),
        };

        if response.repeat {
            self.micro_op_queue.push_front(micro_op);
        };

        !response.halt
    }
}

/// Micro operations
impl CPU {
    fn mo_bus_release(&mut self, bus: &mut Bus) -> MicroOpResponse {
        bus.release_ownership(BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_take(&mut self, bus: &mut Bus) -> MicroOpResponse {
        let success = bus.take_ownership(BusOwner::CPU);
        if success {
            MicroOpResponse::default()
        } else {
            MicroOpResponse::new_repeat()
        }
    }

    fn mo_bus_write_address(&mut self, bus: &mut Bus, register: CPUReg) -> MicroOpResponse {
        // Failed write operations will be ignored
        let address = Address::new(self.get_register(register));
        bus.put_address(address, BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_write_data(&mut self, bus: &mut Bus, register: CPUReg) -> MicroOpResponse {
        // Failed write operations will be ignored
        bus.put_data(self.get_register(register), BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_read_byte(&mut self, bus: &Bus, register: CPUReg) -> MicroOpResponse {
        let data = (bus.get_data() & 0xFF) as i8 as i64 as u64; // Sign extension
        self.set_register(register, data);
        MicroOpResponse::default()
    }

    fn mo_bus_read_half_word(&mut self, bus: &Bus, register: CPUReg) -> MicroOpResponse {
        let data = (bus.get_data() & 0xFFFF) as i16 as i64 as u64;
        self.set_register(register, data);
        MicroOpResponse::default()
    }

    fn mo_bus_read_word(&mut self, bus: &Bus, register: CPUReg) -> MicroOpResponse {
        let data = (bus.get_data() & 0xFFFF_FFFF) as i32 as i64 as u64;
        self.set_register(register, data);
        MicroOpResponse::default()
    }

    fn mo_bus_read_double_word(&mut self, bus: &Bus, register: CPUReg) -> MicroOpResponse {
        let data = bus.get_data();
        self.set_register(register, data);
        MicroOpResponse::default()
    }

    fn mo_bus_set_read(&mut self, bus: &mut Bus) -> MicroOpResponse {
        bus.put_status(BusStatus::Read, BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_set_write_byte(&mut self, bus: &mut Bus) -> MicroOpResponse {
        bus.put_status(BusStatus::WriteByte, BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_set_write_half_word(&mut self, bus: &mut Bus) -> MicroOpResponse {
        bus.put_status(BusStatus::WriteHalfWord, BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_set_write_word(&mut self, bus: &mut Bus) -> MicroOpResponse {
        bus.put_status(BusStatus::WriteWord, BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_set_write_double_word(&mut self, bus: &mut Bus) -> MicroOpResponse {
        bus.put_status(BusStatus::WriteDoubleWord, BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_decode(&mut self) -> MicroOpResponse {
        let instruction = self.get_register(IR) as u32;
        let instruction_queue = decompose_instruction(instruction);
        self.micro_op_queue = VecDeque::from(instruction_queue);
        MicroOpResponse::default()
    }

    fn mo_alu_add(&mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> MicroOpResponse {
        let value1 = self.get_register(rs1);
        let value2 = self.get_register(rs2);
        self.set_register(rd, value1.wrapping_add(value2));
        MicroOpResponse::default()
    }

    fn mo_register_load_imm(&mut self, register: CPUReg, imm: u64) -> MicroOpResponse {
        self.set_register(register, imm);
        MicroOpResponse::default()
    }
}

impl CPURegistersAccessTrait for CPU {
    fn get_registers(&self) -> &CPURegisters {
        &self.registers
    }

    fn get_registers_mut(&mut self) -> &mut CPURegisters {
        &mut self.registers
    }
}
