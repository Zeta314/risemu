use crate::{
    bus::{Bus, DRAM_BASE},
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
            dram: DRAM::new(ram_size),
        };

        Self { cpu: CPU::new(bus) }
    }

    pub fn init_dram(&mut self, data: Vec<u8>) {
        self.cpu.bus.dram.initialize(data);
        self.cpu.pc = DRAM_BASE;
    }

    pub fn run(&mut self) {
        let mut last_pc;
        loop {
            last_pc = self.cpu.pc;

            match self.cpu.fetch_and_execute() {
                Ok(_) => {}
                Err(ex) => {
                    println!("{ex:#?} @ {:#x}", self.cpu.pc);

                    match ex {
                        RVException::Breakpoint => {}
                        RVException::EnvironmentCall => {}
                        _ => break,
                    }
                }
            }

            // if the instruction jumps to itself, terminate the simulation
            if last_pc == self.cpu.pc {
                break;
            }
        }
    }
}
