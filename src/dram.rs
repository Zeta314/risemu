use std::mem;

use crate::{bus::Device, exception::RVException};

pub struct DRAM {
    memory: Vec<u8>,
}

impl DRAM {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0u8; size],
        }
    }

    pub fn initialize(&mut self, data: Vec<u8>) {
        self.memory.splice(..data.len(), data.iter().cloned());
    }

    // write routines
    fn write08(&mut self, address: u32, value: u8) {
        let index = address as usize;
        self.memory[index] = value;
    }

    fn write16(&mut self, address: u32, value: u16) {
        let index = address as usize;
        self.memory[index + 0] = ((value >> 0) & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
    }

    fn write32(&mut self, address: u32, value: u32) {
        let index = address as usize;
        self.memory[index + 0] = ((value >> 0) & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((value >> 24) & 0xff) as u8;
    }

    // read routines
    fn read08(&self, address: u32) -> u8 {
        let index = address as usize;
        self.memory[index]
    }

    fn read16(&self, address: u32) -> u16 {
        let index = address as usize;
        (self.memory[index] as u16) | ((self.memory[index + 1] as u16) << 8)
    }

    fn read32(&self, address: u32) -> u32 {
        let index = address as usize;
        (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24)
    }
}

impl Device for DRAM {
    fn size(&self) -> usize {
        self.memory.len()
    }

    fn read<T: Sized>(&self, address: u32) -> Result<T, RVException> {
        match mem::size_of::<T>() {
            08 => Ok(unsafe { mem::transmute_copy(&self.read08(address)) }),
            16 => Ok(unsafe { mem::transmute_copy(&self.read16(address)) }),
            32 => Ok(unsafe { mem::transmute_copy(&self.read32(address)) }),

            _ => Err(RVException::LoadAccessFault),
        }
    }

    fn write<T: Sized>(&mut self, address: u32, value: T) -> Result<(), RVException> {
        match mem::size_of::<T>() {
            08 => Ok(self.write08(address, unsafe { mem::transmute_copy(&value) })),
            16 => Ok(self.write16(address, unsafe { mem::transmute_copy(&value) })),
            32 => Ok(self.write32(address, unsafe { mem::transmute_copy(&value) })),

            _ => Err(RVException::StoreAccessFault),
        }
    }
}
