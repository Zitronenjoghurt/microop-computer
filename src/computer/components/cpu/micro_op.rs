use crate::computer::components::cpu::registers::{CPUReg, CPUReg::*};
use std::collections::VecDeque;

#[derive(Debug, Default, PartialEq)]
pub enum MicroOp {
    #[default]
    Stall,
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
    BusSetWrite,

    // ALU operations
    /// rd, rs1, rs2
    ALUAdd(CPUReg, CPUReg, CPUReg),

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
}

impl MicroOpResponse {
    pub fn new_repeat() -> Self {
        Self {
            repeat: true,
            ..Default::default()
        }
    }
}
