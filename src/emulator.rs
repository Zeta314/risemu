use crate::{
    bus::{Address, Bus, ROM_BASE},
    cpu::CPU,
    exception::RVException,
    ram::RAM,
    rom::ROM,
};

pub struct Emulator {
    pub cpu: CPU,
}

impl Emulator {
    pub fn new(rom_size: usize, ram_size: usize) -> Self {
        let bus = Bus {
            ram: RAM::new(ram_size),
            rom: ROM::new(rom_size),
        };

        Self { cpu: CPU::new(bus) }
    }

    pub fn init_rom(&mut self, data: Vec<u8>) {
        self.cpu.bus.rom.initialize(data);
        self.cpu.pc = ROM_BASE;
    }

    pub fn init_ram(&mut self, data: Vec<u8>) {
        self.cpu.bus.ram.initialize(data);
    }

    pub fn set_pc(&mut self, value: Address) {
        self.cpu.pc = value;
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
