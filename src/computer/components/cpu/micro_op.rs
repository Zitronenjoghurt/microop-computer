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
    BusReadByte(u8),
    BusWriteAddress(u8),
    BusWriteData(u8),
    BusSetRead,
    BusSetWrite,

    // ALU operations
    /// rd, rs1, rs2
    ALUAdd(u8, u8, u8),

    // Register operations
    RegisterLoadImm(u8, u64),
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
