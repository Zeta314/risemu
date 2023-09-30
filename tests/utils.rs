use riscv_emu::{bus::DRAM_BASE, cpu::Register, emulator::Emulator};

const DRAM_SIZE: usize = 0x10000;

fn create_xregs(values: Vec<(usize, Register)>) -> [Register; 32] {
    let mut xregs = [0; 32];

    // initialize the stack pointer
    xregs[2] = DRAM_BASE + (DRAM_SIZE as u32);

    // initialize the given registers
    for (idx, value) in values.iter() {
        xregs[*idx] = *value;
    }

    xregs
}

pub fn execute(code: Vec<u8>, xregs: Vec<(usize, Register)>) {
    let mut emulator = Emulator::new(DRAM_SIZE);

    let code_length = code.len();
    emulator.init_dram(code);

    while emulator.cpu.pc < (DRAM_BASE + (code_length as u32)) {
        match emulator.cpu.fetch_and_execute() {
            Ok(_) => {}
            Err(ex) => {
                println!("error @ {:#x} [{:#?}]", emulator.cpu.pc, ex);
                break;
            }
        }
    }

    // check the registers values
    let xregs = create_xregs(xregs);
    for (idx, expected) in xregs.iter().enumerate() {
        assert_eq!(
            *expected, emulator.cpu.xregs[idx],
            "expected register {idx} to be {expected:#x} but it's {:#x}",
            emulator.cpu.xregs[idx]
        );
    }
}
