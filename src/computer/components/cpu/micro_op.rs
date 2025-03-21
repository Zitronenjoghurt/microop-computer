use crate::computer::components::cpu::registers::CPUReg;
use std::collections::VecDeque;

#[derive(Debug, Default, PartialEq)]
pub enum MicroOp {
    #[default]
    Stall,

    /// Fetch/Decode Ops
    BusWritePC,
    BusReadIR,
    IncrementPC,
    Decode,

    // Bus operations
    BusRelease,
    BusTake,
    BusReadByte(CPUReg),
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
            Self::BusWritePC,
            Self::BusSetRead,
            Self::BusReadIR,
            Self::BusRelease,
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
