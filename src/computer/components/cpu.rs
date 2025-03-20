mod decode;
mod micro_op;
mod registers;

use crate::computer::components::bus::owner::BusOwner;
use crate::computer::components::bus::status::BusStatus;
use crate::computer::components::bus::Bus;
use crate::computer::components::cpu::decode::decode_instruction;
use crate::computer::components::cpu::micro_op::{MicroOp, MicroOpResponse};
use crate::computer::components::cpu::registers::{CPURegisters, CPURegistersAccessTrait};
use std::collections::VecDeque;

#[derive(Debug, Default, PartialEq)]
pub struct CPU {
    registers: CPURegisters,
    /// Program counter
    pc: u64,
    /// Instruction register
    ir: u32,
    micro_op_queue: VecDeque<MicroOp>,
}

impl CPU {
    pub fn new() -> CPU {
        CPU::default()
    }

    pub fn tick(&mut self, bus: &mut Bus) {
        if self.micro_op_queue.is_empty() {
            self.micro_op_queue = MicroOp::default_queue()
        }

        let micro_op = self.micro_op_queue.pop_front().unwrap();
        let response = match micro_op {
            MicroOp::Stall => MicroOpResponse::default(),
            MicroOp::BusWritePC => self.mo_bus_write_pc(bus),
            MicroOp::BusReadIR => self.mo_bus_read_ir(bus),
            MicroOp::BusRelease => self.mo_bus_release(bus),
            MicroOp::BusTake => self.mo_bus_take(bus),
            MicroOp::BusReadData(register) => self.mo_bus_read_data(bus, register),
            MicroOp::BusWriteAddress(register) => self.mo_bus_write_address(bus, register),
            MicroOp::BusWriteData(register) => self.mo_bus_write_data(bus, register),
            MicroOp::BusSetRead => self.mo_bus_set_read(bus),
            MicroOp::BusSetWrite => self.mo_bus_set_write(bus),
            MicroOp::Decode => self.mo_decode(),
            MicroOp::ALUAdd(rd, rs1, rs2) => todo!(),
        };

        if response.repeat {
            self.micro_op_queue.push_front(micro_op);
        }
    }
}

/// Micro operations
impl CPU {
    fn mo_bus_release(&mut self, bus: &mut Bus) -> MicroOpResponse {
        bus.release_ownership(BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_write_pc(&mut self, bus: &mut Bus) -> MicroOpResponse {
        bus.put_address(self.pc, BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_read_ir(&mut self, bus: &Bus) -> MicroOpResponse {
        let instruction = bus.get_data() as u32;
        self.ir = instruction;
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

    fn mo_bus_write_address(&mut self, bus: &mut Bus, register: u8) -> MicroOpResponse {
        // Failed write operations will be ignored
        bus.put_address(self.get_register(register), BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_write_data(&mut self, bus: &mut Bus, register: u8) -> MicroOpResponse {
        // Failed write operations will be ignored
        bus.put_data(self.get_register(register), BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_read_data(&mut self, bus: &Bus, register: u8) -> MicroOpResponse {
        let data = bus.get_data();
        self.set_register(register, data);
        MicroOpResponse::default()
    }

    fn mo_bus_set_read(&mut self, bus: &mut Bus) -> MicroOpResponse {
        bus.put_status(BusStatus::Read, BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_bus_set_write(&mut self, bus: &mut Bus) -> MicroOpResponse {
        bus.put_status(BusStatus::Write, BusOwner::CPU);
        MicroOpResponse::default()
    }

    fn mo_decode(&mut self) -> MicroOpResponse {
        let instruction_queue = decode_instruction(self.ir);
        self.micro_op_queue = VecDeque::from(instruction_queue);
        MicroOpResponse::default()
    }

    fn mo_alu_add(&mut self, rd: u8, rs1: u8, rs2: u8) -> MicroOpResponse {
        let value1 = self.get_register(rs1);
        let value2 = self.get_register(rs2);
        self.set_register(rd, value1.wrapping_add(value2));
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
