use std::collections::HashMap;
use std::hash::Hash;
use ux;

use crate::primitives::*;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Instruction32Signature {
    opcode: Opcode,
    func3 : Option<Func3>,
    func7 : Option<Func7>,
    cst12 : Option<ux::u12>,
}

struct OpcodeData {
    mnemonic : &'static str,
    fmt : Instruction32Fmt,
    signature : Instruction32Signature,
}

pub struct ISAHelper {
    opdata : Vec<OpcodeData>,
    pub op2fmt : HashMap::<Opcode, Instruction32Fmt>,
    signature2mnemonic : HashMap::<Instruction32Signature, &'static str>,
}

macro_rules! opdataR {
    ($name:literal, $op:expr, $func3:literal, $func7:literal) => {
        OpcodeData { mnemonic : $name
                                , fmt : Instruction32Fmt::R
                                , signature : Instruction32Signature {opcode : Opcode::new($op)
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func7 : Some(Func7::new($func7))
                                                                                         , cst12 : None }
                              }
    };
}

macro_rules! opdataI {
    ($name:literal, $op:expr, $func3:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : Instruction32Fmt::I
                                , signature : Instruction32Signature {opcode : Opcode::new($op)
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func7 : None
                                                                                         , cst12 : None }
                             }
    };
}

macro_rules! opdataI_f {
    ($name:literal, $op:expr, $func3:literal, $func7:literal) => {
        OpcodeData { mnemonic : $name
                                , fmt : Instruction32Fmt::IF
                                , signature : Instruction32Signature {opcode : Opcode::new($op)
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func7 : Some(Func7::new($func7))
                                                                                         , cst12 : None }
                             }
    };
}

macro_rules! opdataI_c {
    ($name:literal, $op:expr, $func3:literal, $cst12:literal) => {
        OpcodeData { mnemonic : $name
                                , fmt : Instruction32Fmt::IC
                                , signature : Instruction32Signature {opcode : Opcode::new($op)
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func7 : None
                                                                                         , cst12 : Some(ux::u12::new($cst12)) }
                             }
    };
}

macro_rules! opdataS {
    ($name:literal, $op:expr, $func3:literal) => {
        OpcodeData { mnemonic : $name
                                , fmt : Instruction32Fmt::S
                                , signature : Instruction32Signature {opcode : Opcode::new($op)
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func7 : None
                                                                                         , cst12 : None }
                              }
    };
}

macro_rules! opdataSB {
    ($name:literal, $op:expr, $func3:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : Instruction32Fmt::SB
                                , signature : Instruction32Signature {opcode : Opcode::new($op)
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func7 : None
                                                                                         , cst12 : None }
                              }
    };
}

macro_rules! opdataU {
    ($name:literal, $op:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : Instruction32Fmt::U
                                , signature : Instruction32Signature {opcode : Opcode::new($op)
                                                                                         , func3 : None
                                                                                         , func7 : None
                                                                                         , cst12 : None }
                              }
    };
}

macro_rules! opdataUJ {
    ($name:literal, $op:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : Instruction32Fmt::UJ
                                , signature : Instruction32Signature {opcode : Opcode::new($op)
                                                                                         , func3 : None
                                                                                         , func7 : None
                                                                                         , cst12 : None }
                             }
    };
}

fn i2signature(i : &Instruction32) -> Instruction32Signature {
        match i {
            Instruction32::R {func7, rs2:_, rs1:_, func3, rd:_, opcode} =>
                Instruction32Signature { opcode: *opcode, func3 : Some(*func3), func7 : Some(*func7), cst12 : None },
            Instruction32::I { imm:_, rs1:_, func3, rd:_, opcode} =>
                Instruction32Signature { opcode: *opcode, func3 : Some(*func3), func7 : None, cst12 : None },
            Instruction32::IC { cst, rs1:_, func3, rd:_, opcode} =>
                Instruction32Signature { opcode: *opcode, func3 : Some(*func3), func7 : None, cst12 : Some(*cst) },
            Instruction32::IF { func7, imm:_, rs1:_, func3, rd:_, opcode} =>
                Instruction32Signature { opcode: *opcode, func3 : Some(*func3), func7 : Some(*func7), cst12 : None },
            Instruction32::S { imm:_, rs2:_, rs1:_, func3, opcode} =>
                Instruction32Signature { opcode: *opcode, func3 : Some(*func3), func7 : None, cst12 : None },
            Instruction32::SB { imm:_, rs2:_, rs1:_, func3, opcode} =>
                Instruction32Signature { opcode: *opcode, func3 : Some(*func3), func7 : None, cst12 : None },
            Instruction32::U { imm:_, rd:_, opcode} =>
                Instruction32Signature { opcode: *opcode, func3 : None, func7 : None, cst12 : None },
            Instruction32::UJ { imm:_, rd:_, opcode} =>
                Instruction32Signature { opcode: *opcode, func3 : None, func7 : None, cst12 : None },
        }
}

