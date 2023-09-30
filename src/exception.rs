use crate::cpu::Instruction;

#[derive(Debug)]
pub enum RVException {
    StoreAccessFault,
    LoadAccessFault,
    IllegalInstruction(Instruction),
}
