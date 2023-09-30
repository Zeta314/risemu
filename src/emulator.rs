use crate::{
    bus::{Bus, DRAM_BASE},
    cpu::CPU,
    dram::DRAM,
};

pub struct Emulator {
    pub cpu: CPU,
}

impl Emulator {
    pub fn new(ram_size: usize) -> Self {
        let mut cpu = CPU {
            xregs: [0x00; 32],
            pc: 0x00,
            bus: Bus {
                dram: DRAM::new(ram_size),
            },
        };

        // set the stack pointer to the end of DRAM
        cpu.xregs[2] = DRAM_BASE + (ram_size as u32);

        Self { cpu }
    }

    pub fn init_dram(&mut self, data: Vec<u8>) {
        self.cpu.bus.dram.initialize(data);
        self.cpu.pc = DRAM_BASE;
    }
}
