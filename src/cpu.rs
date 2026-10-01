use std::ops::{Index, IndexMut};
use crate::{memory_bus::MemoryBus};

pub(crate) struct Cpu {
    registers: Registers,
    bus: MemoryBus,
}

impl Cpu {

    pub fn new() -> Self {
       Cpu { registers: Registers::new(), bus: MemoryBus::new() } 
    }

    pub fn step(&mut self) {}

    fn fetch8(&mut self) -> u8 {
        let op_code = self.bus.read(self.registers.pc);
        self.registers.pc += 1;
        op_code
    }

    fn fetch16(&mut self) -> u16 {
        let low: u16 = self.fetch8() as u16;
        let high: u16 = self.fetch8() as u16;

        (high << 8) | low
    }
    
    // TODO - there might be a better aproach without as much boilerplate
    fn read_r8(&mut self, r8: Reg8) -> u8 {
        use Reg8::*;

        match r8 {
            B => self.registers.b,
            C => self.registers.c,
            D => self.registers.d,
            E => self.registers.e,
            H => self.registers.h,
            L => self.registers.l,
            HLMem => self.bus.read(self.registers.hl()),
            A => self.registers.a,
        }
    }

    fn write_r8(&mut self, r8: Reg8, val: u8) {
        use Reg8::*;

        match r8 {
            B => self.registers.b = val,
            C => self.registers.c = val,
            D => self.registers.d = val,
            E => self.registers.e = val,
            H => self.registers.h = val,
            L => self.registers.l = val,
            HLMem => self.bus.write(self.registers.hl(), val),
            A => self.registers.a = val,
        };
    }

    fn read_r16(&mut self, r16: Reg16) -> u16 {
        use Reg16::*;

        match r16 {
            BC => self.registers.bc(),
            DE => self.registers.de(),
            HL => self.registers.hl(),
            SP => self.registers.sp,
            AF => self.registers.af(),
            HLD => self.registers.hl(), //TODO
            HLI => self.registers.hl(), //TODO
        }
    }

    fn write_r16(&mut self, r16: Reg16, val: u16) {
        use Reg16::*;

        match r16 {
            BC => self.registers.set_bc(val),
            DE => self.registers.set_de(val),
            HL => self.registers.set_hl(val),
            SP => self.registers.sp = val,
            AF => self.registers.set_af(val),
            HLD => self.registers.set_hl(val), //TODO
            HLI => self.registers.set_hl(val), //TODO
        };
    } 

    fn decode(instruction: u8) -> Instruction {
        let fields = OpFields { 
            x: instruction >> 6,
            y: (instruction >> 3) & 0b0111,
            z: (instruction & 0b0111), 
            p: (instruction >> 4) & 0b0011, 
            q: (instruction >> 3) & 0b0001, 
        };

        use Instruction::*;

        let reg8_y = Reg8::from_index(fields.y);
        let reg8_z = Reg8::from_index(fields.z);
        let reg16_reg = Reg16::from_index(fields.p, Reg16Mode::Reg);
        let reg16_mem = Reg16::from_index(fields.p, Reg16Mode::Mem);
        let cond = Cond::from_index(fields.y >> 1);

        match (fields.x, fields.y, fields.z) {
            // Block 0
            // (0,0,0) => Nop,
            // (0,0x1,0x0) => LdImm16Sp,
            (0,_,0x0) => {
                match fields.y {
                    0x0 => Nop,
                    0x1 => LdImm16Sp,
                    0x2 => Stop,
                    0x3 => JrImm,
                    _ => JrCondImm { cond: cond },
                }
            }
            (0,_,0x1) => {
                match fields.q {
                    0 => LdR16Imm { dst: reg16_reg },
                    1 => AddHlR16 { operand: reg16_reg },
                    _ => Nop,
                }
            } 
            (0,_,0x2) => {
                match fields.q {
                    0 => LdMemA { dst_addr: reg16_mem },
                    1 => LdAMem { src_addr: reg16_mem },
                    _ => Nop,
                }
            }, 
            (0,_,0x3) => {
                match fields.q {
                    0 => IncR16 { operand: reg16_reg },
                    1 => DecR16 { operand: reg16_reg },
                    _ => Nop,
                }
            }
            (0,_,0x4) => IncR { operand: reg8_y },
            (0,_,0x5) => DecR { operand: reg8_y },
            (0,_,0x6) => LdRImm { dest: reg8_y },
            (0,_,0x7) => AccFlag { tag: fields.y },

            // Block 1
            (1,_,_) => LdRR { dest: reg8_y, src: reg8_z },

            // Undefined - TODO
            (_,_,_) => Nop,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        use Instruction::*;

        // match instruction {
        //     Nop => (),
        //     LdImm16Sp => (),
        //     LdR16Imm { dst } => (),
        //     LdMemA { dst_addr } => (),
        //     LdAMem { src_addr } => (),
        // }
    }

}

struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}


impl Registers {
    pub fn new() -> Self {
        // Hardcoded values after skipping boot ROM
        // TODO - perhaps add boot ROM compatability later
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            sp: 0xfffe,
            pc: 0x0100,
        }
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val as u8) & 0xF0;
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }
}

// Logic based on the contents from https://www.z80.info/decoding.htm
struct OpFields {
    x: u8,
    y: u8,
    z: u8,
    p: u8,
    q: u8,
}

#[derive(Clone, Copy)]
enum Reg8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HLMem,
    A,
}

impl Reg8 {
    fn from_index(index: u8) -> Reg8 {
        [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::HLMem, Reg8::A][index as usize]
    }
}

enum Reg16Mode {
    Reg,
    Stk,
    Mem,
}

#[derive(Clone, Copy)]
enum Reg16 {
    BC,
    DE,
    HL,
    SP,
    AF,
    HLD,
    HLI,
}

impl Reg16 {
    fn from_index(index: u8, mode: Reg16Mode) -> Reg16 {
        match mode {
            Reg16Mode::Reg => [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::SP][index as usize],
            Reg16Mode::Stk => [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::AF][index as usize],
            Reg16Mode::Mem => [Reg16::BC, Reg16::DE, Reg16::HLI, Reg16::HLD][index as usize],
        }
    }
}

#[derive(Clone, Copy)]
enum Cond {
    NZ,
    N,
    NC,
    C,
}

impl Cond {
    fn from_index(index: u8) -> Cond {
        [Cond::NZ, Cond::N, Cond::NC, Cond::C][index as usize]
    }
}

enum Instruction {
    Nop,
    // Loads     
    // (Block 0)
    LdR16Imm{dst: Reg16},
    LdMemA{dst_addr: Reg16},
    LdAMem{src_addr: Reg16},
    LdImm16Sp,

    IncR16{operand: Reg16},
    DecR16{operand: Reg16},
    AddHlR16{operand: Reg16},

    IncR{operand: Reg8},
    DecR{operand: Reg8},

    LdRImm{dest: Reg8},

    AccFlag{tag: u8},
    JrImm,
    JrCondImm{cond: Cond},
    Stop,
    
    // (Block 1)
    LdRR{dest: Reg8, src: Reg8}
}
