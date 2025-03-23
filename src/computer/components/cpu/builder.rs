use crate::computer::components::cpu::registers::builder::CPURegistersBuilderTrait;
use crate::computer::components::cpu::registers::flags::CPUFlagsAccessTrait;
use crate::computer::components::cpu::registers::{CPURegisters, CPURegistersAccessTrait};
use crate::computer::components::cpu::CPU;

#[derive(Debug, Default, PartialEq)]
pub struct CPUBuilder {
    registers: CPURegisters,
}

impl CPUBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CPU {
        let mut cpu = CPU::new();
        cpu.set_registers(self.registers);
        cpu
    }
}

impl CPURegistersAccessTrait for CPUBuilder {
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

impl CPUFlagsAccessTrait for CPUBuilder {
    fn get_flags(&self) -> u64 {
        self.registers.get_flags()
    }

    fn set_flags(&mut self, value: u64) {
        self.registers.set_flags(value);
    }
}

impl CPURegistersBuilderTrait for CPUBuilder {}
