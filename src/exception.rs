#[derive(Debug)]
pub enum RVException {
    StoreAccessFault,
    LoadAccessFault,
    LoadAddressMisaligned,

    IllegalInstruction(u32),

    EnvironmentCall,
    Breakpoint,
}
