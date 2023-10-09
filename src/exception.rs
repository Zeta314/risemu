#[derive(Debug)]
pub enum RVException {
    StoreAccessFault,
    LoadAccessFault,

    IllegalInstruction(u32),
}
