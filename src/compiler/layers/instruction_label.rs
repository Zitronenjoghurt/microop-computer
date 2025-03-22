use crate::compiler::layers::instructions::InstructionLayer;
use crate::computer::components::cpu::registers::CPUReg;

pub trait InstructionLabelLayer: InstructionLayer {
    fn lb_label(mut self, rd: CPUReg, rs1: CPUReg, data_label: &str) -> Self {
        let instruction_index = self.get_instructions().len();
        self = self.lb(rd, rs1, 0);
        self.add_label_reference(data_label.to_owned(), instruction_index);
        self
    }
}
