use crate::compiler::layers::instruction_label::InstructionLabelLayer;
use crate::compiler::layers::instructions::InstructionLayer;
use crate::compiler::layers::program_builder::ProgramBuilderLayer;
use crate::compiler::Compiler;
use crate::computer::components::cpu::registers::builder::CPURegistersBuilderTrait;
use crate::computer::components::cpu::registers::reg::CPUReg::*;
use crate::computer::components::cpu::registers::CPURegistersAccessTrait;
use crate::computer::components::cpu::CPU;
use crate::tests::{setup_and_run, setup_and_run_custom_cpu};

#[test]
fn test_add() {
    let cpu = CPU::builder().x1(230).x2(1337).build();
    let program = Compiler::new().add(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program);
    assert_eq!(computer.cpu.get_register(X3), 1567);
}

#[test]
fn test_lb() {
    let program = Compiler::new()
        .data("test", vec![69])
        .lb_label(X1, X0, "test")
        .compile();
    let computer = setup_and_run(program);
    assert_eq!(computer.cpu.get_register(X1), 69);
}
