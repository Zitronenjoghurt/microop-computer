use crate::computer::components::cpu::registers::reg::CPUReg;
use crate::computer::components::cpu::registers::reg::CPUReg::*;
use std::collections::VecDeque;

#[derive(Debug, Default, PartialEq)]
pub enum MicroOp {
    #[default]
    Stall,
    Halt,
    /// Decodes the instruction in the instruction register and decomposes it to micro operations
    Decode,

    // Bus operations
    BusRelease,
    BusTake,
    BusReadByte(CPUReg),
    BusReadHalfWord(CPUReg),
    BusReadWord(CPUReg),
    BusReadDoubleWord(CPUReg),
    BusWriteAddress(CPUReg),
    BusWriteData(CPUReg),
    BusSetRead,
    BusSetWriteByte,
    BusSetWriteHalfWord,
    BusSetWriteWord,
    BusSetWriteDoubleWord,

    // ALU operations
    /// rd, rs1, rs2
    ALUAdd(CPUReg, CPUReg, CPUReg),
    ALUSub(CPUReg, CPUReg, CPUReg),

    // Register operations
    RegisterLoadImm(CPUReg, u64),
}

impl MicroOp {
    pub fn default_queue() -> VecDeque<Self> {
        VecDeque::from(vec![
            Self::BusTake,
            Self::BusWriteAddress(PC),
            Self::BusSetRead,
            Self::BusReadWord(IR),
            Self::BusRelease,
            Self::RegisterLoadImm(TMP0, 4),
            Self::ALUAdd(PC, PC, TMP0),
            Self::Decode,
        ])
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct MicroOpResponse {
    pub repeat: bool,
    pub halt: bool,
}

impl MicroOpResponse {
    pub fn new_repeat() -> Self {
        Self {
            repeat: true,
            ..Default::default()
        }
    }

    pub fn new_halt() -> Self {
        Self {
            halt: true,
            ..Default::default()
        }
    }
}
