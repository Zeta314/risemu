use crate::{
    bus::{Address, Bus, Device, RAM_BASE},
    exception::RVException,
};

pub struct CPU {
    pub xregs: [u64; 32],
    pub bus: Bus,
    pub pc: u64,
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        let mut xregs = [0x00; 32];

        // set the stack pointer to the end of RAM
        xregs[2] = RAM_BASE + (bus.ram.size() as u64);

        Self {
            xregs,
            bus,
            pc: 0x00,
        }
    }

    pub fn fetch_and_execute(&mut self) -> Result<(), RVException> {
        let instruction = self.fetch()?;
        self.execute(instruction)?;
        self.pc += 4;

        self.xregs[0] = 0x00; // hardwire x0 to be zero

        Ok(())
    }

    fn read<T: Sized>(&self, address: Address) -> Result<T, RVException> {
        self.bus.read::<T>(address)
    }

    fn write<T: Sized>(&mut self, address: Address, value: T) -> Result<(), RVException> {
        self.bus.write::<T>(address, value)
    }

    fn fetch(&self) -> Result<u32, RVException> {
        self.read::<u32>(self.pc)
    }

    fn execute(&mut self, instruction: u32) -> Result<(), RVException> {
        let _instruction = instruction as u64; // extend it for convenience

        let opcode = _instruction & 0x7F;
        let funct3 = (_instruction & 0x00007000) >> 12;
        let funct7 = (_instruction & 0xFE000000) >> 25;

        let dest = ((_instruction & 0xF80) >> 7) as usize;
        let source1 = ((_instruction & 0x000F8000) >> 15) as usize;
        let source2 = ((_instruction & 0x01F00000) >> 20) as usize;

        match opcode {
            // LUI
            0b0110111 => {
                self.xregs[dest] = _instruction & 0xFFFFF000;
            }

            // AUIPC
            0b0010111 => {
                self.xregs[dest] = self.pc + (_instruction & 0xFFFFF000);
            }

            // JAL
            0b1101111 => {
                self.xregs[dest] = self.pc + 0x04;

                let offset = ((_instruction & 0x80000000) as i32 as i64 >> 11) as u64
                    | (_instruction & 0xFF000)
                    | ((_instruction >> 9) & 0x800)
                    | ((_instruction >> 20) & 0x7FE);

                self.pc += offset;
                self.pc -= 0x04;
            }

            // JALR
            0b1100111 => {
                let tmp = self.pc.wrapping_add(0x04);
                let offset = (_instruction as i32 as i64) >> 20;
                let target = ((self.xregs[source1] as i64) + offset) & !0x01;

                self.pc = target as u64;
                self.pc -= 0x04;

                self.xregs[dest] = tmp;
            }

            // BRANCH
            0b1100011 => {
                let immediate = ((_instruction & 0x80000000) as i32 as i64 >> 19) as u64
                    | ((_instruction & 0x80) << 4)
                    | ((_instruction >> 20) & 0x7E0)
                    | ((_instruction >> 7) & 0x1E);

                match funct3 {
                    // BEQ
                    0b000 => {
                        if self.xregs[source1] == self.xregs[source2] {
                            self.pc += immediate;
                            self.pc -= 0x04;
                        }
                    }

                    // BNE
                    0b001 => {
                        if self.xregs[source1] != self.xregs[source2] {
                            self.pc += immediate;
                            self.pc -= 0x04;
                        }
                    }

                    // BLT
                    0b100 => {
                        if (self.xregs[source1] as i64) < (self.xregs[source2] as i64) {
                            self.pc += immediate;
                            self.pc -= 0x04;
                        }
                    }

                    // BGE
                    0b101 => {
                        if (self.xregs[source1] as i64) >= (self.xregs[source2] as i64) {
                            self.pc += immediate;
                            self.pc -= 0x04;
                        }
                    }

                    // BLTU
                    0b110 => {
                        if self.xregs[source1] < self.xregs[source2] {
                            self.pc += immediate;
                            self.pc -= 0x04;
                        }
                    }

                    // BGEU
                    0b111 => {
                        if self.xregs[source1] >= self.xregs[source2] {
                            self.pc += immediate;
                            self.pc -= 0x04;
                        }
                    }

                    _ => return Err(RVException::IllegalInstruction),
                }
            }

            // LOAD
            0b0000011 => {
                let offset = ((_instruction as i32 as i64) >> 20) as u64;
                let address = self.xregs[source1] + offset;

                match funct3 {
                    // LB
                    0b000 => {
                        let value = self.read::<i8>(address)?;
                        self.xregs[dest] = value as i64 as u64;
                    }

                    // LH
                    0b001 => {
                        let value = self.read::<i16>(address)?;
                        self.xregs[dest] = value as i64 as u64;
                    }

                    // LW
                    0b010 => {
                        let value = self.read::<i32>(address)?;
                        self.xregs[dest] = value as i64 as u64;
                    }

                    // LD
                    0b011 => {
                        let value = self.read::<i64>(address)?;
                        self.xregs[dest] = value as u64;
                    }

                    // LBU
                    0b100 => {
                        let value = self.read::<u8>(address)?;
                        self.xregs[dest] = value as u64;
                    }

                    // LHU
                    0b101 => {
                        let value = self.read::<u16>(address)?;
                        self.xregs[dest] = value as u64;
                    }

                    // LWU
                    0b110 => {
                        let value = self.read::<u32>(address)?;
                        self.xregs[dest] = value as u64;
                    }

                    _ => return Err(RVException::IllegalInstruction),
                }
            }

            // STORE
            0b0100011 => {
                let offset = (((_instruction & 0xFE000000) as i32 as i64 >> 20) as u64)
                    | ((_instruction >> 7) & 0x1F);
                let address = self.xregs[source1] + offset;

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

                    // SD
                    0b011 => {
                        self.write::<u64>(address, self.xregs[source2] as u64)?;
                    }

                    _ => return Err(RVException::IllegalInstruction),
                }
            }

            // IMMEDIATE
            0b0010011 => {
                let immediate = ((_instruction as i32 as i64) >> 20) as u64;
                let funct6 = funct7 >> 1;

                match funct3 {
                    // ADDI
                    0b000 => {
                        self.xregs[dest] = self.xregs[source1] + immediate;
                    }

                    // SLLI
                    0b001 => {
                        let shift = (_instruction >> 20) & 0x3F;
                        self.xregs[dest] = self.xregs[source1] << shift;
                    }

                    // SLTI
                    0b010 => {
                        self.xregs[dest] = if (self.xregs[source1] as i64) < (immediate as i64) {
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
                        let shift = (_instruction >> 20) & 0x3F;

                        match funct6 {
                            0x00 => {
                                self.xregs[dest] = self.xregs[source1] >> shift;
                            }

                            0x10 => {
                                self.xregs[dest] = ((self.xregs[source1] as i64) >> shift) as u64;
                            }

                            _ => return Err(RVException::IllegalInstruction),
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

                    _ => return Err(RVException::IllegalInstruction),
                }
            }

            // IMMEDIATE32
            0b0011011 => {
                let immediate = ((_instruction as i32 as i64) >> 20) as u64;
                let shift = (immediate & 0x1F) as u32;

                match funct3 {
                    // ADDIW
                    0b000 => {
                        self.xregs[dest] = (self.xregs[source1] + immediate) as i32 as i64 as u64;
                    }

                    // SLLIW
                    0b001 => {
                        self.xregs[dest] = (self.xregs[source1] << shift) as i32 as i64 as u64;
                    }

                    // SRLIW & SRAIW
                    0b101 => match funct7 {
                        0b000000 => {
                            self.xregs[dest] = (self.xregs[source1] as u32 >> shift) as i64 as u64;
                        }

                        0b100000 => {
                            self.xregs[dest] = (self.xregs[source1] as i32 >> shift) as i64 as u64;
                        }

                        _ => return Err(RVException::IllegalInstruction),
                    },

                    _ => return Err(RVException::IllegalInstruction),
                }
            }

            // OPERATION
            0b0110011 => {
                match (funct3, funct7) {
                    // ADD
                    (0b000, 0b0000000) => {
                        self.xregs[dest] = self.xregs[source1] + self.xregs[source2];
                    }

                    // SUB
                    (0b000, 0b0100000) => {
                        self.xregs[dest] = self.xregs[source1] - self.xregs[source2];
                    }

                    // SLL
                    (0b001, 0b0000000) => {
                        self.xregs[dest] = self.xregs[source1] << (self.xregs[source2] & 0x3F);
                    }

                    // SLT
                    (0b010, 0b0000000) => {
                        self.xregs[dest] =
                            if (self.xregs[source1] as i64) < (self.xregs[source2] as i64) {
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
                            ((self.xregs[source1] as i64) >> (self.xregs[source2] & 0x3F)) as u64;
                    }

                    // OR
                    (0b110, 0b0000000) => {
                        self.xregs[dest] = self.xregs[source1] | self.xregs[source2];
                    }

                    // AND
                    (0b111, 0b0000000) => {
                        self.xregs[dest] = self.xregs[source1] & self.xregs[source2];
                    }

                    // =============================================================================
                    // RV64M

                    // MUL
                    (0b000, 0b0000001) => {
                        self.xregs[dest] =
                            ((self.xregs[source1] as i64) * (self.xregs[source2] as i64)) as u64;
                    }

                    // MULH
                    (0b001, 0b0000001) => {
                        self.xregs[dest] = ((self.xregs[source1] as i64 as i128)
                            * (self.xregs[source2] as i64 as i128)
                            >> 64) as u64;
                    }

                    // MULHSU
                    (0b010, 0b0000001) => {
                        self.xregs[dest] = ((self.xregs[source1] as i64 as u128)
                            * (self.xregs[source2] as u128)
                            >> 64) as u64;
                    }

                    // MULHU
                    (0b011, 0b0000001) => {
                        self.xregs[dest] = ((self.xregs[source1] as u128)
                            * (self.xregs[source2] as u128)
                            >> 64) as u64;
                    }

                    // DIV
                    (0b100, 0b0000001) => {
                        let dividend = self.xregs[source1] as i64;
                        let divisor = self.xregs[source2] as i64;

                        self.xregs[dest] = if divisor == 0 {
                            u64::MAX // division by zero
                        } else if dividend == i64::MIN && divisor == -1 {
                            dividend as u64 // overflow
                        } else {
                            (dividend / divisor) as u64
                        }
                    }

                    // DIVU
                    (0b101, 0b0000001) => {
                        let dividend = self.xregs[source1];
                        let divisor = self.xregs[source2];

                        self.xregs[dest] = if divisor == 0 {
                            u64::MAX // division by zero
                        } else {
                            dividend / divisor
                        }
                    }

                    // REM
                    (0b110, 0b0000001) => {
                        let dividend = self.xregs[source1] as i64;
                        let divisor = self.xregs[source2] as i64;

                        self.xregs[dest] = if divisor == 0 {
                            dividend as u64 // division by zero
                        } else if dividend == i64::MIN && divisor == -1 {
                            0 // overflow
                        } else {
                            (dividend % divisor) as u64
                        }
                    }

                    // REMU
                    (0b111, 0b0000001) => {
                        let dividend = self.xregs[source1];
                        let divisor = self.xregs[source2];

                        self.xregs[dest] = if divisor == 0 {
                            dividend // division by zero
                        } else {
                            dividend % divisor
                        }
                    }

                    _ => return Err(RVException::IllegalInstruction),
                }
            }

            // OPERATION32
            0b111011 => match (funct3, funct7) {
                // ADDW
                (0b000, 0b0000000) => {
                    self.xregs[dest] =
                        (self.xregs[source1] + self.xregs[source2]) as i32 as i64 as u64;
                }

                // SUBW
                (0b000, 0b0100000) => {
                    self.xregs[dest] = (self.xregs[source1] - self.xregs[source2]) as i32 as u64;
                }

                // SLLW
                (0b001, 0b0000000) => {
                    self.xregs[dest] =
                        (self.xregs[source1] << (self.xregs[source2] & 0x3F)) as i32 as i64 as u64;
                }

                // SRLW
                (0b101, 0b0000000) => {
                    self.xregs[dest] = (self.xregs[source1] as u32 >> (self.xregs[source2] & 0x1F))
                        as i32 as i64 as u64;
                }

                // SRAW
                (0b101, 0b0100000) => {
                    self.xregs[dest] =
                        (self.xregs[source1] as i32 >> (self.xregs[source2] & 0x1F)) as i64 as u64;
                }

                // =================================================================================
                // RV64M

                // MULW
                (0b000, 0b0000001) => {
                    self.xregs[dest] =
                        ((self.xregs[source1] as i32) * (self.xregs[source2] as i32)) as i64 as u64;
                }

                // DIVW
                (0b100, 0b0000001) => {
                    let dividend = self.xregs[source1] as i32;
                    let divisor = self.xregs[source2] as i32;

                    self.xregs[dest] = if divisor == 0 {
                        u64::MAX // division by zero
                    } else if dividend == i32::MIN && divisor == -1 {
                        dividend as i64 as u64 // overflow
                    } else {
                        (dividend / divisor) as i64 as u64
                    }
                }

                // DIVUW
                (0b101, 0b0000001) => {
                    let dividend = self.xregs[source1] as u32;
                    let divisor = self.xregs[source2] as u32;

                    self.xregs[dest] = if divisor == 0 {
                        u64::MAX // division by zero
                    } else {
                        (dividend / divisor) as i32 as i64 as u64
                    }
                }

                // REMW
                (0b110, 0b0000001) => {
                    let dividend = self.xregs[source1] as i32;
                    let divisor = self.xregs[source2] as i32;

                    self.xregs[dest] = if divisor == 0 {
                        dividend as i64 as u64 // division by zero
                    } else if dividend == i32::MIN && divisor == -1 {
                        0 // overflow
                    } else {
                        (dividend % divisor) as i64 as u64
                    }
                }

                // REMUW
                (0b111, 0b0000001) => {
                    let dividend = self.xregs[source1] as i32;
                    let divisor = self.xregs[source2] as i32;

                    self.xregs[dest] = if divisor == 0 {
                        dividend as i32 as i64 as u64 // division by zero
                    } else {
                        (dividend % divisor) as i32 as i64 as u64
                    }
                }

                _ => return Err(RVException::IllegalInstruction),
            },

            // MEM-MISC
            0b0001111 => {
                match funct3 {
                    // FENCE
                    0b000 => {}

                    _ => return Err(RVException::IllegalInstruction),
                }
            }

            // SYSTEM
            0b1110011 => {
                match source2 {
                    // ECALL
                    0b000000000000 => return Err(RVException::EnvironmentCall),

                    // EBREAK
                    0b000000000001 => return Err(RVException::Breakpoint),

                    _ => return Err(RVException::IllegalInstruction),
                }
            }

            _ => return Err(RVException::IllegalInstruction),
        }

        Ok(())
    }
}
