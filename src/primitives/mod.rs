use ux;
use std::fmt;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Opcode {
 pub v : ux::u7,
}

impl Opcode {
    pub fn new(v : u8) -> Opcode {
        Opcode { v : ux::u7::new(v) }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#04X}", self.v)
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Func3 {
 pub v : ux::u3,
}

impl Func3 {
    pub fn new(v : u8) -> Func3 {
        Func3 { v : ux::u3::new(v) }
    }
}

impl fmt::Display for Func3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.v)
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Register {
 pub v : ux::u5,
}

impl Register {
    pub fn new(v : u8) -> Register {
        Register { v : ux::u5::new(v) }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}", self.v)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Func7 {
 pub v : ux::u7,
}

impl Func7 {
    pub fn new(v : u8) -> Func7 {
        Func7 { v : ux::u7::new(v) }
    }
}

impl fmt::Display for Func7 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#04X}", self.v)
    }
}

pub enum ImmFunc {
    Imm(ux::u12),
    FuncImm { func7 : Func7, imm : ux::u5 },
    Const(ux::u12),
}

impl ImmFunc {
    pub fn from_imm(v : u16) -> ImmFunc {
        ImmFunc::Imm( ux::u12::new(v) )
    }
    pub fn from_func_imm( func7 : Func7, v : u8) -> ImmFunc {
        ImmFunc::FuncImm { func7, imm : ux::u5::new(v) }
    }
    pub fn from_const( v : u16 ) -> ImmFunc {
        ImmFunc::Const( ux::u12::new(v) )
    }
    pub fn cst12(&self) -> Option<ux::u12> {
        match &self {
            ImmFunc::Const( v ) => Some(*v),
            _ => None,
        }
    }
    pub fn func7(&self) -> Option<Func7> {
        match &self {
            ImmFunc::FuncImm { func7, .. } => Some(*func7),
            _ => None,
        }
    }
}

impl fmt::Display for ImmFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         match &self  {
             ImmFunc::Imm( imm ) => write!(f, "imm: {}",  imm),
             ImmFunc::FuncImm { func7, imm } => write!(f, "func7: {}, imm: {:#04X}", func7, imm),
             ImmFunc::Const( imm ) => write!(f, "cst: {}",  imm),
         }
    }
}

pub enum Instruction {
    R { func7 : Func7, rs2 : Register, rs1 : Register, func3 : Func3, rd : Register, opcode : Opcode },
    I  { immfunc : ImmFunc, rs1 : Register, func3 : Func3, rd : Register, opcode : Opcode },
    S { imm: ux::u12, rs2: Register, rs1: Register, func3 : Func3, opcode : Opcode },
    SB { imm: ux::u13, rs2: Register, rs1: Register, func3 : Func3, opcode : Opcode },
    U { imm: u32, rd : Register, opcode : Opcode },
    UJ { imm: ux::u21, rd : Register, opcode : Opcode },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::R {func7, rs2, rs1, func3, rd, opcode:_} =>
                write!(f, "fmt: R, func7: {}, rs2: {}, rs1: {}, func3: {}, rd: {}", func7, rs2, rs1, func3, rd),
            Instruction::I { immfunc, rs1, func3, rd, opcode:_} =>
                write!(f, "fmt: I, {}, rs1 : {}, func3 : {}, rd : {}", immfunc, rs1, func3, rd),
            Instruction::S { imm, rs2, rs1, func3, opcode:_} =>
                write!(f, "fmt: S, imm: {:#06X}, rs2: {}, rs1: {}, func3: {}", imm, rs2, rs1, func3),
            Instruction::SB { imm, rs2, rs1, func3, opcode:_} =>
                write!(f, "fmt: S, imm: {:#06X}, rs2: {}, rs1: {}, func3: {}", imm, rs2, rs1, func3),
            Instruction::U { imm, rd, opcode:_} =>
                write!(f, "fmt: U, imm: {:#010X}, rd : {}", imm, rd),
            Instruction::UJ { imm, rd, opcode:_} =>
                write!(f, "fmt: U, imm: {:#010X}, rd : {}", imm, rd),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::R {func7, rs2, rs1, func3, rd, opcode} =>
                write!(f, "op: {}, fmt: R, func7: {}, rs2: {}, rs1: {}, func3: {}, rd: {}", opcode, func7, rs2, rs1, func3, rd),
            Instruction::I { immfunc, rs1, func3, rd, opcode} =>
                write!(f, "op: {}, fmt: I, {}, rs1 : {}, func3 : {}, rd : {}", opcode, immfunc, rs1, func3, rd),
            Instruction::S { imm, rs2, rs1, func3, opcode} =>
                write!(f, "op: {}, fmt: S, imm: {:#06X}, rs2: {}, rs1: {}, func3: {}", opcode, imm, rs2, rs1, func3),
            Instruction::SB { imm, rs2, rs1, func3, opcode} =>
                write!(f, "op: {}, fmt: S, imm: {:#06X}, rs2: {}, rs1: {}, func3: {}", opcode, imm, rs2, rs1, func3),
            Instruction::U { imm, rd, opcode} =>
                write!(f, "op: {}, fmt: U, imm: {:#010X}, rd : {}", opcode, imm, rd),
            Instruction::UJ { imm, rd, opcode} =>
                write!(f, "op: {}, fmt: U, imm: {:#010X}, rd : {}", opcode, imm, rd),
        }
    }
}


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InstructionFmt { R, I, S, SB, U, UJ }

