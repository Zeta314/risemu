use crate::{ram::RAM, exception::RVException, rom::ROM};

pub type Address = u64;

pub const RAM_BASE: Address = 0x8000_0000;
pub const ROM_BASE: Address = 0x1000;

pub trait Device {
    fn size(&self) -> usize;
    fn read<T: Sized>(&self, address: Address) -> Result<T, RVException>;
    fn write<T: Sized>(&mut self, address: Address, value: T) -> Result<(), RVException>;
}

pub struct Bus {
    pub ram: RAM,
    pub rom: ROM,
}

impl Bus {
    pub fn read<T: Sized>(&self, address: Address) -> Result<T, RVException> {
        if address >= RAM_BASE && address < (RAM_BASE + self.ram.size() as Address) {
            return self.ram.read::<T>(address - RAM_BASE);
        }

        if address >= ROM_BASE && address < (ROM_BASE + self.rom.size() as Address) {
            return self.rom.read::<T>(address - ROM_BASE);
        }

        Err(RVException::LoadAccessFault)
    }

    pub fn write<T: Sized>(&mut self, address: Address, value: T) -> Result<(), RVException> {
        if address >= RAM_BASE && address < (RAM_BASE + self.ram.size() as Address) {
            return self.ram.write::<T>(address - RAM_BASE, value);
        }

        Err(RVException::StoreAccessFault)
    }
}
