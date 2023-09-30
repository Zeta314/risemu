use crate::cpu::Instruction;

pub enum RVException {
    StoreAccessFault,
    LoadAccessFault,
    IllegalInstruction(Instruction),
}
