use ux;
use std::fmt;
use std::hash::Hash;

use super::primitives::{Func3, Register};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct OpcodeC {
 pub v : ux::u2,
}

impl OpcodeC {
    pub fn new(v : u8) -> OpcodeC {
        OpcodeC { v : ux::u2::new(v) }
    }
}

impl fmt::Display for OpcodeC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Func2 {
 pub v : ux::u2,
}

impl Func2 {
    pub fn new(v : u8) -> Func2{
        Func2 { v : ux::u2::new(v) }
    }
}

impl fmt::Display for Func2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.v)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Func4 {
 pub v : ux::u4,
}

impl Func4 {
    pub fn new(v : u8) -> Func4 {
        Func4 { v : ux::u4::new(v) }
    }
}

impl fmt::Display for Func4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.v)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Func6 {
 pub v : ux::u6,
}

impl Func6 {
    pub fn new(v : u8) -> Func6 {
        Func6 { v : ux::u6::new(v) }
    }
}

impl fmt::Display for Func6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.v)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Register3 {
 pub v : ux::u3,
}

impl Register3 {
    pub fn new(v : u8) -> Register3 {
        Register3 { v : ux::u3::new(v) }
    }
}

impl fmt::Display for Register3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}", self.v)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InstructionCFmt { CR, CI, CSS, CIW, CL, CS, CA, CB, CJ }

pub enum InstructionC {
    CR  { func4 : Func4, rdrs1 : Register, rs2 : Register,  opcode : OpcodeC },
    CI   { func3 : Func3, imm : ux::u6 , rdrs1 : Register, opcode : OpcodeC },
    CSS { func3 : Func3, imm : ux::u6 , rs2 : Register, opcode : OpcodeC },
    CIW { func3 : Func3, imm : u8 , rdprime : Register3, opcode : OpcodeC },
    CL  { func3 : Func3, imm : ux::u5, rs1prime : Register3, rdprime : Register3, opcode : OpcodeC },
    CS  { func3 : Func3, imm : ux::u5, rs1prime : Register3, rs2prime : Register3, opcode : OpcodeC },
    CA  { func6 : Func6,  rdrs1prime : Register3,  func2 : Func2, rs2prime : Register3, opcode : OpcodeC },
    CB  { func3 : Func3, offset : u8, rs1prime : Register3, opcode : OpcodeC },
    CJ   { func3 : Func3, target : ux::u11,  opcode : OpcodeC },
}
