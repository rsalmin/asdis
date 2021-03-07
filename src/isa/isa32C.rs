use std::collections::HashMap;
use std::hash::Hash;
use ux;

use crate::primitives::*;

#[derive(PartialEq, Eq, Hash, Clone)]
struct InstructionSignature {
    opcode: OpcodeC,
    func2 : Option<Func2>,
    func3 : Option<Func3>,
    func4 : Option<Func4>,
    func6 : Option<Func6>,
    cst : Option<ux::u14>,
}

struct OpcodeData {
    mnemonic : &'static str,
    fmt : InstructionCFmt,
    signature : InstructionSignature,
}

pub struct ISAHelperC {
    opdata : Vec<OpcodeData>,
    pub op2fmt : HashMap::<InstructionSignature, InstructionFmt>,
    signature2mnemonic : HashMap::<InstructionSignature, &'static str>,
}

macro_rules! opdataCI {
    ($name:literal, $op:expr, $func3:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : InstructionCFmt::CI
                                , signature : InstructionSignature {opcode : OpcodeC::new($op)
                                                                                         , func2 : None
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func4 : None
                                                                                         , func6 : None
                                                                                         , cst : None }
                             }
    };
}

macro_rules! opdataCJ {
    ($name:literal, $op:expr, $func3:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : InstructionCFmt::CJ
                                , signature : InstructionSignature {opcode : OpcodeC::new($op)
                                                                                         , func2 : None
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func4 : None
                                                                                         , func6 : None
                                                                                         , cst : None }
                             }
    };
}

macro_rules! opdataCR {
    ($name:literal, $op:expr, $func4:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : InstructionCFmt::CR
                                , signature : InstructionSignature {opcode : OpcodeC::new($op)
                                                                                         , func2 : None
                                                                                         , func3 : None
                                                                                         , func4 : Some(Func4::new($func4))
                                                                                         , func6 : None
                                                                                         , cst : None }
                             }
    };
}

macro_rules! opdataCB {
    ($name:literal, $op:expr, $func3:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : InstructionCFmt::CB
                                , signature : InstructionSignature {opcode : OpcodeC::new($op)
                                                                                         , func2 : None
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func4 : None
                                                                                         , func6 : None
                                                                                         , cst : None }
                             }
    };
}

macro_rules! opdataCB1 {
    ($name:literal, $op:expr, $func3:expr, $func2:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : InstructionCFmt::CB1
                                , signature : InstructionSignature {opcode : OpcodeC::new($op)
                                                                                         , func2 : Some(Func2::new($func2))
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func4 : None
                                                                                         , func6 : None
                                                                                         , cst : None }
                             }
    };
}

macro_rules! opdataCB2 {
    ($name:literal, $op:expr, $func3:expr, $func2:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : InstructionCFmt::CB2
                                , signature : InstructionSignature {opcode : OpcodeC::new($op)
                                                                                         , func2 : Some(Func2::new($func2))
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func4 : None
                                                                                         , func6 : None
                                                                                         , cst : None }
                             }
    };
}

macro_rules! opdataCIW {
    ($name:literal, $op:expr, $func3:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : InstructionCFmt::CIW
                                , signature : InstructionSignature {opcode : OpcodeC::new($op)
                                                                                         , func2 : None
                                                                                         , func3 : Some(Func3::new($func3))
                                                                                         , func4 : None
                                                                                         , func6 : None
                                                                                         , cst : None }
                             }
    };
}

macro_rules! opdataCA {
    ($name:literal, $op:expr, $func6:expr, $func2:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : InstructionCFmt::CB2
                                , signature : InstructionSignature {opcode : OpcodeC::new($op)
                                                                                         , func2 : Some(Func2::new($func2))
                                                                                         , func3 : None
                                                                                         , func4 : None
                                                                                         , func6 : Some(Func6::new($func6))
                                                                                         , cst : None}
                             }
    };
}

macro_rules! opdataCCST {
    ($name:literal, $op:expr, $cst:expr) => {
        OpcodeData { mnemonic : $name
                                , fmt : InstructionCFmt::CCST
                                , signature : InstructionSignature {opcode : OpcodeC::new($op)
                                                                                         , func2 : None
                                                                                         , func3 : None
                                                                                         , func4 : None
                                                                                         , func6 : None
                                                                                         , cst : Some(ux::u14::new($cst))}
                             }
    };
}

