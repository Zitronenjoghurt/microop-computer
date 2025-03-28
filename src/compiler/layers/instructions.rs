use crate::compiler::layers::program_builder::ProgramBuilderLayer;
use crate::computer::components::cpu::registers::reg::CPUReg;
use crate::computer::instructions::Instruction;

pub trait InstructionLayer: ProgramBuilderLayer {
    fn add(mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Self {
        self.add_instruction(Instruction::Add(rd, rs1, rs2));
        self
    }

    fn and(mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Self {
        self.add_instruction(Instruction::And(rd, rs1, rs2));
        self
    }

    fn or(mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Self {
        self.add_instruction(Instruction::Or(rd, rs1, rs2));
        self
    }

    fn sub(mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Self {
        self.add_instruction(Instruction::Sub(rd, rs1, rs2));
        self
    }

    fn xor(mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Self {
        self.add_instruction(Instruction::Xor(rd, rs1, rs2));
        self
    }

    fn sll(mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Self {
        self.add_instruction(Instruction::Sll(rd, rs1, rs2));
        self
    }

    fn srl(mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Self {
        self.add_instruction(Instruction::Srl(rd, rs1, rs2));
        self
    }

    fn sra(mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Self {
        self.add_instruction(Instruction::Sra(rd, rs1, rs2));
        self
    }

    fn lb(mut self, rd: CPUReg, rs1: CPUReg, imm: u64) -> Self {
        self.add_instruction(Instruction::Lb(rd, rs1, imm));
        self
    }
}
