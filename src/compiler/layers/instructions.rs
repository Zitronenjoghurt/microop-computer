use crate::compiler::layers::program_builder::ProgramBuilderLayer;
use crate::computer::components::cpu::registers::CPUReg;
use crate::computer::instructions::Instruction;

pub trait InstructionLayer: ProgramBuilderLayer {
    fn add(mut self, rd: CPUReg, rs1: CPUReg, rs2: CPUReg) -> Self {
        self.add_instruction(Instruction::Add(rd, rs1, rs2));
        self
    }

    fn lb(mut self, rd: CPUReg, rs1: CPUReg, imm: u64) -> Self {
        self.add_instruction(Instruction::Lb(rd, rs1, imm));
        self
    }
}
