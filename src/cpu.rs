use std::mem;

use crate::{
    bus::{Address, Bus},
    exception::RVException,
};

pub type Register = u32;
pub type Instruction = u32;

#[repr(u32)]
enum Opcodes {
    LUI = 0b0110111,
    AUIPC = 0b0010111,
    JAL = 0b1101111,
    JALR = 0b1100111,
    BRANCH = 0b1100011,
    LOAD = 0b0000011,
    STORE = 0b0100011,
    IMM = 0b0010011,
    OP = 0b0110011,
    MISC = 0b0001111,
}

pub struct CPU {
    pub xregs: [Register; 32],
    pub pc: Register,
    pub bus: Bus,
}

impl CPU {
    pub fn cycle(&mut self) -> Result<(), RVException> {
        self.xregs[0] = 0x00; // hardwire x0 to be zero

        let instruction = self.fetch()?;
        self.execute(instruction)?;

        Ok(())
    }

    fn read<T: Sized>(&self, address: Address) -> Result<T, RVException> {
        self.bus.read::<T>(address)
    }

    fn write<T: Sized>(&mut self, address: Address, value: T) -> Result<(), RVException> {
        self.bus.write::<T>(address, value)
    }

    fn fetch(&self) -> Result<Instruction, RVException> {
        self.read::<Instruction>(self.pc)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), RVException> {
        let opcode = instruction & 0x7F;
        let funct3 = (instruction & 0x00007000) >> 12;
        let funct7 = (instruction & 0xFe000000) >> 25;

        let dest = ((instruction & 0xF80) >> 7) as usize;
        let source1 = ((instruction & 0x000F8000) >> 15) as usize;
        let source2 = ((instruction & 0x01F00000) >> 20) as usize;

        match unsafe { mem::transmute(opcode) } {
            Opcodes::LUI => {
                self.xregs[dest] = instruction & 0xFFFFF000;
            }

            Opcodes::AUIPC => {
                self.xregs[dest] = self.pc.wrapping_add(instruction & 0xFFFFF000);
            }

            Opcodes::JAL => {
                self.xregs[dest] = self.pc.wrapping_add(0x04);

                let offset = ((instruction & 0x80000000) as i32 >> 11) as u32
                    | (instruction & 0xff000)
                    | ((instruction >> 9) & 0x800)
                    | ((instruction >> 20) & 0x7fe);

                self.pc = self.pc.wrapping_add(offset).wrapping_sub(0x04);
            }

            Opcodes::JALR => {
                let tmp = self.pc.wrapping_add(0x04);
                let offset = (instruction as i32) >> 20;
                let target = ((self.xregs[source1] as i32).wrapping_add(offset)) & !0x01;

                self.pc = (target as u32).wrapping_sub(0x04);
                self.xregs[dest] = tmp;
            }

            Opcodes::BRANCH => {
                let immediate = ((instruction & 0x80000000) as i32 >> 19) as u32
                    | ((instruction & 0x80) << 4)
                    | ((instruction >> 20) & 0x7e0)
                    | ((instruction >> 7) & 0x1e);

                match funct3 {
                    // BEQ
                    0b000 => {
                        if self.xregs[source1] == self.xregs[source2] {
                            self.pc = self.pc.wrapping_add(immediate).wrapping_sub(0x04);
                        }
                    }

                    // BNE
                    0b001 => {
                        if self.xregs[source1] != self.xregs[source2] {
                            self.pc = self.pc.wrapping_add(immediate).wrapping_sub(0x04);
                        }
                    }

                    // BLT
                    0b100 => {
                        if (self.xregs[source1] as i32) < (self.xregs[source2] as i32) {
                            self.pc = self.pc.wrapping_add(immediate).wrapping_sub(0x04);
                        }
                    }

                    // BGE
                    0b101 => {
                        if (self.xregs[source1] as i32) >= (self.xregs[source2] as i32) {
                            self.pc = self.pc.wrapping_add(immediate).wrapping_sub(0x04);
                        }
                    }

                    // BLTU
                    0b110 => {
                        if self.xregs[source1] < self.xregs[source2] {
                            self.pc = self.pc.wrapping_add(immediate).wrapping_sub(0x04);
                        }
                    }

                    // BGEU
                    0b111 => {
                        if self.xregs[source1] >= self.xregs[source2] {
                            self.pc = self.pc.wrapping_add(immediate).wrapping_sub(0x04);
                        }
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            Opcodes::LOAD => {
                let offset = ((instruction as i32) >> 20) as u32;
                let address = self.xregs[source1].wrapping_add(offset);

                match funct3 {
                    // LB
                    0b000 => {
                        let value = self.read::<i8>(address)?;
                        self.xregs[dest] = value as i32 as u32;
                    }

                    // LH
                    0b001 => {
                        let value = self.read::<i16>(address)?;
                        self.xregs[dest] = value as i32 as u32;
                    }

                    // LW
                    0b010 => {
                        let value = self.read::<i32>(address)?;
                        self.xregs[dest] = value as u32;
                    }

                    // LBU
                    0b100 => {
                        let value = self.read::<u8>(address)?;
                        self.xregs[dest] = value as u32;
                    }

                    // LHU
                    0b101 => {
                        let value = self.read::<u16>(address)?;
                        self.xregs[dest] = value as u32;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            Opcodes::STORE => {
                let offset = (((instruction & 0xFE000000) as i32 >> 20) as u32)
                    | ((instruction >> 7) & 0x1F);
                let address = self.xregs[source1].wrapping_add(offset);

                match funct3 {
                    // SB
                    0b000 => {
                        self.write::<u8>(address, self.xregs[source2] as u8)?;
                    }

                    // SH
                    0b001 => {
                        self.write::<u16>(address, self.xregs[source2] as u16)?;
                    }

                    // SW
                    0b010 => {
                        self.write::<u32>(address, self.xregs[source2] as u32)?;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            Opcodes::IMM => {
                let immediate = ((instruction as i32) >> 20) as u32;
                let funct6 = funct7 >> 1;

                match funct3 {
                    // ADDI
                    0b000 => {
                        self.xregs[dest] = self.xregs[source1].wrapping_add(immediate);
                    }

                    // SLLI
                    0b001 => {
                        let shift = (instruction >> 20) & 0x3F;
                        self.xregs[dest] = self.xregs[source1] << shift;
                    }

                    // SLTI
                    0b010 => {
                        self.xregs[dest] = if (self.xregs[source1] as i32) < (immediate as i32) {
                            1
                        } else {
                            0
                        }
                    }

                    // SLTIU
                    0b011 => {
                        self.xregs[dest] = if self.xregs[source1] < immediate {
                            1
                        } else {
                            0
                        };
                    }

                    // XORI
                    0b100 => {
                        self.xregs[dest] = self.xregs[source1] ^ immediate;
                    }

                    // SRLI & SRAI
                    0b101 => {
                        let shift = (instruction >> 20) & 0x3F;

                        match funct6 {
                            0x00 => {
                                self.xregs[dest] = self.xregs[source1] >> shift;
                            }

                            0x10 => {
                                self.xregs[dest] = ((self.xregs[source1] as i32) >> shift) as u32;
                            }

                            _ => return Err(RVException::IllegalInstruction(instruction)),
                        }
                    }

                    // ORI
                    0b110 => {
                        self.xregs[dest] = self.xregs[source1] | immediate;
                    }

                    // ANDI
                    0b111 => {
                        self.xregs[dest] = self.xregs[source1] & immediate;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            Opcodes::OP => {
                match (funct3, funct7) {
                    // ADD
                    (0b000, 0b0000000) => {
                        self.xregs[dest] = self.xregs[source1].wrapping_add(self.xregs[source2]);
                    }

                    // SUB
                    (0b000, 0b0100000) => {
                        self.xregs[dest] = self.xregs[source1].wrapping_sub(self.xregs[source2]);
                    }

                    // SLL
                    (0b001, 0b0000000) => {
                        self.xregs[dest] = self.xregs[source1] << (self.xregs[source2] & 0x3F);
                    }

                    // SLT
                    (0b010, 0b0000000) => {
                        self.xregs[dest] =
                            if (self.xregs[source1] as i32) < (self.xregs[source2] as i32) {
                                1
                            } else {
                                0
                            }
                    }

                    // SLTU
                    (0b011, 0b0000000) => {
                        self.xregs[dest] = if self.xregs[source1] < self.xregs[source2] {
                            1
                        } else {
                            0
                        }
                    }

                    // XOR
                    (0b100, 0b0000000) => {
                        self.xregs[dest] = self.xregs[source1] ^ self.xregs[source2];
                    }

                    // SRL
                    (0b101, 0b0000000) => {
                        self.xregs[dest] = self.xregs[source1] >> (self.xregs[source2] & 0x3F);
                    }

                    // SRA
                    (0b101, 0b0100000) => {
                        self.xregs[dest] =
                            ((self.xregs[source1] as i32) >> (self.xregs[source2] & 0x3F)) as u32;
                    }

                    // OR
                    (0b110, 0b0000000) => {
                        self.xregs[dest] = self.xregs[source1] | self.xregs[source2];
                    }

                    // AND
                    (0b111, 0b0000000) => {
                        self.xregs[dest] = self.xregs[source1] & self.xregs[source2];
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            Opcodes::MISC => {
                match funct3 {
                    // FENCE
                    0b000 => {}

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }
        }

        Ok(())
    }
}
