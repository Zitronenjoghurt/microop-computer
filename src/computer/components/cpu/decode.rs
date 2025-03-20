use crate::computer::components::cpu::micro_op::MicroOp;
use crate::computer::components::cpu::registers::{TMP0, TMP1};

pub fn decode_instruction(instruction: u32) -> Vec<MicroOp> {
    let opcode = instruction as u8 & 0b0111_1111;

    match opcode {
        0b0110011 => decode_r(instruction),
        _ => unimplemented!(),
    }
}

fn decode_r(instruction: u32) -> Vec<MicroOp> {
    let funct3 = get_funct3(instruction);
    let funct7 = get_funct7(instruction);

    let rd = get_rd(instruction);
    let rs1 = get_rs1(instruction);
    let rs2 = get_rs2(instruction);

    match (funct3, funct7) {
        (0x0, 0x00) => decode_add(rd, rs1, rs2),
        _ => unimplemented!(),
    }
}

fn decode_i(instruction: u32, opcode: u8) -> Vec<MicroOp> {
    let funct3 = get_funct3(instruction);
    let imm = ((instruction as i32) >> 20) as u64;

    let rd = get_rd(instruction);
    let rs1 = get_rs1(instruction);

    match (opcode, funct3) {
        (0b0000_0011, 0x0) => decode_lb(rd, rs1, imm),
        _ => unimplemented!(),
    }
}

// BASE INTEGER INSTRUCTIONS
fn decode_add(rd: u8, rs1: u8, rs2: u8) -> Vec<MicroOp> {
    vec![MicroOp::ALUAdd(rd, rs1, rs2)]
}

// LOAD INSTRUCTIONS
fn decode_lb(rd: u8, rs1: u8, imm: u64) -> Vec<MicroOp> {
    vec![
        MicroOp::RegisterLoadImm(TMP0, imm),
        MicroOp::ALUAdd(TMP1, rs1, TMP0),
        MicroOp::BusTake,
        MicroOp::BusWriteAddress(TMP1),
        MicroOp::BusSetRead,
        MicroOp::BusReadByte(rd),
        MicroOp::BusRelease,
    ]
}

// INSTRUCTION FORMAT DECODING
fn get_funct3(instruction: u32) -> u8 {
    (instruction >> 12) as u8 & 0b0000_0111
}

fn get_funct7(instruction: u32) -> u8 {
    (instruction >> 25) as u8 & 0b0111_1111
}

fn get_rd(instruction: u32) -> u8 {
    (instruction >> 7) as u8 & 0b0001_1111
}

fn get_rs1(instruction: u32) -> u8 {
    (instruction >> 15) as u8 & 0b0001_1111
}

fn get_rs2(instruction: u32) -> u8 {
    (instruction >> 20) as u8 & 0b0001_1111
}
