use crate::computer::components::cpu::micro_op::MicroOp;
use crate::computer::components::cpu::registers::{CPUReg, CPUReg::*};
use crate::computer::instructions::Instruction;

pub fn decompose_instruction(instruction_bits: u32) -> (Instruction, Vec<MicroOp>) {
    let instruction = Instruction::decode(instruction_bits);
    let queue = match instruction {
        Instruction::Add(rd, rs1, rs2) => decompose_add(rd, rs1, rs2),
        Instruction::Lb(rd, rs1, imm) => decompose_lb(rd, rs1, imm),
        Instruction::ECall => vec![MicroOp::Halt],
        Instruction::EBreak => vec![MicroOp::Halt],
    };
    (instruction, queue)
}

// BASE INTEGER INSTRUCTIONS
fn decompose_add(rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Vec<MicroOp> {
    vec![MicroOp::ALUAdd(rd, rs1, rs2)]
}

// LOAD INSTRUCTIONS
fn decompose_lb(rd: CPUReg, rs1: CPUReg, imm: u64) -> Vec<MicroOp> {
    vec![
        MicroOp::RegisterLoadImm(TMP0, imm),
        MicroOp::ALUAdd(TMP1, rs1, TMP0),
        MicroOp::BusTake,
        MicroOp::BusWriteAddress(TMP1),
        MicroOp::BusSetRead,
        MicroOp::BusReadByte(rd),
        MicroOp::BusRelease,
    ]
}
