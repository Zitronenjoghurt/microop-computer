use crate::computer::instructions::Instruction;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Program {
    pub binary: Vec<u8>,
    data_start: usize,
}

impl Program {
    pub fn new(binary: Vec<u8>, data_start: usize) -> Self {
        Self { binary, data_start }
    }

    pub fn build(instructions: &Vec<Instruction>, additional_data: &Vec<u8>) -> Self {
        let mut binary: Vec<u8> = instructions
            .iter()
            .map(|instruction| instruction.to_byte_vector())
            .flatten()
            .collect();

        let data_start = binary.len();
        binary.extend(additional_data);
        Self::new(binary, data_start)
    }

    pub fn display_binary(&self) -> String {
        let mut result = String::from("=={PROGRAM}==\n");
        let chunks = self.binary.chunks(4);

        let mut printed_data_start = false;
        for (i, chunk) in chunks.enumerate() {
            if i * 4 >= self.data_start && !printed_data_start {
                result.push_str("\n=={DATA}==\n");
                printed_data_start = true;
            }
            let line = format!(
                "[0x{:04X}] {:08b}{:08b}{:08b}{:08b}",
                i * 4,
                chunk.get(3).unwrap_or(&0),
                chunk.get(2).unwrap_or(&0),
                chunk.get(1).unwrap_or(&0),
                chunk.get(0).unwrap_or(&0)
            );
            result.push_str(&line);
            result.push('\n');
        }

        result
    }
}
