use crate::compiler::program::Program;
use crate::computer::instructions::Instruction;

pub trait ProgramBuilderLayer {
    fn get_instructions(&self) -> &Vec<Instruction>;
    fn get_instructions_mut(&mut self) -> &mut Vec<Instruction>;

    fn add_instruction(&mut self, instruction: Instruction) {
        self.get_instructions_mut().push(instruction);
    }

    fn get_program(&self) -> Program {
        Program::from_instructions(self.get_instructions())
    }
}
