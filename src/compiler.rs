use crate::compiler::layers::instructions::InstructionLayer;
use crate::compiler::layers::program_builder::ProgramBuilderLayer;
use crate::computer::instructions::Instruction;

pub mod layers;
mod program;

#[derive(Debug, Default)]
pub struct Compiler {
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ProgramBuilderLayer for Compiler {
    fn get_instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    fn get_instructions_mut(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }
}

impl InstructionLayer for Compiler {}
