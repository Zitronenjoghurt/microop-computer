use crate::computer::components::cpu::registers::CPUReg;
use crate::computer::instructions::decode::decode_instruction;
use crate::computer::instructions::encode::encode_instruction;

mod decode;
mod encode;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    /// rd, rs1, rs2
    Add(CPUReg, CPUReg, CPUReg),
    /// rd, rs1, imm
    Lb(CPUReg, CPUReg, u64),
}

impl Instruction {
    pub fn encode(&self) -> u32 {
        encode_instruction(&self)
    }

    pub fn decode(instruction: u32) -> Instruction {
        decode_instruction(instruction)
    }

    pub fn to_byte_vector(&self) -> Vec<u8> {
        let encoded = self.encode();
        vec![
            encoded as u8,
            (encoded >> 8) as u8,
            (encoded >> 16) as u8,
            (encoded >> 24) as u8,
        ]
    }
}
