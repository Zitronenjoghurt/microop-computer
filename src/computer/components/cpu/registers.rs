#[derive(Debug, Default, PartialEq)]
pub struct CPURegisters {
    /// RISC-V integer registers
    x: [u64; 31],
    /// Micro-Op tmp registers (not part of RISC-V)
    tmp: [u64; 8],
}

impl CPURegistersAccessTrait for CPURegisters {
    fn get_registers(&self) -> &CPURegisters {
        self
    }

    fn get_registers_mut(&mut self) -> &mut CPURegisters {
        self
    }
}

pub trait CPURegistersAccessTrait {
    fn get_registers(&self) -> &CPURegisters;
    fn get_registers_mut(&mut self) -> &mut CPURegisters;

    fn get_register(&self, reg: CPUReg) -> u64 {
        let index = reg as usize;
        if index == 0 {
            0
        } else if index < 32 {
            self.get_x(index - 1)
        } else {
            self.get_tmp(index - 31)
        }
    }

    fn set_register(&mut self, reg: CPUReg, value: u64) {
        let index = reg as usize;
        if index == 0 {
            return;
        }

        if index < 32 {
            self.set_x(index - 1, value);
        } else {
            self.set_tmp(index - 31, value);
        }
    }

    fn get_x(&self, index: usize) -> u64 {
        self.get_registers().x[index]
    }

    fn set_x(&mut self, index: usize, value: u64) {
        self.get_registers_mut().x[index] = value;
    }

    fn get_tmp(&self, index: usize) -> u64 {
        self.get_registers().tmp[index]
    }

    fn set_tmp(&mut self, index: usize, value: u64) {
        self.get_registers_mut().tmp[index] = value;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CPUReg {
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
    TMP0 = 32,
    TMP1 = 33,
    TMP2 = 34,
    TMP3 = 35,
    TMP4 = 36,
    TMP5 = 37,
    TMP6 = 38,
    TMP7 = 39,
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
