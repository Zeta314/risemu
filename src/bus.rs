use crate::{dram::DRAM, exception::RVException};

pub type Address = u32;

const DRAM_BASE: Address = 0x8000_0000;

pub trait Device {
    fn size(&self) -> usize;
    fn read<T: Sized>(&self, address: Address) -> Result<T, RVException>;
    fn write<T: Sized>(&mut self, address: Address, value: T) -> Result<(), RVException>;
}

pub struct Bus {
    dram: DRAM,
}

impl Bus {
    pub fn read<T: Sized>(&self, address: Address) -> Result<T, RVException> {
        if address >= DRAM_BASE && address < (self.dram.size() as Address) {
            return self.dram.read::<T>(address);
        }

        Err(RVException::LoadAccessFault)
    }

    pub fn write<T: Sized>(&mut self, address: Address, value: T) -> Result<(), RVException> {
        if address >= DRAM_BASE && address < (self.dram.size() as Address) {
            return self.dram.write::<T>(address, value);
        }

        Err(RVException::StoreAccessFault)
    }
}