impl ISAHelper {
    pub fn new() -> ISAHelper {
        let system_op = 0x73;

        let opdata = vec![ opdataI!("lb", 3, 0)
                                 , opdataI!("lh", 3, 1)
                                 , opdataI!("lw", 3, 2)
                                 , opdataI!("ld", 3, 3)
                                 , opdataI!("lbu", 3, 4)
                                 , opdataI!("lhu", 3, 5)
                                 , opdataI!("lwu", 3, 6)
                                 , opdataI!("fence", 0xF, 0)
                                 , opdataI!("fence.i", 0xF, 1)
                                 , opdataI!("addi", 0x13, 0)
                                 , opdataI!("addi", 0x13, 0)
                                 , opdataI!("slli", 0x13, 1)
                                 , opdataI!("slti", 0x13, 2)
                                 , opdataI!("sltiu", 0x13, 3)
                                 , opdataI!("xori", 0x13, 4)
                                 , opdataI_f!("srli", 0x13, 5, 0)
                                 , opdataI_f!("srai", 0x13, 5, 0x20)
                                 , opdataI!("orii", 0x13, 6)
                                 , opdataI!("andi", 0x13, 7)
                                 , opdataU!("auipc", 0x17)
                                 , opdataI!("addiw", 0x1B, 0)
                                 , opdataI!("slliw", 0x1B, 1)
                                 , opdataI_f!("srliw", 0x1B, 5, 0)
                                 , opdataI_f!("sraiw", 0x1B, 5, 0x20)
                                 , opdataS!("sb", 0x23, 0)
                                 , opdataS!("sh", 0x23, 1)
                                 , opdataS!("sw", 0x23, 2)
                                 , opdataS!("sd", 0x23, 3)
                                 , opdataR!("add", 0x33, 0, 0)
                                 , opdataR!("sub", 0x33, 0, 0x20)
                                 , opdataR!("sll", 0x33, 1, 0)
                                 , opdataR!("slt", 0x33, 2, 0)
                                 , opdataR!("sltu", 0x33, 3, 0)
                                 , opdataR!("xor", 0x33, 4, 0)
                                 , opdataR!("srl", 0x33, 5, 0)
                                 , opdataR!("sra", 0x33, 5, 0x20)
                                 , opdataR!("or", 0x33, 6, 0)
                                 , opdataR!("and", 0x33, 7, 0)
                                 , opdataU!("lui", 0x37)
                                 , opdataR!("addw", 0x3B, 0, 0)
                                 , opdataR!("subw", 0x3B, 0, 0x20)
                                 , opdataR!("sllw", 0x3B, 1, 0)
                                 , opdataR!("srlw", 0x3B, 5, 0)
                                 , opdataR!("sraw", 0x3B, 5, 0x20)
                                 , opdataSB!("beq", 0x63, 0)
                                 , opdataSB!("bne", 0x63, 1)
                                 , opdataSB!("blt", 0x63, 4)
                                 , opdataSB!("bge", 0x63, 5)
                                 , opdataSB!("bltu", 0x63, 6)
                                 , opdataSB!("bgeu", 0x63, 7)
                                 , opdataI!("jalr", 0x67, 0)
                                 , opdataUJ!("jal", 0x6F)
                                 , opdataI_c!("ecall", system_op, 0, 0)
                                 , opdataI_c!("ebreak", system_op, 0, 1)
                                 , opdataI!("CSRRW", system_op, 1)
                                 , opdataI!("CSRRS", system_op, 2)
                                 , opdataI!("CSRRC", system_op, 3)
                                 , opdataI!("CSRRWI", system_op, 4)
                                 , opdataI!("CSRRSI", system_op, 5)
                                 , opdataI!("CSRRCI", system_op, 6)
                                 ];

        println!("ISA has {} instructions", opdata.len());
        let mut op2fmt = HashMap::<Opcode, Instruction32Fmt>::new();
        let mut signature2mnemonic = HashMap::<Instruction32Signature, &str>::new();
        for elt in &opdata{
            op2fmt.insert(elt.signature.opcode, elt.fmt);
            signature2mnemonic.insert(elt.signature.clone(), elt.mnemonic);
        }
        ISAHelper {opdata, op2fmt, signature2mnemonic}
    }

    pub fn mnemonic(&self, i : &Instruction32) -> String {
        let sign = i2signature(i);
        match self.signature2mnemonic.get( &sign ) {
            Some ( s ) => String::from(*s),
            None => String::from("<unk>"),
        }
    }
}
