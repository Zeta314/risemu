use crate::{bus::Bus, cpu::CPU, ram::RAM, exception::RVException, rom::ROM};

pub struct Emulator {
    pub cpu: CPU,
}

impl Emulator {
    pub fn new(rom_size: usize, ram_size: usize) -> Self {
        let bus = Bus {
            dram: RAM::new(ram_size),
            rom: ROM::new(rom_size),
        };

        Self { cpu: CPU::new(bus) }
    }

    pub fn init_rom(&mut self, data: Vec<u8>) {
        self.cpu.bus.rom.initialize(data);
    }

    pub fn init_dram(&mut self, data: Vec<u8>) {
        self.cpu.bus.dram.initialize(data);
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
