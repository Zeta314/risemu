use crate::{
    bus::{Bus, RAM_BASE},
    cpu::CPU,
    dram::DRAM,
    exception::RVException,
};

pub struct Emulator {
    pub cpu: CPU,
}

impl Emulator {
    pub fn new(ram_size: usize) -> Self {
        let bus = Bus {
            ram: DRAM::new(ram_size),
        };

        Self { cpu: CPU::new(bus) }
    }

    pub fn init_ram(&mut self, data: Vec<u8>) {
        self.cpu.bus.ram.initialize(data);
        self.cpu.pc = RAM_BASE;
    }

    pub fn run(&mut self) -> Result<(), RVException> {
        loop {
            self.cpu.fetch_and_execute()?;
        }
    }
}
