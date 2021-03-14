use ux;
use std::fmt;

use super::primitives::*;

pub enum Instruction32 {
    R { func7 : Func7, rs2 : Register, rs1 : Register, func3 : Func3, rd : Register, opcode : Opcode },
    I  { imm : ux::u12, rs1 : Register, func3 : Func3, rd : Register, opcode : Opcode },
    IC  { cst : ux::u12, rs1 : Register, func3 : Func3, rd : Register, opcode : Opcode },
    IF  { func7 : Func7, imm : ux::u5, rs1 : Register, func3 : Func3, rd : Register, opcode : Opcode },
    S { imm: ux::u12, rs2: Register, rs1: Register, func3 : Func3, opcode : Opcode },
    SB { imm: ux::u13, rs2: Register, rs1: Register, func3 : Func3, opcode : Opcode },
    U { imm: u32, rd : Register, opcode : Opcode },
    UJ { imm: ux::u21, rd : Register, opcode : Opcode },
}

impl fmt::Display for Instruction32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction32::R {func7, rs2, rs1, func3, rd, opcode:_} =>
                write!(f, "fmt: R, func7: {}, rs2: {}, rs1: {}, func3: {}, rd: {}", func7, rs2, rs1, func3, rd),
            Instruction32::I { imm, rs1, func3, rd, opcode:_} =>
                write!(f, "fmt: I, imm : {}, rs1 : {}, func3 : {}, rd : {}", imm, rs1, func3, rd),
            Instruction32::IC { cst, rs1, func3, rd, opcode:_} =>
                write!(f, "fmt: Ic, cst : {}, rs1 : {}, func3 : {}, rd : {}", cst, rs1, func3, rd),
            Instruction32::IF { func7, imm , rs1, func3, rd, opcode:_} =>
                write!(f, "fmt: If, func7 : {}, imm : {}, rs1 : {}, func3 : {}, rd : {}", func7, imm, rs1, func3, rd),
            Instruction32::S { imm, rs2, rs1, func3, opcode:_} =>
                write!(f, "fmt: S, imm: {:#06X}, rs2: {}, rs1: {}, func3: {}", imm, rs2, rs1, func3),
            Instruction32::SB { imm, rs2, rs1, func3, opcode:_} =>
                write!(f, "fmt: S, imm: {:#06X}, rs2: {}, rs1: {}, func3: {}", imm, rs2, rs1, func3),
            Instruction32::U { imm, rd, opcode:_} =>
                write!(f, "fmt: U, imm: {:#010X}, rd : {}", imm, rd),
            Instruction32::UJ { imm, rd, opcode:_} =>
                write!(f, "fmt: U, imm: {:#010X}, rd : {}", imm, rd),
        }
    }
}

impl fmt::Debug for Instruction32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction32::R {func7, rs2, rs1, func3, rd, opcode} =>
                write!(f, "op: {}, fmt: R, func7: {}, rs2: {}, rs1: {}, func3: {}, rd: {}", opcode, func7, rs2, rs1, func3, rd),
            Instruction32::I { imm, rs1, func3, rd, opcode} =>
                write!(f, "op: {}, fmt: I, imm : {}, rs1 : {}, func3 : {}, rd : {}", opcode, imm, rs1, func3, rd),
            Instruction32::IC { cst, rs1, func3, rd, opcode} =>
                write!(f, "op: {}, fmt: I, cst : {}, rs1 : {}, func3 : {}, rd : {}", opcode, cst, rs1, func3, rd),
            Instruction32::IF { func7, imm, rs1, func3, rd, opcode} =>
                write!(f, "op: {}, fmt: I, func7 : {}, imm : {}, rs1 : {}, func3 : {}, rd : {}", opcode, func7, imm, rs1, func3, rd),
            Instruction32::S { imm, rs2, rs1, func3, opcode} =>
                write!(f, "op: {}, fmt: S, imm: {:#06X}, rs2: {}, rs1: {}, func3: {}", opcode, imm, rs2, rs1, func3),
            Instruction32::SB { imm, rs2, rs1, func3, opcode} =>
                write!(f, "op: {}, fmt: S, imm: {:#06X}, rs2: {}, rs1: {}, func3: {}", opcode, imm, rs2, rs1, func3),
            Instruction32::U { imm, rd, opcode} =>
                write!(f, "op: {}, fmt: U, imm: {:#010X}, rd : {}", opcode, imm, rd),
            Instruction32::UJ { imm, rd, opcode} =>
                write!(f, "op: {}, fmt: U, imm: {:#010X}, rd : {}", opcode, imm, rd),
        }
    }
}


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Instruction32Fmt { R, I, IC, IF, S, SB, U, UJ }

