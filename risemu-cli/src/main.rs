use std::fs;

use clap::Parser;
use risemu::{emulator::Emulator, bus::ROM_BASE};

use crate::args::CLIArgs;

mod args;

fn main() {
    let args = CLIArgs::parse();
    let mut emulator = Emulator::new(args.rom_size, args.ram_size);
    
    let rom_data = fs::read(args.program).expect("failed to read ROM file");
    emulator.init_rom(rom_data);

    if let Some(ram_file) = args.ram {
        let ram_data = fs::read(ram_file).expect("failed to read RAM file");
        emulator.init_ram(ram_data);
    }

    emulator.run();
}
