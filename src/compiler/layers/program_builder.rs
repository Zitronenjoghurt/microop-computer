use crate::compiler::label_reference::LabelReference;
use crate::compiler::program::Program;
use crate::computer::instructions::Instruction;
use std::collections::HashMap;

pub trait ProgramBuilderLayer: Sized {
    fn compile(self) -> Program;
    fn get_instructions(&self) -> &Vec<Instruction>;
    fn get_instructions_mut(&mut self) -> &mut Vec<Instruction>;
    fn get_data(&self) -> &Vec<u8>;
    fn get_data_mut(&mut self) -> &mut Vec<u8>;
    fn get_data_labels(&self) -> &HashMap<String, usize>;
    fn get_data_labels_mut(&mut self) -> &mut HashMap<String, usize>;
    fn get_label_references(&self) -> &Vec<LabelReference>;
    fn get_label_references_mut(&mut self) -> &mut Vec<LabelReference>;

    fn add_instruction(&mut self, instruction: Instruction) {
        self.get_instructions_mut().push(instruction);
    }

    fn data(mut self, label: &str, data: Vec<u8>) -> Self {
        let start_index = self.get_data().len();
        self.get_data_mut().extend(data);
        self.add_data_label(label.to_owned(), start_index);
        self
    }

    fn add_data_label(&mut self, name: String, index: usize) {
        self.get_data_labels_mut().insert(name, index);
    }

    fn add_label_reference(&mut self, label: String, instruction_index: usize) {
        let label_reference = LabelReference::new(label, instruction_index);
        self.get_label_references_mut().push(label_reference);
    }
}
