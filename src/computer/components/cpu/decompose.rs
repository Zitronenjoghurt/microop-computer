use crate::computer::components::cpu::micro_op::MicroOp;
use crate::computer::components::cpu::registers::reg::CPUReg;
use crate::computer::components::cpu::registers::reg::CPUReg::*;
use crate::computer::instructions::Instruction;

pub fn decompose_instruction(instruction_bits: u32) -> (Instruction, Vec<MicroOp>) {
    let instruction = Instruction::decode(instruction_bits);
    let queue = match instruction {
        Instruction::Add(rd, rs1, rs2) => decompose_add(rd, rs1, rs2),
        Instruction::And(rd, rs1, rs2) => decompose_and(rd, rs1, rs2),
        Instruction::Or(rd, rs1, rs2) => decompose_or(rd, rs1, rs2),
        Instruction::Sub(rd, rs1, rs2) => decompose_sub(rd, rs1, rs2),
        Instruction::Xor(rd, rs1, imm) => decompose_xor(rd, rs1, imm),
        Instruction::Sll(rd, rs1, rs2) => decompose_sll(rd, rs1, rs2),
        Instruction::Srl(rd, rs1, rs2) => decompose_srl(rd, rs1, rs2),
        Instruction::Sra(rd, rs1, rs2) => decompose_sra(rd, rs1, rs2),
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

fn decompose_and(rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Vec<MicroOp> {
    vec![MicroOp::ALUAnd(rd, rs1, rs2)]
}

fn decompose_or(rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Vec<MicroOp> {
    vec![MicroOp::ALUOr(rd, rs1, rs2)]
}

fn decompose_sub(rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Vec<MicroOp> {
    vec![MicroOp::ALUSub(rd, rs1, rs2)]
}

fn decompose_xor(rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Vec<MicroOp> {
    vec![MicroOp::ALUXor(rd, rs1, rs2)]
}

fn decompose_sll(rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Vec<MicroOp> {
    vec![MicroOp::ALUSll(rd, rs1, rs2)]
}

fn decompose_srl(rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Vec<MicroOp> {
    vec![MicroOp::ALUSrl(rd, rs1, rs2)]
}

fn decompose_sra(rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Vec<MicroOp> {
    vec![MicroOp::ALUSra(rd, rs1, rs2)]
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
