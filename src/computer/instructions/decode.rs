use crate::computer::components::cpu::registers::reg::CPUReg;
use crate::computer::instructions::Instruction;

pub fn decode_instruction(instruction: u32) -> Instruction {
    let opcode = instruction as u8 & 0b0111_1111;

    match opcode {
        0b000_0011 | 0b111_0011 => decode_i(instruction, opcode),
        0b011_0011 => decode_r(instruction, opcode),
        _ => unimplemented!(),
    }
}

fn decode_r(instruction: u32, opcode: u8) -> Instruction {
    let funct3 = get_funct3(instruction);
    let funct7 = get_funct7(instruction);

    let rd = get_rd(instruction);
    let rs1 = get_rs1(instruction);
    let rs2 = get_rs2(instruction);

    match (funct3, funct7, opcode) {
        (0x0, 0x00, 0b011_0011) => Instruction::Add(rd, rs1, rs2),
        (0x7, 0x00, 0b011_0011) => Instruction::And(rd, rs1, rs2),
        (0x6, 0x00, 0b011_0011) => Instruction::Or(rd, rs1, rs2),
        (0x0, 0x20, 0b011_0011) => Instruction::Sub(rd, rs1, rs2),
        (0x4, 0x00, 0b011_0011) => Instruction::Xor(rd, rs1, rs2),
        (0x1, 0x00, 0b011_0011) => Instruction::Sll(rd, rs1, rs2),
        (0x5, 0x00, 0b011_0011) => Instruction::Srl(rd, rs1, rs2),
        (0x5, 0x20, 0b011_0011) => Instruction::Sra(rd, rs1, rs2),
        _ => unimplemented!(),
    }
}

fn decode_i(instruction: u32, opcode: u8) -> Instruction {
    let funct3 = get_funct3(instruction);
    let imm = ((instruction as i32) >> 20) as u64;

    let rd = get_rd(instruction);
    let rs1 = get_rs1(instruction);

    match (opcode, funct3, imm) {
        (0b000_0011, 0x0, _) => Instruction::Lb(rd, rs1, imm),
        (0b111_0011, 0x0, 0x0) => Instruction::ECall,
        (0b111_0011, 0x0, 0x1) => Instruction::EBreak,
        _ => unimplemented!(),
    }
}

// INSTRUCTION FORMAT DECODING
fn get_funct3(instruction: u32) -> u8 {
    (instruction >> 12) as u8 & 0b0000_0111
}

fn get_funct7(instruction: u32) -> u8 {
    (instruction >> 25) as u8 & 0b0111_1111
}

fn get_rd(instruction: u32) -> CPUReg {
    ((instruction >> 7) as u8 & 0b0001_1111).into()
}

fn get_rs1(instruction: u32) -> CPUReg {
    ((instruction >> 15) as u8 & 0b0001_1111).into()
}

fn get_rs2(instruction: u32) -> CPUReg {
    ((instruction >> 20) as u8 & 0b0001_1111).into()
}
