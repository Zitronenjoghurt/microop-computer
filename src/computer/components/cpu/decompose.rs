use crate::computer::components::cpu::micro_op::MicroOp;
use crate::computer::components::cpu::registers::CPUReg;
use crate::computer::instructions::decode::decode_instruction;
use crate::computer::instructions::Instruction;

pub fn decompose_instruction(instruction_bits: u32) -> Vec<MicroOp> {
    let instruction = decode_instruction(instruction_bits);
    match instruction {
        Instruction::Add(rd, rs1, rs2) => decompose_add(rd, rs1, rs2),
        Instruction::Lb(rd, rs1, imm) => decompose_lb(rd, rs1, imm),
    }
}

// BASE INTEGER INSTRUCTIONS
fn decompose_add(rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Vec<MicroOp> {
    vec![MicroOp::ALUAdd(rd, rs1, rs2)]
}

// LOAD INSTRUCTIONS
fn decompose_lb(rd: CPUReg, rs1: CPUReg, imm: u64) -> Vec<MicroOp> {
    vec![
        MicroOp::RegisterLoadImm(CPUReg::TMP0, imm),
        MicroOp::ALUAdd(CPUReg::TMP1, rs1, CPUReg::TMP0),
        MicroOp::BusTake,
        MicroOp::BusWriteAddress(CPUReg::TMP1),
        MicroOp::BusSetRead,
        MicroOp::BusReadByte(rd),
        MicroOp::BusRelease,
    ]
}
