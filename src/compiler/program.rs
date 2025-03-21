use crate::computer::instructions::Instruction;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Program {
    pub data: Vec<u8>,
}

impl Program {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn from_instructions(instructions: &Vec<Instruction>) -> Self {
        let data = instructions
            .iter()
            .map(|instruction| instruction.to_byte_vector())
            .flatten()
            .collect();
        Self::new(data)
    }

    pub fn display_binary(&self) -> String {
        let mut result = String::new();
        let chunks = self.data.chunks(4);

        for (i, chunk) in chunks.enumerate() {
            let line = format!(
                "[0x{:04X}] {:08b}{:08b}{:08b}{:08b}",
                i * 4,
                chunk[3],
                chunk[2],
                chunk[1],
                chunk[0]
            );
            result.push_str(&line);
            result.push('\n');
        }

        result
    }
}
