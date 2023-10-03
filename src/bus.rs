use crate::{dram::DRAM, exception::RVException, rom::ROM};

pub type Address = u64;

pub const DRAM_BASE: Address = 0x8000_0000;
pub const ROM_BASE: Address = 0x1000;

pub trait Device {
    fn size(&self) -> usize;
    fn read<T: Sized>(&self, address: Address) -> Result<T, RVException>;
    fn write<T: Sized>(&mut self, address: Address, value: T) -> Result<(), RVException>;
}

pub struct Bus {
    pub dram: DRAM,
    pub rom: ROM,
}

impl Bus {
    pub fn read<T: Sized>(&self, address: Address) -> Result<T, RVException> {
        if address >= DRAM_BASE && address < (DRAM_BASE + self.dram.size() as Address) {
            return self.dram.read::<T>(address - DRAM_BASE);
        }

        if address >= ROM_BASE && address < (ROM_BASE + self.rom.size() as Address) {
            return self.rom.read::<T>(address - ROM_BASE);
        }

        Err(RVException::LoadAccessFault)
    }

    pub fn write<T: Sized>(&mut self, address: Address, value: T) -> Result<(), RVException> {
        if address >= DRAM_BASE && address < (DRAM_BASE + self.dram.size() as Address) {
            return self.dram.write::<T>(address - DRAM_BASE, value);
        }

        Err(RVException::StoreAccessFault)
    }
}
