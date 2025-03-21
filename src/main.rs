use crate::compiler::layers::instructions::InstructionLayer;
use crate::compiler::layers::program_builder::ProgramBuilderLayer;
use crate::compiler::Compiler;
use crate::computer::components::cpu::registers::CPUReg::*;

mod compiler;
mod computer;
mod utils;

fn main() {
    let program = Compiler::new()
        .lb(X1, X0, 0xFF)
        .add(X1, X1, X1)
        .get_program();
    println!("{}", program.display_binary());
}
