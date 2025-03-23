use crate::compiler::layers::instruction_label::InstructionLabelLayer;
use crate::compiler::layers::instructions::InstructionLayer;
use crate::compiler::layers::program_builder::ProgramBuilderLayer;
use crate::compiler::Compiler;
use crate::computer::components::cpu::registers::CPURegistersAccessTrait;
use crate::computer::Computer;
use crate::logging::initialize_logging;
use computer::components::cpu::registers::reg::CPUReg::*;

mod compiler;
mod computer;
mod logging;
#[cfg(test)]
mod tests;
mod utils;

fn main() {
    initialize_logging();

    let program = Compiler::new()
        .data("lb_data", vec![17])
        .lb_label(X1, X0, "lb_data")
        .add(X1, X1, X1)
        .compile();
    println!("{}", program.display_binary());

    let mut computer = Computer::new();
    computer.set_boot_rom(program.binary);

    while computer.tick() {}
    println!("{}", computer.cpu.get_registers());
}
