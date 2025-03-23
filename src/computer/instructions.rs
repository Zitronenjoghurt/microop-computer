use crate::computer::components::cpu::registers::reg::CPUReg;
use crate::computer::instructions::decode::decode_instruction;
use crate::computer::instructions::encode::encode_instruction;
use std::fmt::Display;

mod decode;
mod encode;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    /// rd, rs1, rs2
    Add(CPUReg, CPUReg, CPUReg),
    /// rd, rs1, imm
    Lb(CPUReg, CPUReg, u64),
    ECall,
    EBreak,
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

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Add(rd, rs1, rs2) => write!(f, "ADD {} = {} + {}", rd, rs1, rs2),
            Instruction::Lb(rd, rs1, imm) => write!(f, "LB {} = M[{} + {}]", rd, rs1, imm),
            Instruction::ECall => write!(f, "ECALL"),
            Instruction::EBreak => write!(f, "EBREAK"),
        }
    }
}
