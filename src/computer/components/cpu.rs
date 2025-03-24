use crate::computer::address::Address;
use crate::computer::components::bus::owner::BusOwner;
use crate::computer::components::bus::status::BusStatus;
use crate::computer::components::bus::Bus;
use crate::computer::components::cpu::builder::CPUBuilder;
use crate::computer::components::cpu::decompose::decompose_instruction;
use crate::computer::components::cpu::micro_op::{MicroOp, MicroOpResponse};
use crate::computer::components::cpu::registers::flags::CPUFlagsAccessTrait;
use crate::computer::components::cpu::registers::{CPURegisters, CPURegistersAccessTrait};
use crate::log_microop_debug;
use log::{debug, trace};
use registers::reg::CPUReg;
use registers::reg::CPUReg::IR;
use std::collections::VecDeque;

mod builder;
mod decompose;
mod micro_op;
pub mod registers;

#[derive(Debug, Default, PartialEq)]
pub struct CPU {
    registers: CPURegisters,
    micro_op_queue: VecDeque<MicroOp>,
    ticks: u64,
    decode_counter: u64,
}

impl CPU {
    pub fn new() -> CPU {
        CPU::default()
    }

    pub fn builder() -> CPUBuilder {
        CPUBuilder::new()
    }

    pub fn tick(&mut self, bus: &mut Bus) -> bool {
        trace!(target: "cpu", "Tick {}", self.ticks);

        if self.micro_op_queue.is_empty() {
            self.micro_op_queue = MicroOp::default_queue();
            debug!(target: "cpu", "New Fetch/Decode Cycle")
        }

        let micro_op = self.micro_op_queue.pop_front().unwrap();
        let response = match micro_op {
            MicroOp::Stall => self.mo_stall(),
            MicroOp::Halt => self.mo_halt(),
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
            MicroOp::ALUAnd(rd, rs1, rs2) => self.mo_alu_and(rd, rs1, rs2),
            MicroOp::ALUOr(rd, rs1, rs2) => self.mo_alu_or(rd, rs1, rs2),
            MicroOp::ALUSub(rd, rs1, rs2) => self.mo_alu_sub(rd, rs1, rs2),
            MicroOp::ALUXor(rd, rs1, rs2) => self.mo_alu_xor(rd, rs1, rs2),
            MicroOp::RegisterLoadImm(register, imm) => self.mo_register_load_imm(register, imm),
        };

        if response.repeat {
            self.micro_op_queue.push_front(micro_op);
        };

        self.ticks = self.ticks.wrapping_add(1);

        !response.halt
    }
}

/// Micro operations
impl CPU {
    fn mo_stall(&self) -> MicroOpResponse {
        log_microop_debug!("stall", "Stalling");
        MicroOpResponse::default()
    }

    fn mo_halt(&self) -> MicroOpResponse {
        log_microop_debug!("halt", "Halting");
        MicroOpResponse::new_halt()
    }

    fn mo_bus_release(&mut self, bus: &mut Bus) -> MicroOpResponse {
        let success = bus.release_ownership(BusOwner::CPU);
        log_microop_debug!("bus_release", "{}", if success { "✔" } else { "✘" });
        MicroOpResponse::default()
    }

    fn mo_bus_take(&mut self, bus: &mut Bus) -> MicroOpResponse {
        let success = bus.take_ownership(BusOwner::CPU);
        if success {
            log_microop_debug!("bus_take", "✔");
            MicroOpResponse::default()
        } else {
            log_microop_debug!("bus_take", "✘ (rescheduling)");
            MicroOpResponse::new_repeat()
        }
    }

    fn mo_bus_write_address(&mut self, bus: &mut Bus, register: CPUReg) -> MicroOpResponse {
        // Failed write operations will be ignored
        let address = Address::new(self.get_register(register));
        bus.put_address(address, BusOwner::CPU);
        log_microop_debug!("bus_write_address", "{register} → {address}");
        MicroOpResponse::default()
    }

    fn mo_bus_write_data(&mut self, bus: &mut Bus, register: CPUReg) -> MicroOpResponse {
        // Failed write operations will be ignored
        let value = self.get_register(register);
        let success = bus.put_data(value, BusOwner::CPU);
        log_microop_debug!("bus_write_data", "{register} ← {value}; Success: {success}");
        MicroOpResponse::default()
    }

    fn mo_bus_read_byte(&mut self, bus: &Bus, register: CPUReg) -> MicroOpResponse {
        let data = (bus.get_data() & 0xFF) as i8 as i64 as u64; // Sign extension
        self.set_register(register, data);
        log_microop_debug!("bus_read_byte", "{register} ← {data}");
        MicroOpResponse::default()
    }

    fn mo_bus_read_half_word(&mut self, bus: &Bus, register: CPUReg) -> MicroOpResponse {
        let data = (bus.get_data() & 0xFFFF) as i16 as i64 as u64;
        self.set_register(register, data);
        log_microop_debug!("bus_read_half_word", "{register} ← {data}");
        MicroOpResponse::default()
    }

    fn mo_bus_read_word(&mut self, bus: &Bus, register: CPUReg) -> MicroOpResponse {
        let data = (bus.get_data() & 0xFFFF_FFFF) as i32 as i64 as u64;
        self.set_register(register, data);
        log_microop_debug!("bus_read_word", "{register} ← {data}");
        MicroOpResponse::default()
    }

