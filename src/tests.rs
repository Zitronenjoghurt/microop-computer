use crate::compiler::program::Program;
use crate::computer::components::cpu::CPU;
use crate::computer::Computer;

mod test_instructions;

pub fn setup_and_run(program: Program, ticks: u64) -> Computer {
    let mut computer = Computer::new();
    computer.set_boot_rom(program.binary);
    for _ in 0..ticks {
        if !computer.tick() {
            break;
        }
    }
    computer
}

pub fn setup_and_run_custom_cpu(cpu: CPU, program: Program, ticks: u64) -> Computer {
    let mut computer = Computer::new();
    computer.cpu = cpu;
    computer.set_boot_rom(program.binary);
    for _ in 0..ticks {
        if !computer.tick() {
            break;
        }
    }
    computer
}
