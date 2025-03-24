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
#[case::nz_c(u64::MAX, 1337, u64::MAX, false, true)]
#[case::z_nc(0, 0, 0, true, false)]
fn test_add(
    #[case] a: u64,
    #[case] b: u64,
    #[case] result: u64,
    #[case] zero: bool,
    #[case] carry: bool,
) {
    let cpu = CPU::builder().x1(a).x2(b).build();
    let program = Compiler::new().add(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program, 7);
    assert_eq!(computer.cpu.get_register(X3), result);
    assert_eq!(computer.cpu.get_zero(), zero);
    assert_eq!(computer.cpu.get_carry(), carry);
    assert!(!computer.cpu.get_subtract());
}

#[rstest]
#[case::nz(0b1010, 0b1111, 0b1010, false)]
#[case::z(0, 0, 0, true)]
fn test_and(#[case] a: u64, #[case] b: u64, #[case] result: u64, #[case] zero: bool) {
    let cpu = CPU::builder()
        .x1(a)
        .x2(b)
        .carry(true)
        .subtract(true)
        .build();
    let program = Compiler::new().and(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program, 7);
    assert_eq!(computer.cpu.get_register(X3), result);
    assert_eq!(computer.cpu.get_zero(), zero);
    assert!(!computer.cpu.get_carry());
    assert!(!computer.cpu.get_subtract());
}

#[rstest]
#[case::nz(0b1010, 0b1110, 0b1110, false)]
#[case::z(0, 0, 0, true)]
fn test_or(#[case] a: u64, #[case] b: u64, #[case] result: u64, #[case] zero: bool) {
    let cpu = CPU::builder()
        .x1(a)
        .x2(b)
        .carry(true)
        .subtract(true)
        .build();
    let program = Compiler::new().or(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program, 7);
    assert_eq!(computer.cpu.get_register(X3), result);
    assert_eq!(computer.cpu.get_zero(), zero);
    assert!(!computer.cpu.get_carry());
    assert!(!computer.cpu.get_subtract());
}

#[rstest]
#[case::nz_nc(1337, 1235, 102, false, false)]
#[case::z_nc(513, 513, 0, true, false)]
#[case::z_c(1, u64::MAX, 0, true, true)]
fn test_sub(
    #[case] a: u64,
    #[case] b: u64,
    #[case] result: u64,
    #[case] zero: bool,
    #[case] carry: bool,
) {
    let cpu = CPU::builder().x1(a).x2(b).build();
    let program = Compiler::new().sub(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program, 7);
    assert_eq!(computer.cpu.get_register(X3), result);
    assert_eq!(computer.cpu.get_zero(), zero);
    assert_eq!(computer.cpu.get_carry(), carry);
    assert!(computer.cpu.get_subtract());
}

#[rstest]
#[case::nz(0b1010, 0b1110, 0b0100, false)]
#[case::z(0, 0, 0, true)]
fn test_xor(#[case] a: u64, #[case] b: u64, #[case] result: u64, #[case] zero: bool) {
    let cpu = CPU::builder()
        .x1(a)
        .x2(b)
        .carry(true)
        .subtract(true)
        .build();
    let program = Compiler::new().xor(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program, 7);
    assert_eq!(computer.cpu.get_register(X3), result);
    assert_eq!(computer.cpu.get_zero(), zero);
    assert!(!computer.cpu.get_carry());
    assert!(!computer.cpu.get_subtract());
}

#[rstest]
#[case(0b1010, 2, 0b101000)]
#[case(120, 1, 240)]
fn test_sll(#[case] value: u64, #[case] shift: u64, #[case] result: u64) {
    let cpu = CPU::builder().x1(value).x2(shift).build();
    let program = Compiler::new().sll(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program, 7);
    assert_eq!(computer.cpu.get_register(X3), result);
}

#[rstest]
#[case(0b1010, 2, 0b10)]
#[case(120, 1, 60)]
fn test_srl(#[case] value: u64, #[case] shift: u64, #[case] result: u64) {
    let cpu = CPU::builder().x1(value).x2(shift).build();
    let program = Compiler::new().srl(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program, 7);
    assert_eq!(computer.cpu.get_register(X3), result);
}

#[rstest]
#[case(-2, 1, -1)]
#[case(-1280, 3, -160)]
fn test_sra(#[case] value: i64, #[case] shift: u64, #[case] result: i64) {
    let cpu = CPU::builder().x1(value as u64).x2(shift).build();
    let program = Compiler::new().sra(X3, X1, X2).compile();
    let computer = setup_and_run_custom_cpu(cpu, program, 7);
    assert_eq!(computer.cpu.get_register(X3), result as u64);
}

#[test]
fn test_lb() {
    let program = Compiler::new()
        .data("test", vec![69])
        .lb_label(X1, X0, "test")
        .compile();
    let computer = setup_and_run(program, 13);
    assert_eq!(computer.cpu.get_register(X1), 69);
}
