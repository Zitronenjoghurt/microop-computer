use crate::compiler::label_reference::LabelReference;
use crate::compiler::layers::instruction_label::InstructionLabelLayer;
use crate::compiler::layers::instructions::InstructionLayer;
use crate::compiler::layers::program_builder::ProgramBuilderLayer;
use crate::compiler::program::Program;
use crate::computer::instructions::Instruction;
use std::collections::HashMap;

mod label_reference;
pub mod layers;
mod program;

#[derive(Debug, Default)]
pub struct Compiler {
    instructions: Vec<Instruction>,
    data: Vec<u8>,
    data_labels: HashMap<String, usize>,
    /// References an instruction index to a data label
    label_references: Vec<LabelReference>,
}

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }

    fn resolve_label_references(&mut self) {
        let data_start_address = self.instructions.len() as u64 * 4;

        self.label_references.iter().for_each(|reference| {
            let instruction = self
                .instructions
                .get_mut(reference.instruction_index)
                .expect(&format!(
                    "Invalid instruction label reference '{}': no instruction at index '{}'",
                    reference.label, reference.instruction_index
                ));
            let data_index = self.data_labels.get(&reference.label).expect(&format!(
                "Invalid instruction label reference '{}': no corresponding data labels found",
                reference.label
            ));

            let address = data_start_address + (*data_index as u64);
            let new_instruction =
                LabelReference::insert_into_instruction(instruction.clone(), address);
            self.instructions[reference.instruction_index] = new_instruction;
        });
    }
}

impl ProgramBuilderLayer for Compiler {
    fn compile(mut self) -> Program {
        self.add_instruction(Instruction::EBreak);
        self.resolve_label_references();
        Program::build(&self.instructions, &self.data)
    }

    fn get_instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    fn get_instructions_mut(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    fn get_data_labels(&self) -> &HashMap<String, usize> {
        &self.data_labels
    }

    fn get_data_labels_mut(&mut self) -> &mut HashMap<String, usize> {
        &mut self.data_labels
    }

    fn get_label_references(&self) -> &Vec<LabelReference> {
        &self.label_references
    }

    fn get_label_references_mut(&mut self) -> &mut Vec<LabelReference> {
        &mut self.label_references
    }
}

impl InstructionLayer for Compiler {}
impl InstructionLabelLayer for Compiler {}
