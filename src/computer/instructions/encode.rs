use crate::computer::components::cpu::registers::reg::CPUReg;
use crate::computer::instructions::Instruction;

pub fn encode_instruction(instruction: &Instruction) -> u32 {
    match instruction {
        Instruction::Add(rd, rs1, rs2) => encode_r_type(0x00, *rs2, *rs1, 0x0, *rd, 0b0011_0011),
        Instruction::Sub(rd, rs1, rs2) => encode_r_type(0x20, *rs2, *rs1, 0x0, *rd, 0b0011_0011),
        Instruction::Lb(rd, rs1, imm) => encode_i_type(*imm, *rs1, 0x0, *rd, 0b0000_0011),
        Instruction::ECall => encode_i_type(0x0, 0u8.into(), 0x0, 0u8.into(), 0b111_0011),
        Instruction::EBreak => encode_i_type(0x1, 0u8.into(), 0x0, 0u8.into(), 0b111_0011),
    }
}

fn encode_r_type(fn7: u8, rs2: CPUReg, rs1: CPUReg, fn3: u8, rd: CPUReg, opcode: u8) -> u32 {
    ((fn7 as u32) << 25)
        | ((rs2.to_riscv() as u32 & 0b0001_1111) << 20)
        | ((rs1.to_riscv() as u32 & 0b0001_1111) << 15)
        | (((fn3 & 0b0000_0111) as u32) << 12)
        | ((rd.to_riscv() as u32 & 0b0001_1111) << 7)
        | ((opcode & 0b0111_1111) as u32)
}

fn encode_i_type(imm: u64, rs1: CPUReg, fn3: u8, rd: CPUReg, opcode: u8) -> u32 {
    ((imm as u32) << 20)
        | ((rs1.to_riscv() as u32 & 0b0001_1111) << 15)
        | (((fn3 & 0b0000_0111) as u32) << 12)
        | ((rd.to_riscv() as u32 & 0b0001_1111) << 7)
        | ((opcode & 0b0111_1111) as u32)
}
