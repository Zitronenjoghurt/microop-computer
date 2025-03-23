use crate::compiler::layers::instruction_label::InstructionLabelLayer;
use crate::compiler::layers::instructions::InstructionLayer;
use crate::compiler::layers::program_builder::ProgramBuilderLayer;
use crate::compiler::Compiler;
use crate::computer::components::cpu::registers::builder::CPURegistersBuilderTrait;
use crate::computer::components::cpu::registers::flags::CPUFlagsAccessTrait;
use crate::computer::components::cpu::registers::reg::CPUReg::*;
use crate::computer::components::cpu::registers::CPURegistersAccessTrait;
use crate::computer::components::cpu::CPU;
use crate::tests::{setup_and_run, setup_and_run_custom_cpu};
use rstest::rstest;

#[rstest]
#[case::nz_nc(230, 1337, 1567, false, false)]
#[case::nz_c(u64::MAX, 1337, 1336, false, true)]
#[case::z_nc(0, 0, 0, true, false)]
#[case::z_c(u64::MAX, 1, 0, true, true)]
fn test_add(
    #[case] a: u64,
    #[case] b: u64,
    #[case] result: u64,
    #[case] zero: bool,
    #[case] carry: bool,
) {
    let cpu = CPU::builder().x1(a).x2(b).build();
    let program = Compiler::new().add(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program, 9);
    assert_eq!(computer.cpu.get_register(X3), result);
    assert_eq!(computer.cpu.get_zero(), zero);
    assert_eq!(computer.cpu.get_carry(), carry);
    assert!(!computer.cpu.get_subtract());
}

#[rstest]
#[case::nz_nc(1337, 1235, 102, false, false)]
#[case::nz_c(1, u64::MAX, 2, false, true)]
#[case::z_nc(513, 513, 0, true, false)]
fn test_sub(
    #[case] a: u64,
    #[case] b: u64,
    #[case] result: u64,
    #[case] zero: bool,
    #[case] carry: bool,
) {
    let cpu = CPU::builder().x1(a).x2(b).build();
    let program = Compiler::new().sub(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program, 9);
    assert_eq!(computer.cpu.get_register(X3), result);
    assert_eq!(computer.cpu.get_zero(), zero);
    assert_eq!(computer.cpu.get_carry(), carry);
    assert!(computer.cpu.get_subtract());
}

#[test]
fn test_lb() {
    let program = Compiler::new()
        .data("test", vec![69])
        .lb_label(X1, X0, "test")
        .compile();
    let computer = setup_and_run(program, 15);
    assert_eq!(computer.cpu.get_register(X1), 69);
}
