use crate::{
    bus::{Address, Bus, RAM_BASE},
    cpu::CPU,
    dram::DRAM,
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

    pub fn set_pc(&mut self, value: Address) {
        self.cpu.pc = value;
    }

    pub fn run(&mut self) {
        loop {
            match self.cpu.fetch_and_execute() {
                Ok(_) => {}
                Err(ex) => {
                    println!("{ex:#?} @ {:#x}", self.cpu.pc);

                    break;
                }
            }
        }
    }
}