//fn i2signature(i : &Instruction) -> InstructionSignature {
//        match i {
//            Instruction::R {func7, rs2:_, rs1:_, func3, rd:_, opcode} =>
//                InstructionSignature { opcode: *opcode, func3 : Some(*func3), func7 : Some(*func7), cst12 : None },
//            Instruction::I { imm:_, rs1:_, func3, rd:_, opcode} =>
//                InstructionSignature { opcode: *opcode, func3 : Some(*func3), func7 : None, cst12 : None },
//            Instruction::IC { cst, rs1:_, func3, rd:_, opcode} =>
//                InstructionSignature { opcode: *opcode, func3 : Some(*func3), func7 : None, cst12 : Some(*cst) },
//            Instruction::IF { func7, imm:_, rs1:_, func3, rd:_, opcode} =>
//                InstructionSignature { opcode: *opcode, func3 : Some(*func3), func7 : Some(*func7), cst12 : None },
//            Instruction::S { imm:_, rs2:_, rs1:_, func3, opcode} =>
//                InstructionSignature { opcode: *opcode, func3 : Some(*func3), func7 : None, cst12 : None },
//            Instruction::SB { imm:_, rs2:_, rs1:_, func3, opcode} =>
//                InstructionSignature { opcode: *opcode, func3 : Some(*func3), func7 : None, cst12 : None },
//            Instruction::U { imm:_, rd:_, opcode} =>
//                InstructionSignature { opcode: *opcode, func3 : None, func7 : None, cst12 : None },
//            Instruction::UJ { imm:_, rd:_, opcode} =>
//                InstructionSignature { opcode: *opcode, func3 : None, func7 : None, cst12 : None },
//        }
//}

impl ISAHelperC {
    pub fn new() -> ISAHelperC {
        let c0 = 0u8;
        let c1 = 1u8;
        let c2 = 2u8;

        let opdata = vec![
                                // RV32DC opdataCI!("c.fldsp", c2, 1),
                                // RV128 opdataCI!("c.lqsp", c2, 1),
                                opdataCI!("c.lwsp", c2, 2),
                                // RV32FC  opdataCI!("c.flwsp", c2, 3),
                                // RV64/128 opdataCI!("c.ldsp", c2, 3),

                                // RV32DC opdataCI!("c.fsdsp", c2, 5),
                                // RV128 opdataCI!("c.sqsp", c2, 5),
                                opdataCI!("c.swsp", c2, 6),
                                // RV32FC  opdataCI!("c.fswsp", c2, 7),
                                // RV64/128 opdataCI!("c.sdsp", c2, 7),

                                // RV32DC opdataCI!("c.fld", c0, 1),
                                // RV128 opdataCI!("c.lq", c0, 1),
                                opdataCI!("c.lw", c0, 2),
                                // RV32FC opdataCI!("c.flw", c0, 3),
                                // RV64/128 opdataCI!("c.ld", c0, 3),

                                // RV32DC opdataCI!("c.fsd", c0, 5),
                                // RV128 opdataCI!("c.sq", c0, 5),
                                opdataCI!("c.sw", c0, 6),
                                // RV32FC opdataCI!("c.fsw", c0, 7),
                                // RV64/128 opdataCI!("c.sd", c0, 7),

                                opdataCJ!("c.j", c1, 5),
                                opdataCJ!("c.jal", c1, 1),

                                opdataCR!("c.jr", c2, 8),
                                opdataCR!("c.jalr", c2, 9),

                                opdataCB!("c.beqz", c1, 6),
                                opdataCB!("c.bnez", c1, 7),

                                opdataCI!("c.li", c1, 2),
                                opdataCI!("c.lui", c1, 3), //nzimm != 0
                                opdataCI!("c.addi", c1, 0), //nzimmm  == 0
                                //RV64C/RV128C opdataCI!("c.addiw", c1, 1),
                                opdataCI!("c.addi16sp", c1, 3), //nzimmm == 0

                                opdataCIW!("c.addi4spn", c0, 0),
                                opdataCI!("c.slli", c2, 0),

                                opdataCB1!("c.srli", c1, 4, 0),  // !!!
                                opdataCB1!("c.srai", c1, 4, 1), // !!!

                                opdataCB2!("c.andi", c1, 4, 2 ),
                                opdataCR!("c.mv", c2, 8),
                                opdataCR!("c.add", c2, 9),

                                opdataCA!("c.and", c1, 0x23, 3),
                                opdataCA!("c.or", c1, 0x23, 2 ),
                                opdataCA!("c.xor", c1, 0x23 , 1),
                                opdataCA!("c.sub", c1, 0x23, 0),
                                opdataCA!("c.addw", c1, 0x27, 1),
                                opdataCA!("c.subw", c1, 0x27, 0),

                                opdataCCST!("c.nop", c1, 0),
                                opdataCCST!("c.ebreak", c2, 0x2800),
                                 ];

        println!("ISA has {} instructions", opdata.len());
        let mut op2fmt = HashMap::<InstructionSignature, InstructionFmt>::new();
        let mut signature2mnemonic = HashMap::<InstructionSignature, &str>::new();
        for elt in &opdata{
            op2fmt.insert(elt.signature.clone(), elt.fmt);
            signature2mnemonic.insert(elt.signature.clone(), elt.mnemonic);
        }
        ISAHelperC {opdata} //, op2fmt, signature2mnemonic}
    }

//    pub fn mnemonic(&self, i : &Instruction) -> String {
//        let sign = i2signature(i);
//        match self.signature2mnemonic.get( &sign ) {
//            Some ( s ) => String::from(*s),
//            None => String::from("<unk>"),
//        }
//    }
}
