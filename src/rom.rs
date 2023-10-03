use std::mem;

use crate::{
    bus::{Address, Device},
    exception::RVException,
};

pub struct ROM {
    memory: Vec<u8>,
}

impl ROM {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0x00; size],
        }
    }

    pub fn initialize(&mut self, data: Vec<u8>) {
        self.memory.splice(..data.len(), data.iter().cloned());
    }

    // read routines
    fn read08(&self, address: Address) -> u8 {
        let index = address as usize;
        self.memory[index]
    }

    fn read16(&self, address: Address) -> u16 {
        let index = address as usize;
        (self.memory[index] as u16) | ((self.memory[index + 1] as u16) << 8)
    }

    fn read32(&self, address: Address) -> u32 {
        let index = address as usize;
        (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24)
    }

    fn read64(&self, address: Address) -> u64 {
        let index = address as usize;
        (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
            | ((self.memory[index + 2] as u64) << 16)
            | ((self.memory[index + 3] as u64) << 24)
            | ((self.memory[index + 4] as u64) << 32)
            | ((self.memory[index + 5] as u64) << 40)
            | ((self.memory[index + 6] as u64) << 48)
            | ((self.memory[index + 7] as u64) << 56)
    }
}

impl Device for ROM {
    fn size(&self) -> usize {
        self.memory.len()
    }

    fn read<T: Sized>(&self, address: Address) -> Result<T, RVException> {
        match 8 * mem::size_of::<T>() {
            08 => Ok(unsafe { mem::transmute_copy(&self.read08(address)) }),
            16 => Ok(unsafe { mem::transmute_copy(&self.read16(address)) }),
            32 => Ok(unsafe { mem::transmute_copy(&self.read32(address)) }),
            64 => Ok(unsafe { mem::transmute_copy(&self.read64(address)) }),

            _ => Err(RVException::LoadAccessFault),
        }
    }

    fn write<T: Sized>(&mut self, _address: Address, _value: T) -> Result<(), RVException> {
        Err(RVException::StoreAccessFault)
    }
}
