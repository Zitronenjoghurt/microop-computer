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
            32 => CPUReg::PC,
            33 => CPUReg::IR,
            34 => CPUReg::TMP0,
            35 => CPUReg::TMP1,
            36 => CPUReg::TMP2,
            37 => CPUReg::TMP3,
            38 => CPUReg::TMP4,
            39 => CPUReg::TMP5,
            40 => CPUReg::TMP6,
            41 => CPUReg::TMP7,
            _ => panic!("Invalid register index: {}", value),
        }
    }
}

impl From<u8> for CPUReg {
    fn from(value: u8) -> Self {
        CPUReg::from(value as usize)
    }
}

impl Display for CPUReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CPUReg::X0 => write!(f, "x0"),
            CPUReg::X1 => write!(f, "x1"),
            CPUReg::X2 => write!(f, "x2"),
            CPUReg::X3 => write!(f, "x3"),
            CPUReg::X4 => write!(f, "x4"),
            CPUReg::X5 => write!(f, "x5"),
            CPUReg::X6 => write!(f, "x6"),
            CPUReg::X7 => write!(f, "x7"),
            CPUReg::X8 => write!(f, "x8"),
            CPUReg::X9 => write!(f, "x9"),
            CPUReg::X10 => write!(f, "x10"),
            CPUReg::X11 => write!(f, "x11"),
            CPUReg::X12 => write!(f, "x12"),
            CPUReg::X13 => write!(f, "x13"),
            CPUReg::X14 => write!(f, "x14"),
            CPUReg::X15 => write!(f, "x15"),
            CPUReg::X16 => write!(f, "x16"),
            CPUReg::X17 => write!(f, "x17"),
            CPUReg::X18 => write!(f, "x18"),
            CPUReg::X19 => write!(f, "x19"),
            CPUReg::X20 => write!(f, "x20"),
            CPUReg::X21 => write!(f, "x21"),
            CPUReg::X22 => write!(f, "x22"),
            CPUReg::X23 => write!(f, "x23"),
            CPUReg::X24 => write!(f, "x24"),
            CPUReg::X25 => write!(f, "x25"),
            CPUReg::X26 => write!(f, "x26"),
            CPUReg::X27 => write!(f, "x27"),
            CPUReg::X28 => write!(f, "x28"),
            CPUReg::X29 => write!(f, "x29"),
            CPUReg::X30 => write!(f, "x30"),
            CPUReg::X31 => write!(f, "x31"),
            CPUReg::PC => write!(f, "PC"),
            CPUReg::IR => write!(f, "IR"),
            CPUReg::TMP0 => write!(f, "TMP0"),
            CPUReg::TMP1 => write!(f, "TMP1"),
            CPUReg::TMP2 => write!(f, "TMP2"),
            CPUReg::TMP3 => write!(f, "TMP3"),
            CPUReg::TMP4 => write!(f, "TMP4"),
            CPUReg::TMP5 => write!(f, "TMP5"),
            CPUReg::TMP6 => write!(f, "TMP6"),
            CPUReg::TMP7 => write!(f, "TMP7"),
        }
    }
}
