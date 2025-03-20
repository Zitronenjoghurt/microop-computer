use crate::computer::components::cpu::micro_op::MicroOp;

pub fn decode_instruction(instruction: u32) -> Vec<MicroOp> {
    let opcode = instruction as u8 & 0b0111_1111;

    match opcode {
        0b0110011 => decode_r(instruction, opcode),
        _ => unimplemented!(),
    }
}

fn decode_r(instruction: u32, opcode: u8) -> Vec<MicroOp> {
    let funct3 = (instruction >> 12) as u8 & 0b0000_0111;
    let funct7 = (instruction >> 25) as u8 & 0b0111_1111;

    let rd = (instruction >> 7) as u8 & 0b0001_1111;
    let rs1 = (instruction >> 15) as u8 & 0b0001_1111;
    let rs2 = (instruction >> 20) as u8 & 0b0001_1111;

    match (funct3, funct7) {
        (0x0, 0x00) => decode_add(rd, rs1, rs2),
        _ => unimplemented!(),
    }
}

// BASE INTEGER INSTRUCTIONS
fn decode_add(rd: u8, rs1: u8, rs2: u8) -> Vec<MicroOp> {
    vec![MicroOp::ALUAdd(rd, rs1, rs2)]
}