    fn mo_bus_read_double_word(&mut self, bus: &Bus, register: CPUReg) -> MicroOpResponse {
        let data = bus.get_data();
        self.set_register(register, data);
        log_microop_debug!("bus_read_word", "{register} ← {data}");
        MicroOpResponse::default()
    }

    fn mo_bus_set_read(&mut self, bus: &mut Bus) -> MicroOpResponse {
        let success = bus.put_status(BusStatus::Read, BusOwner::CPU);
        log_microop_debug!("bus_set_read", "{}", if success { "✔" } else { "✘" });
        MicroOpResponse::default()
    }

    fn mo_bus_set_write_byte(&mut self, bus: &mut Bus) -> MicroOpResponse {
        let success = bus.put_status(BusStatus::WriteByte, BusOwner::CPU);
        log_microop_debug!("bus_set_write_byte", "{}", if success { "✔" } else { "✘" });
        MicroOpResponse::default()
    }

    fn mo_bus_set_write_half_word(&mut self, bus: &mut Bus) -> MicroOpResponse {
        let success = bus.put_status(BusStatus::WriteHalfWord, BusOwner::CPU);
        log_microop_debug!("bus_set_write_hw", "{}", if success { "✔" } else { "✘" });
        MicroOpResponse::default()
    }

    fn mo_bus_set_write_word(&mut self, bus: &mut Bus) -> MicroOpResponse {
        let success = bus.put_status(BusStatus::WriteWord, BusOwner::CPU);
        log_microop_debug!("bus_set_write_w", "{}", if success { "✔" } else { "✘" });
        MicroOpResponse::default()
    }

    fn mo_bus_set_write_double_word(&mut self, bus: &mut Bus) -> MicroOpResponse {
        let success = bus.put_status(BusStatus::WriteDoubleWord, BusOwner::CPU);
        log_microop_debug!("bus_set_write_dw", "{}", if success { "✔" } else { "✘" });
        MicroOpResponse::default()
    }

    fn mo_decode(&mut self) -> MicroOpResponse {
        let instruction_bits = self.get_register(IR) as u32;
        let (instruction, queue) = decompose_instruction(instruction_bits);
        self.micro_op_queue = VecDeque::from(queue);
        self.decode_counter = self.decode_counter.wrapping_add(1);
        log_microop_debug!(
            "decode",
            "#{}: {:032b} | {instruction}",
            self.decode_counter,
            instruction_bits
        );
        MicroOpResponse::default()
    }

    fn mo_register_load_imm(&mut self, register: CPUReg, imm: u64) -> MicroOpResponse {
        self.set_register(register, imm);
        log_microop_debug!("register_load_imm", "{register} ← {imm}");
        MicroOpResponse::default()
    }
}

/// ALU OPERATIONS
impl CPU {
    fn mo_alu_add(&mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> MicroOpResponse {
        let value1 = self.get_register(rs1);
        let value2 = self.get_register(rs2);
        let (mut result, carry) = value1.overflowing_add(value2);
        if carry {
            result = u64::MAX;
        }
        self.set_register(rd, result);
        self.set_carry(carry);
        self.set_zero(result == 0);
        self.set_subtract(false);
        log_microop_debug!(
            "alu_add",
            "{rd}({result}) = {rs1}({value1}) + {rs2}({value2})"
        );
        MicroOpResponse::default()
    }

    fn mo_alu_and(&mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> MicroOpResponse {
        let value1 = self.get_register(rs1);
        let value2 = self.get_register(rs2);
        let result = value1 & value2;
        self.set_register(rd, result);
        self.set_carry(false);
        self.set_zero(result == 0);
        self.set_subtract(false);
        log_microop_debug!(
            "alu_and",
            "{rd}({result}) = {rs1}({value1}) & {rs2}({value2})"
        );
        MicroOpResponse::default()
    }

    fn mo_alu_or(&mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> MicroOpResponse {
        let value1 = self.get_register(rs1);
        let value2 = self.get_register(rs2);
        let result = value1 | value2;
        self.set_register(rd, result);
        self.set_carry(false);
        self.set_zero(result == 0);
        self.set_subtract(false);
        log_microop_debug!(
            "alu_or",
            "{rd}({result}) = {rs1}({value1}) | {rs2}({value2})"
        );
        MicroOpResponse::default()
    }

    fn mo_alu_sub(&mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> MicroOpResponse {
        let value1 = self.get_register(rs1);
        let value2 = self.get_register(rs2);
        let (mut result, carry) = value1.overflowing_sub(value2);
        if carry {
            result = 0;
        }
        self.set_register(rd, result);
        self.set_carry(carry);
        self.set_zero(result == 0);
        self.set_subtract(true);
        log_microop_debug!(
            "alu_sub",
            "{rd}({result}) = {rs1}({value1}) - {rs2}({value2})"
        );
        MicroOpResponse::default()
    }

    fn mo_alu_xor(&mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> MicroOpResponse {
        let value1 = self.get_register(rs1);
        let value2 = self.get_register(rs2);
        let result = value1 ^ value2;
        self.set_register(rd, result);
        self.set_carry(false);
        self.set_zero(result == 0);
        self.set_subtract(false);
        log_microop_debug!(
            "alu_xor",
            "{rd}({result}) = {rs1}({value1}) ^ {rs2}({value2})"
        );
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

    fn set_registers(&mut self, registers: CPURegisters) {
        self.registers.set_registers(registers);
    }
}

impl CPUFlagsAccessTrait for CPU {
    fn get_flags(&self) -> u64 {
        self.registers.get_flags()
    }

    fn set_flags(&mut self, value: u64) {
        self.registers.set_flags(value);
    }
}
