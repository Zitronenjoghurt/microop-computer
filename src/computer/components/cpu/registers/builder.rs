use crate::computer::components::cpu::registers::reg::CPUReg;
use crate::computer::components::cpu::registers::{CPURegisters, CPURegistersAccessTrait};

#[derive(Debug, Default, PartialEq)]
pub struct CPURegistersBuilder {
    registers: CPURegisters,
}

impl CPURegistersBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CPURegisters {
        self.registers
    }
}

impl CPURegistersAccessTrait for CPURegistersBuilder {
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

impl CPURegistersBuilderTrait for CPURegistersBuilder {}

pub trait CPURegistersBuilderTrait: CPURegistersAccessTrait + Sized {
    fn x0(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X0, value);
        self
    }

    fn x1(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X1, value);
        self
    }

    fn x2(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X2, value);
        self
    }

    fn x3(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X3, value);
        self
    }

    fn x4(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X4, value);
        self
    }

    fn x5(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X5, value);
        self
    }

    fn x6(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X6, value);
        self
    }

    fn x7(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X7, value);
        self
    }

    fn x8(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X8, value);
        self
    }

    fn x9(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X9, value);
        self
    }

    fn x10(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X10, value);
        self
    }

    fn x11(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X11, value);
        self
    }

    fn x12(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X12, value);
        self
    }

    fn x13(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X13, value);
        self
    }

    fn x14(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X14, value);
        self
    }

    fn x15(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X15, value);
        self
    }

    fn x16(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X16, value);
        self
    }

    fn x17(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X17, value);
        self
    }

    fn x18(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X18, value);
        self
    }

    fn x19(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X19, value);
        self
    }

    fn x20(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X20, value);
        self
    }

    fn x21(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X21, value);
        self
    }

    fn x22(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X22, value);
        self
    }

    fn x23(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X23, value);
        self
    }

    fn x24(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X24, value);
        self
    }

    fn x25(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X25, value);
        self
    }

    fn x26(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X26, value);
        self
    }

    fn x27(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X27, value);
        self
    }

    fn x28(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X28, value);
        self
    }

    fn x29(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X29, value);
        self
    }

    fn x30(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X30, value);
        self
    }

    fn x31(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::X31, value);
        self
    }

    fn pc(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::PC, value);
        self
    }

    fn ir(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::IR, value);
        self
    }

    fn tmp0(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::TMP0, value);
        self
    }

    fn tmp1(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::TMP1, value);
        self
    }

    fn tmp2(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::TMP2, value);
        self
    }

    fn tmp3(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::TMP3, value);
        self
    }

    fn tmp4(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::TMP4, value);
        self
    }

    fn tmp5(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::TMP5, value);
        self
    }

    fn tmp6(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::TMP6, value);
        self
    }

    fn tmp7(mut self, value: u64) -> Self {
        self.get_registers_mut().set_register(CPUReg::TMP7, value);
        self
    }
}
