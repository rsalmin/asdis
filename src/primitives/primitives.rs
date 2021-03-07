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

pub enum Instruction {
    R { func7 : Func7, rs2 : Register, rs1 : Register, func3 : Func3, rd : Register, opcode : Opcode },
    I  { imm : ux::u12, rs1 : Register, func3 : Func3, rd : Register, opcode : Opcode },
    IC  { cst : ux::u12, rs1 : Register, func3 : Func3, rd : Register, opcode : Opcode },
    IF  { func7 : Func7, imm : ux::u5, rs1 : Register, func3 : Func3, rd : Register, opcode : Opcode },
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
            Instruction::I { imm, rs1, func3, rd, opcode:_} =>
                write!(f, "fmt: I, imm : {}, rs1 : {}, func3 : {}, rd : {}", imm, rs1, func3, rd),
            Instruction::IC { cst, rs1, func3, rd, opcode:_} =>
                write!(f, "fmt: Ic, cst : {}, rs1 : {}, func3 : {}, rd : {}", cst, rs1, func3, rd),
            Instruction::IF { func7, imm , rs1, func3, rd, opcode:_} =>
                write!(f, "fmt: If, func7 : {}, imm : {}, rs1 : {}, func3 : {}, rd : {}", func7, imm, rs1, func3, rd),
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
            Instruction::I { imm, rs1, func3, rd, opcode} =>
                write!(f, "op: {}, fmt: I, imm : {}, rs1 : {}, func3 : {}, rd : {}", opcode, imm, rs1, func3, rd),
            Instruction::IC { cst, rs1, func3, rd, opcode} =>
                write!(f, "op: {}, fmt: I, cst : {}, rs1 : {}, func3 : {}, rd : {}", opcode, cst, rs1, func3, rd),
            Instruction::IF { func7, imm, rs1, func3, rd, opcode} =>
                write!(f, "op: {}, fmt: I, func7 : {}, imm : {}, rs1 : {}, func3 : {}, rd : {}", opcode, func7, imm, rs1, func3, rd),
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
pub enum InstructionFmt { R, I, IC, IF, S, SB, U, UJ }

