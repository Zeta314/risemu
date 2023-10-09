use std::{cmp, num::FpCategory, time::Instant};

use crate::{
    bus::{Address, Bus, Device, RAM_BASE},
    exception::RVException,
};

pub struct CPU {
    pub xregs: [u64; 32],
    pub fregs: [f64; 32],
    pub csrs: [u64; 4096],
    pub bus: Bus,
    pub pc: u64,

    time_update: Option<Instant>,
    reservation_set: Vec<u64>,
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        let mut xregs = [0x00; 32];
        let fregs = [0.00; 32];
        let csrs = [0x00; 4096];

        // set the stack pointer to the end of RAM
        xregs[2] = RAM_BASE + (bus.ram.size() as u64);

        Self {
            xregs,
            fregs,
            csrs,
            bus,
            pc: 0x00,
            time_update: None,
            reservation_set: vec![],
        }
    }

    pub fn fetch_and_execute(&mut self) -> Result<(), RVException> {
        // fetch & execute the instruction
        let instruction = self.fetch()?;
        self.execute(instruction)?;
        self.pc += 4;

        self.update();

        Ok(())
    }

    fn update(&mut self) {
        self.xregs[0] = 0x00; // hardwire x0 to be zero
        self.csrs[0xC00] += 1; // increment the cycles CSR

        // update the time CSR accordingly based on real-time
        match self.time_update {
            Some(ref mut time_update) => {
                let diff = time_update.elapsed();
                if diff.as_millis() >= 1000 {
                    self.csrs[0xC01] += 1;
                    self.time_update = Some(Instant::now());
                }
            }

            None => {
                self.time_update = Some(Instant::now());
            }
        }
    }

    fn read<T: Sized>(&self, address: Address) -> Result<T, RVException> {
        self.bus.read::<T>(address)
    }

    fn write<T: Sized>(&mut self, address: Address, value: T) -> Result<(), RVException> {
        // the SC must fail if a write from some other device to the bytes
        // accessed by the LR can be observed to occur between the LR and SC.
        if self.reservation_set.contains(&address) {
            self.reservation_set.retain(|&x| x != address);
        }

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

                    _ => return Err(RVException::IllegalInstruction(instruction)),
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

                    _ => return Err(RVException::IllegalInstruction(instruction)),
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

                    _ => return Err(RVException::IllegalInstruction(instruction)),
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

                        _ => return Err(RVException::IllegalInstruction(instruction)),
                    },

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // OPERATION
            0b0110011 => {
                match (funct3, funct7) {
                    // =============================================================================
                    // RV64I

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
                            self.csrs[0x03] |= 1 << 3; // set the DZ flag
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
                            self.csrs[0x03] |= 1 << 3; // set the DZ flag
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

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // OPERATION32
            0b111011 => match (funct3, funct7) {
                // =============================================================================
                // RV64I

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
                        self.csrs[0x03] |= 1 << 3; // set the DZ flag
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
                        self.csrs[0x03] |= 1 << 3; // set the DZ flag
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

                _ => return Err(RVException::IllegalInstruction(instruction)),
            },

            // =====================================================================================
            // RV64F

            // LOAD-FP
            0b0000111 => {
                let offset = ((_instruction as i32 as i64) >> 20) as u64;
                let address = self.xregs[source1] + offset;

                match funct3 {
                    // RV64F
                    // FLW
                    0b010 => {
                        self.fregs[dest] = f32::from_bits(self.read::<u32>(address)?) as f64;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // STORE-FP
            0b0100111 => {
                let offset = ((((_instruction as i32 as i64) >> 20) as u64) & 0xfe0)
                    | ((_instruction >> 7) & 0x1f);
                let address = self.xregs[source1] + offset;

                match funct3 {
                    // RV64F
                    // FSW
                    0b010 => {
                        self.write::<u32>(address, (self.fregs[source2] as f32).to_bits())?;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // FMADD
            0b1000011 => {
                let source3 = ((_instruction & 0xf8000000) >> 27) as usize;
                let funct2 = (_instruction & 0x03000000) >> 25;

                match funct2 {
                    // FMADD.S
                    0b00 => {
                        self.fregs[dest] =
                            ((self.fregs[source1] as f32) * (self.fregs[source2] as f32)
                                + (self.fregs[source3] as f32)) as f64;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // FMSUB
            0b1000111 => {
                let source3 = ((_instruction & 0xf8000000) >> 27) as usize;
                let funct2 = (_instruction & 0x03000000) >> 25;

                match funct2 {
                    // FMSUB.S
                    0b00 => {
                        self.fregs[dest] =
                            ((self.fregs[source1] as f32) * (self.fregs[source2] as f32)
                                - (self.fregs[source3] as f32)) as f64;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // FNMADD
            0b1001111 => {
                let source3 = ((_instruction & 0xf8000000) >> 27) as usize;
                let funct2 = (_instruction & 0x03000000) >> 25;

                match funct2 {
                    // FNMSUB.S
                    0b00 => {
                        self.fregs[dest] =
                            ((-self.fregs[source1] as f32) * (self.fregs[source2] as f32)
                                + (self.fregs[source3] as f32)) as f64;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // FNMSUB
            0b1001011 => {
                let source3 = ((_instruction & 0xf8000000) >> 27) as usize;
                let funct2 = (_instruction & 0x03000000) >> 25;

                match funct2 {
                    // FNMSUB.S
                    0b00 => {
                        self.fregs[dest] =
                            ((-self.fregs[source1] as f32) * (self.fregs[source2] as f32)
                                - (self.fregs[source3] as f32)) as f64;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // OP-FP
            0b1010011 => {
                // make sure that the FRM field is valid
                match (self.csrs[0x003] & (1 << 8)) >> 5 {
                    0b000 => {}
                    0b001 => {}
                    0b010 => {}
                    0b011 => {}
                    0b100 => {}
                    0b111 => {}

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }

                match funct7 {
                    // FADD.S
                    0b0000000 => {
                        self.fregs[dest] =
                            ((self.fregs[source1] as f32) + (self.fregs[source2] as f32)) as f64;
                    }

                    // FSUB.S
                    0b0000100 => {
                        self.fregs[dest] =
                            ((self.fregs[source1] as f32) - (self.fregs[source2] as f32)) as f64;
                    }

                    // FMUL.S
                    0b0001000 => {
                        self.fregs[dest] =
                            ((self.fregs[source1] as f32) * (self.fregs[source2] as f32)) as f64;
                    }

                    // FDIV.S
                    0b0001100 => {
                        self.fregs[dest] =
                            ((self.fregs[source1] as f32) / (self.fregs[source2] as f32)) as f64;
                    }

                    // FSQRT.S
                    0b0101100 => {
                        self.fregs[dest] = (self.fregs[source1] as f32).sqrt() as f64;
                    }

                    0b0010000 => match funct3 {
                        // FSGNJ.S
                        0b000 => {
                            self.fregs[dest] = self.fregs[source1].copysign(self.fregs[source2]);
                        }

                        // FSGNJN.S
                        0b001 => {
                            self.fregs[dest] = self.fregs[source1].copysign(-self.fregs[source2]);
                        }

                        // FSGNJX.S
                        0b010 => {
                            let sign1 = (self.fregs[source1] as f32).to_bits() & 0x80000000;
                            let sign2 = (self.fregs[source2] as f32).to_bits() & 0x80000000;
                            let other = (self.fregs[source1] as f32).to_bits() & 0x7fffffff;

                            self.fregs[dest] = f32::from_bits((sign1 ^ sign2) | other) as f64;
                        }

                        _ => return Err(RVException::IllegalInstruction(instruction)),
                    },

                    0b0010100 => match funct3 {
                        // FMIN.S
                        0b000 => {
                            self.fregs[dest] = self.fregs[source1].min(self.fregs[source2]);
                        }

                        // FMAX.S
                        0b001 => {
                            self.fregs[dest] = self.fregs[source1].max(self.fregs[source2]);
                        }

                        _ => return Err(RVException::IllegalInstruction(instruction)),
                    },

                    0b1100000 => match source2 {
                        // FCVT.W.S
                        0b00000 => {
                            self.xregs[dest] = (self.fregs[source1] as f32).round() as i32 as u64;
                        }

                        // FCVT.WU.S
                        0b00001 => {
                            self.xregs[dest] =
                                (self.fregs[source1] as f32).round() as u32 as i32 as u64;
                        }

                        // FCVT.L.S
                        0b00010 => {
                            self.xregs[dest] = (self.fregs[source1] as f32).round() as u64;
                        }

                        // FCVT.LU.S
                        0b00011 => {
                            self.xregs[dest] = (self.fregs[source1] as f32).round() as u64;
                        }

                        _ => return Err(RVException::IllegalInstruction(instruction)),
                    },

                    0b1110000 => match funct3 {
                        // FMV.X.W
                        0b000 => {
                            self.xregs[dest] =
                                (self.fregs[source1].to_bits() & 0xffffffff) as i32 as i64 as u64;
                        }

                        // FCLASS.S
                        0b001 => {
                            let tmp = self.fregs[source1];

                            match tmp.classify() {
                                FpCategory::Infinite => {
                                    self.xregs[dest] = if tmp.is_sign_negative() { 0 } else { 7 }
                                }

                                FpCategory::Normal => {
                                    self.xregs[dest] = if tmp.is_sign_negative() { 1 } else { 6 }
                                }

                                FpCategory::Subnormal => {
                                    self.xregs[dest] = if tmp.is_sign_negative() { 2 } else { 5 }
                                }

                                FpCategory::Zero => {
                                    self.xregs[dest] = if tmp.is_sign_negative() { 3 } else { 4 }
                                }

                                FpCategory::Nan => self.xregs[dest] = 9,
                            }
                        }

                        _ => return Err(RVException::IllegalInstruction(instruction)),
                    },

                    0b1010000 => match funct3 {
                        // FLE.S
                        0b000 => {
                            self.xregs[dest] = if self.fregs[source1] <= self.fregs[source2] {
                                1
                            } else {
                                0
                            }
                        }

                        // FLT.S
                        0b001 => {
                            self.xregs[dest] = if self.fregs[source1] < self.fregs[source2] {
                                1
                            } else {
                                0
                            }
                        }

                        // FEQ.S
                        0b010 => {
                            self.xregs[dest] = if self.fregs[source1] == self.fregs[source2] {
                                1
                            } else {
                                0
                            }
                        }

                        _ => return Err(RVException::IllegalInstruction(instruction)),
                    },

                    0b1101000 => match source2 {
                        // FCVT.S.W
                        0b00000 => {
                            self.fregs[dest] = self.xregs[source1] as i32 as f32 as f64;
                        }

                        // FCVT.S.WU
                        0b00001 => {
                            self.fregs[dest] = self.xregs[source1] as u32 as f32 as f64;
                        }

                        // FCVT.S.L
                        0b00010 => {
                            self.fregs[dest] = self.xregs[source1] as f32 as f64;
                        }

                        // FCVT.SU.L
                        0b00011 => {
                            self.fregs[dest] = self.xregs[source1] as u64 as f32 as f64;
                        }

                        _ => return Err(RVException::IllegalInstruction(instruction)),
                    },

                    // FMV.W.X
                    0b1111000 => {
                        self.fregs[dest] = f64::from_bits(self.xregs[source1] & 0xffffffff);
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // =====================================================================================

            // MEM-MISC
            0b0001111 => {
                match funct3 {
                    // FENCE
                    0b000 => {}

                    // Zifencei
                    // FENCE.I
                    0b001 => {}

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // SYSTEM
            0b1110011 => {
                let target_csr = ((_instruction >> 20) & 0xFFF) as usize;
                let funct12 = ((_instruction as i64) >> 20) as u64;

                match funct3 {
                    0b000 => {
                        match funct12 {
                            // ECALL
                            0b000000000000 => {
                                return Err(RVException::EnvironmentCall);
                            }

                            // EBREAK
                            0b000000000001 => {
                                return Err(RVException::Breakpoint);
                            }

                            _ => return Err(RVException::IllegalInstruction(instruction)),
                        }
                    }

                    // =============================================================================
                    // Zicsr
                    // CSRRW
                    0b001 => {
                        let tmp = self.csrs[target_csr];
                        self.csrs[target_csr] = self.xregs[source1];
                        self.xregs[dest] = tmp;
                    }

                    // CSRRS
                    0b010 => {
                        let tmp = self.csrs[target_csr];
                        self.csrs[target_csr] = tmp | self.xregs[source1];
                        self.xregs[dest] = tmp;
                    }

                    // CSRRC
                    0b011 => {
                        let tmp = self.csrs[target_csr];
                        self.csrs[target_csr] = tmp & !self.xregs[source1];
                        self.xregs[dest] = tmp;
                    }

                    // CSRRWI
                    0b101 => {
                        self.xregs[dest] = self.csrs[target_csr];
                        self.csrs[target_csr] = source1 as u64;
                    }

                    // CSRRSI
                    0b110 => {
                        let tmp = self.csrs[target_csr];
                        self.csrs[target_csr] = tmp | (source1 as u64);
                        self.xregs[dest] = tmp;
                    }

                    // CSRRCI
                    0b111 => {
                        let tmp = self.csrs[target_csr];
                        self.csrs[target_csr] = tmp & !(source1 as u64);
                        self.xregs[dest] = tmp;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            // RV64A
            // ATOMIC
            0b0101111 => {
                let funct5 = (funct7 & 0b1111100) >> 2;

                match (funct3, funct5) {
                    // LR.W
                    (0b010, 0b00010) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        self.xregs[dest] = self.read::<u32>(address)? as i32 as i64 as u64;
                        self.reservation_set.push(address);
                    }

                    // LR.D
                    (0b011, 0b00010) => {
                        let address = self.xregs[source1];
                        if address % 8 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        self.xregs[dest] = self.read::<u64>(address)?;
                        self.reservation_set.push(address);
                    }

                    // SC.W
                    (0b010, 0b00011) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        if self.reservation_set.contains(&address) {
                            self.reservation_set.retain(|&x| x != address);
                            self.write::<u32>(address, self.xregs[source2] as u32)?;
                            self.xregs[dest] = 0x00;
                        } else {
                            self.reservation_set.retain(|&x| x != address);
                            self.xregs[dest] = 0x01;
                        }
                    }

                    // SC.D
                    (0b011, 0b00011) => {
                        let address = self.xregs[source1];
                        if address % 8 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        if self.reservation_set.contains(&address) {
                            self.reservation_set.retain(|&x| x != address);
                            self.write::<u64>(address, self.xregs[source2])?;
                            self.xregs[dest] = 0x00;
                        } else {
                            self.reservation_set.retain(|&x| x != address);
                            self.xregs[dest] = 0x01;
                        }
                    }

                    // AMOSWAP.W
                    (0b010, 0b00001) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u32>(address)?;
                        self.write::<u32>(address, self.xregs[source2] as u32)?;
                        self.xregs[dest] = tmp as i32 as i64 as u64;
                    }

                    // AMOSWAP.D
                    (0b011, 0b00001) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u64>(address)?;
                        self.write::<u64>(address, self.xregs[source2])?;
                        self.xregs[dest] = tmp;
                    }

                    // AMOADD.W
                    (0b010, 0b00000) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u32>(address)?;
                        self.write::<u32>(address, (tmp as u64 + self.xregs[source2]) as u32)?;
                        self.xregs[dest] = tmp as i32 as i64 as u64;
                    }

                    // AMOADD.D
                    (0b011, 0b00000) => {
                        let address = self.xregs[source1];
                        if address % 8 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u64>(address)?;
                        self.write::<u64>(address, tmp + self.xregs[source2])?;
                        self.xregs[dest] = tmp;
                    }

                    // AMOXOR.W
                    (0b010, 0b00100) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<i32>(address)?;
                        self.write::<u32>(address, (tmp ^ (self.xregs[source2] as i32)) as u32)?;
                        self.xregs[dest] = tmp as i64 as u64;
                    }

                    // AMOXOR.D
                    (0b011, 0b00100) => {
                        let address = self.xregs[source1];
                        if address % 8 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u64>(address)?;
                        self.write::<u64>(address, tmp ^ self.xregs[source2])?;
                        self.xregs[dest] = tmp;
                    }

                    // AMOAND.W
                    (0b010, 0b01100) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<i32>(address)?;
                        self.write::<u32>(address, (tmp & (self.xregs[source2] as i32)) as u32)?;
                        self.xregs[dest] = tmp as i64 as u64;
                    }

                    // AMOAND.D
                    (0b011, 0b01100) => {
                        let address = self.xregs[source1];
                        if address % 8 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u64>(address)?;
                        self.write::<u64>(address, tmp & self.xregs[source2])?;
                        self.xregs[dest] = tmp;
                    }

                    // AMOOR.W
                    (0b010, 0b01000) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<i32>(address)?;
                        self.write::<u32>(address, (tmp | (self.xregs[source2] as i32)) as u32)?;
                        self.xregs[dest] = tmp as i64 as u64;
                    }

                    // AMOOR.D
                    (0b011, 0b01000) => {
                        let address = self.xregs[source1];
                        if address % 8 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u64>(address)?;
                        self.write::<u64>(address, tmp | self.xregs[source2])?;
                        self.xregs[dest] = tmp;
                    }

                    // AMOMIN.W
                    (0b010, 0b10000) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<i32>(address)?;
                        self.write::<u32>(
                            address,
                            cmp::min(tmp, self.xregs[source2] as i32) as u32,
                        )?;
                        self.xregs[dest] = tmp as i64 as u64;
                    }

                    // AMOMIN.D
                    (0b011, 0b10000) => {
                        let address = self.xregs[source1];
                        if address % 8 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<i64>(address)?;
                        self.write::<u64>(
                            address,
                            cmp::min(tmp, self.xregs[source2] as i64) as u64,
                        )?;
                        self.xregs[dest] = tmp as u64;
                    }

                    // AMOMAX.W
                    (0b010, 0b10100) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<i32>(address)?;
                        self.write::<u32>(
                            address,
                            cmp::max(tmp, self.xregs[source2] as i32) as u32,
                        )?;
                        self.xregs[dest] = tmp as i64 as u64;
                    }

                    // AMOMAX.D
                    (0b011, 0b10100) => {
                        let address = self.xregs[source1];
                        if address % 8 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<i64>(address)?;
                        self.write::<u64>(
                            address,
                            cmp::max(tmp, self.xregs[source2] as i64) as u64,
                        )?;
                        self.xregs[dest] = tmp as u64;
                    }

                    // AMOMINU.W
                    (0b010, 0b11000) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u32>(address)?;
                        self.write::<u32>(address, cmp::min(tmp, self.xregs[source2] as u32))?;
                        self.xregs[dest] = tmp as i64 as u64;
                    }

                    // AMOMINU.D
                    (0b011, 0b11000) => {
                        let address = self.xregs[source1];
                        if address % 8 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u64>(address)?;
                        self.write::<u64>(address, cmp::min(tmp, self.xregs[source2]))?;
                        self.xregs[dest] = tmp;
                    }

                    // AMOMAXU.W
                    (0b010, 11100) => {
                        let address = self.xregs[source1];
                        if address % 4 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u32>(address)?;
                        self.write::<u32>(address, cmp::max(tmp, self.xregs[source2] as u32))?;
                        self.xregs[dest] = tmp as i64 as u64;
                    }

                    // AMOMAXU.D
                    (0b011, 11100) => {
                        let address = self.xregs[source1];
                        if address % 8 != 0 {
                            return Err(RVException::LoadAddressMisaligned);
                        }

                        let tmp = self.read::<u64>(address)?;
                        self.write::<u64>(address, cmp::max(tmp, self.xregs[source2]))?;
                        self.xregs[dest] = tmp;
                    }

                    _ => return Err(RVException::IllegalInstruction(instruction)),
                }
            }

            _ => return Err(RVException::IllegalInstruction(instruction)),
        }

        Ok(())
    }
}
