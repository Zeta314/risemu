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
        let mut cpu = CPU {
            xregs: [0x00; 32],
            csrs: [0x00; 4096],
            pc: 0x00,
            bus: Bus {
                dram: DRAM::new(ram_size),
            },
        };

        // set the stack pointer to the end of DRAM
        cpu.xregs[2] = DRAM_BASE + (ram_size as u64);

        Self { cpu }
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
