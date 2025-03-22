use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CPUReg {
    // RISC-V integer registers
    X0 = 0,
    X1 = 1,
    X2 = 2,
    X3 = 3,
    X4 = 4,
    X5 = 5,
    X6 = 6,
    X7 = 7,
    X8 = 8,
    X9 = 9,
    X10 = 10,
    X11 = 11,
    X12 = 12,
    X13 = 13,
    X14 = 14,
    X15 = 15,
    X16 = 16,
    X17 = 17,
    X18 = 18,
    X19 = 19,
    X20 = 20,
    X21 = 21,
    X22 = 22,
    X23 = 23,
    X24 = 24,
    X25 = 25,
    X26 = 26,
    X27 = 27,
    X28 = 28,
    X29 = 29,
    X30 = 30,
    X31 = 31,
    // Micro-Op specific registers (not RISC-V)
    /// Program Counter
    PC = 32,
    /// Instruction Register
    IR = 33,
    TMP0 = 34,
    TMP1 = 35,
    TMP2 = 36,
    TMP3 = 37,
    TMP4 = 38,
    TMP5 = 39,
    TMP6 = 40,
    TMP7 = 41,
}

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
}

impl Default for CPURegisters {
    fn default() -> Self {
        Self { registers: [0; 42] }
    }
}

pub trait CPURegistersAccessTrait {
    fn get_registers(&self) -> &CPURegisters;
    fn get_registers_mut(&mut self) -> &mut CPURegisters;

    fn get_register(&self, reg: CPUReg) -> u64 {
        let index = reg as usize;
        self.get_registers().registers[index]
    }

    fn set_register(&mut self, reg: CPUReg, value: u64) {
        let index = reg as usize;
        self.get_registers_mut().registers[index] = value;
    }
}

impl CPUReg {
    pub fn to_riscv(&self) -> u8 {
        let reg = *self as u8;
        if reg >= 32 {
            panic!("Register index '{reg}' does not exist in RISC-V")
        };
        reg
    }
}

impl From<usize> for CPUReg {
    fn from(value: usize) -> Self {
        match value {
            0 => CPUReg::X0,
            1 => CPUReg::X1,
            2 => CPUReg::X2,
            3 => CPUReg::X3,
            4 => CPUReg::X4,
            5 => CPUReg::X5,
            6 => CPUReg::X6,
            7 => CPUReg::X7,
            8 => CPUReg::X8,
            9 => CPUReg::X9,
            10 => CPUReg::X10,
            11 => CPUReg::X11,
            12 => CPUReg::X12,
            13 => CPUReg::X13,
            14 => CPUReg::X14,
            15 => CPUReg::X15,
            16 => CPUReg::X16,
            17 => CPUReg::X17,
            18 => CPUReg::X18,
            19 => CPUReg::X19,
            20 => CPUReg::X20,
            21 => CPUReg::X21,
            22 => CPUReg::X22,
            23 => CPUReg::X23,
            24 => CPUReg::X24,
            25 => CPUReg::X25,
            26 => CPUReg::X26,
            27 => CPUReg::X27,
            28 => CPUReg::X28,
            29 => CPUReg::X29,
            30 => CPUReg::X30,
            31 => CPUReg::X31,
            32 => CPUReg::TMP0,
            33 => CPUReg::TMP1,
            34 => CPUReg::TMP2,
            35 => CPUReg::TMP3,
            36 => CPUReg::TMP4,
            37 => CPUReg::TMP5,
            38 => CPUReg::TMP6,
            39 => CPUReg::TMP7,
            _ => panic!("Invalid register index: {}", value),
        }
    }
}

impl From<u8> for CPUReg {
    fn from(value: u8) -> Self {
        CPUReg::from(value as usize)
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
