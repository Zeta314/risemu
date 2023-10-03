use std::mem;

use crate::{
    bus::{Address, Device},
    exception::RVException,
};

pub struct RAM {
    memory: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0x00; size],
        }
    }

    pub fn initialize(&mut self, data: Vec<u8>) {
        self.memory.splice(..data.len(), data.iter().cloned());
    }

    // write routines
    fn write08(&mut self, address: Address, value: u8) {
        let index = address as usize;
        self.memory[index] = value;
    }

    fn write16(&mut self, address: Address, value: u16) {
        let index = address as usize;
        self.memory[index + 0] = ((value >> 0) & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
    }

    fn write32(&mut self, address: Address, value: u32) {
        let index = address as usize;
        self.memory[index + 0] = ((value >> 0) & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((value >> 24) & 0xff) as u8;
    }

    fn write64(&mut self, address: Address, value: u64) {
        let index = address as usize;
        self.memory[index + 0] = ((value >> 0) & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((value >> 24) & 0xff) as u8;
        self.memory[index + 4] = ((value >> 32) & 0xff) as u8;
        self.memory[index + 5] = ((value >> 40) & 0xff) as u8;
        self.memory[index + 6] = ((value >> 48) & 0xff) as u8;
        self.memory[index + 7] = ((value >> 56) & 0xff) as u8;
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

impl Device for RAM {
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

    fn write<T: Sized>(&mut self, address: Address, value: T) -> Result<(), RVException> {
        match 8 * mem::size_of::<T>() {
            08 => Ok(self.write08(address, unsafe { mem::transmute_copy(&value) })),
            16 => Ok(self.write16(address, unsafe { mem::transmute_copy(&value) })),
            32 => Ok(self.write32(address, unsafe { mem::transmute_copy(&value) })),
            64 => Ok(self.write64(address, unsafe { mem::transmute_copy(&value) })),

            _ => Err(RVException::StoreAccessFault),
        }
    }
}
