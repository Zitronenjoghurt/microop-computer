use crate::computer::instructions::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct LabelReference {
    pub instruction_index: usize,
    pub label: String,
}

impl LabelReference {
    pub fn new(label: String, instruction_index: usize) -> Self {
        Self {
            instruction_index,
            label,
        }
    }

    pub fn insert_into_instruction(instruction: Instruction, address: u64) -> Instruction {
        match instruction {
            Instruction::Lb(rd, rs1, _) => Instruction::Lb(rd, rs1, address),
            _ => unimplemented!(),
        }
    }
}
