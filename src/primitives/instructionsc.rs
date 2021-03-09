use ux;
use std::fmt;
use std::hash::Hash;

use super::primitives::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InstructionC {
    ADDI4SPN { imm : u8, rd : Register3 },
    LW { imm : ux::u5, rs1 : Register3, rd : Register3 },
    SW { imm : ux::u5, rs1 : Register3, rs2 : Register3 },
    NOP,
    ADDI { imm : ux::u5, rs1rd : Register },
    JAL { imm : ux::u11 },
    LI { imm : ux::u5, rd : Register },
    ADDI16SP { imm : ux::u5 },
    LUI { imm : ux::u5, rd : Register },
    SRLI { imm : ux::u5, rs1rd : Register3 },
    SRAI { imm : ux::u5, rs1rd : Register3 },
    ANDI { imm : ux::u5, rs1rd : Register3 },
    SUB { rs1rd : Register3, rs2 : Register3 },
    XOR { rs1rd : Register3, rs2 : Register3 },
    OR  { rs1rd : Register3, rs2 : Register3 },
    AND { rs1rd : Register3, rs2 : Register3 },
    J { imm : ux::u11 },
    BEQZ { imm : u8, rs1 : Register3 },
    BNEZ { imm : u8, rs1 : Register3 },
    SLLI { imm : ux::u5, rs1rd : Register },
    LWSP { imm : ux::u5, rd : Register },
    JR { rs1 : Register },
    MV { rd : Register, rs2 : Register },
    EBREAK,
    JALR { rs1 : Register },
    ADD { rs1rd : Register, rs2 : Register },
    SWSP { imm : ux::u6, rs2 : Register },
}

//impl fmt::Display for Instruction {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        match self {
//                ADDI4SPN { imm, rd } => write!(f, "c.addi4spn"),
//                LW { imm, rs1, rd } => write!(f, "c.lw"),
//                SW { imm : ux::u5, rs1 : Register3, rs2 : Register3 },
//                NOP,
//                ADDI { imm : ux::u5, rs1rd : Register },
//                JAL { imm : ux::u11 },
//                LI { imm : ux::u5, rd : Register },
//                ADDI16SP { imm : ux::u5 },
//                LUI { imm : ux::u5, rd : Register },
//                SRLI { imm : ux::u5, rs1rd : Register3 },
//                SRAI { imm : ux::u5, rs1rd : Register3 },
//                ANDI { imm : ux::u5, rs1rd : Register3 },
//                SUB { rs1rd : Register3, rs2 : Register3 },
//                XOR { rs1rd : Register3, rs2 : Register3 },
//                OR  { rs1rd : Register3, rs2 : Register3 },
//                AND { rs1rd : Register3, rs2 : Register3 },
//                J { imm : ux::u11 },
//                BEQZ { imm : u8, rs1 : Register3 },
//                BNEZ { imm : u8, rs1 : Register3 },
//                SLLI { imm : ux::u5, rs1rd : Register },
//                LWSP { imm : ux::u5, rd : Register },
//                JR { rs1 : Register },
//                MV { rd : Register, rs2 : Register },
//                EBREAK,
//                JALR { rs1 : Register },
//                ADD { rs1rd : Register, rs2 : Register },
//                SWSP { imm : ux::u6, rs2 : Register },
//        }
//    }
//}