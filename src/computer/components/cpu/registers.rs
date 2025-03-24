use crate::computer::components::cpu::registers::flags::CPUFlagsAccessTrait;
use log::debug;
use reg::CPUReg;
use std::fmt::{Display, Formatter};

pub mod builder;
pub mod flags;
pub mod reg;

#[derive(Debug, PartialEq)]
pub struct CPURegisters {
    registers: [u64; 43],
}

impl CPURegisters {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CPURegistersAccessTrait for CPURegisters {
    fn get_registers(&self) -> &CPURegisters {
        self
    }

    fn get_registers_mut(&mut self) -> &mut CPURegisters {
        self
    }

    fn set_registers(&mut self, registers: CPURegisters) {
        self.registers = registers.registers;
    }
}

impl CPUFlagsAccessTrait for CPURegisters {
    fn get_flags(&self) -> u64 {
        self.get_register(CPUReg::F)
    }

    fn set_flags(&mut self, value: u64) {
        self.set_register(CPUReg::F, value);
    }
}

impl Default for CPURegisters {
    fn default() -> Self {
        Self { registers: [0; 43] }
    }
}

pub trait CPURegistersAccessTrait {
    fn get_registers(&self) -> &CPURegisters;
    fn get_registers_mut(&mut self) -> &mut CPURegisters;
    fn set_registers(&mut self, registers: CPURegisters);

    fn get_register(&self, reg: CPUReg) -> u64 {
        let index = reg as usize;
        self.get_registers().registers[index]
    }

    fn set_register(&mut self, reg: CPUReg, value: u64) {
        // Hardwired writing to IR => speed up fetch/decode cycle
        // On IR writes, the instruction will be analyzed for compressed format and PC will be incremented
        if reg == CPUReg::IR {
            // ToDo: Analyze instruction if its compressed format => set new flag
            self.set_register(CPUReg::PC, self.get_register(CPUReg::PC).wrapping_add(4));
            debug!(target: "cpu", "IR write detected, PC hardwired increment by 4");
        }

        let index = reg as usize;
        self.get_registers_mut().registers[index] = value;
    }
}

impl Display for CPURegisters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "RISC-V Registers:")?;
        for i in 0..32 {
            writeln!(f, "X{:<2}: {}", i, self.registers[i])?;
        }

        writeln!(f, "\nSpecial Registers:")?;
        writeln!(f, "PC  : {}", self.get_register(CPUReg::PC))?;
        writeln!(f, "IR  : {:032b}", self.get_register(CPUReg::IR) as u32)?;
        writeln!(f, "Z   : {}", if self.get_zero() { "✔" } else { "✘" })?;
        writeln!(f, "C   : {}", if self.get_carry() { "✔" } else { "✘" })?;
        writeln!(f, "S   : {}", if self.get_subtract() { "✔" } else { "✘" })?;
        for i in 0..8 {
            let index = 35 + i;
            writeln!(f, "TMP{}: {}", i, self.registers[index])?;
        }

        Ok(())
    }
}
