#[derive(Debug)]
pub enum RVException {
    StoreAccessFault,
    LoadAccessFault,

    IllegalInstruction,

    EnvironmentCall,
    Breakpoint,
}
