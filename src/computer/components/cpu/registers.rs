use reg::CPUReg;
use std::fmt::{Display, Formatter};

pub mod builder;
pub mod reg;

#[derive(Debug, PartialEq)]
pub struct CPURegisters {
    registers: [u64; 42],
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

impl Default for CPURegisters {
    fn default() -> Self {
        Self { registers: [0; 42] }
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
        writeln!(f, "PC  : {}", self.registers[32])?;
        writeln!(f, "IR  : {:032b}", self.registers[33] as u32)?;
        for i in 0..8 {
            let index = 34 + i;
            writeln!(f, "TMP{}: {}", i, self.registers[index])?;
        }

        Ok(())
    }
}
