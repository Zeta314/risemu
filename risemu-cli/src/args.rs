use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLIArgs {
    #[arg(value_name = "PROGRAM")]
    pub program: PathBuf,

    #[arg(short, long, value_name = "IMAGE")]
    pub ram: Option<PathBuf>,

    #[arg(long, default_value_t = 0x5000)]
    pub ram_size: usize,

    #[arg(long, default_value_t = 0x1000)]
    pub rom_size: usize,
}
