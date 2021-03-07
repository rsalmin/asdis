use ux;
use std::fmt;
use std::hash::Hash;

use super::primitives::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InstructionCFmt { CR, CI, CSS, CIW, CL, CS, CA, CB, CB1, CB2, CJ, CCST }

pub enum InstructionC {
    CR  { func4 : Func4, rdrs1 : Register, rs2 : Register,  opcode : OpcodeC },
    CI   { func3 : Func3, imm : ux::u6 , rdrs1 : Register, opcode : OpcodeC },
    CSS { func3 : Func3, imm : ux::u6 , rs2 : Register, opcode : OpcodeC },
    CIW { func3 : Func3, imm : u8 , rdprime : Register3, opcode : OpcodeC },
    CL  { func3 : Func3, imm : ux::u5, rs1prime : Register3, rdprime : Register3, opcode : OpcodeC },
    CS  { func3 : Func3, imm : ux::u5, rs1prime : Register3, rs2prime : Register3, opcode : OpcodeC },
    CA  { func6 : Func6,  rdrs1prime : Register3,  func2 : Func2, rs2prime : Register3, opcode : OpcodeC },
    CB  { func3 : Func3, offset : u8, rs1prime : Register3, opcode : OpcodeC },
    CB1  { func3 : Func3, func2 : Func2, offset : u8, rs1prime : Register3, opcode : OpcodeC },
    CB2  { func3 : Func3, func2 : Func2, offset : u8, rs1prime : Register3, opcode : OpcodeC },
    CJ   { func3 : Func3, target : ux::u11,  opcode : OpcodeC },
    CCST { cst : u16 },
}
