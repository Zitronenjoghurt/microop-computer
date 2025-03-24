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
    And(CPUReg, CPUReg, CPUReg),
    Or(CPUReg, CPUReg, CPUReg),
    Sub(CPUReg, CPUReg, CPUReg),
    Xor(CPUReg, CPUReg, CPUReg),
    Sll(CPUReg, CPUReg, CPUReg),
    Srl(CPUReg, CPUReg, CPUReg),
    Sra(CPUReg, CPUReg, CPUReg),
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
            Instruction::Add(rd, rs1, rs2) => write!(f, "ADD {rd} = {rs1} + {rs2}"),
            Instruction::And(rd, rs1, rs2) => write!(f, "AND {rd} = {rs1} & {rs2}"),
            Instruction::Or(rd, rs1, rs2) => write!(f, "OR {rd} = {rs1} | {rs2}"),
            Instruction::Sub(rd, rs1, rs2) => write!(f, "SUB {rd} = {rs1} - {rs2}"),
            Instruction::Xor(rd, rs1, rs2) => write!(f, "XOR {rd} = {rs1} ^ {rs2}"),
            Instruction::Sll(rd, rs1, rs2) => write!(f, "SLL {rd} = {rs1} << {rs2}"),
            Instruction::Srl(rd, rs1, rs2) => write!(f, "SRL {rd} = {rs1} >> {rs2}"),
            Instruction::Sra(rd, rs1, rs2) => write!(f, "SRA {rd} = {rs1} >>* {rs2}"),
            Instruction::Lb(rd, rs1, imm) => write!(f, "LB {rd} = M[{rs1} + {imm}]"),
            Instruction::ECall => write!(f, "ECALL"),
            Instruction::EBreak => write!(f, "EBREAK"),
        }
    }
}
