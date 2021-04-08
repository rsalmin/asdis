use crate::primitives::*;
use crate::isa::isa::*;
use std::collections::HashMap;

pub type ISARV32I = ISA<RV32Type>;

///helper to show register
fn show_register(v : u32) -> String
{
    format!("r{}", v)
}

impl ISARV32I {
    pub fn new() -> ISARV32I {

        let list = vec! [
            riscv_dis::instruction32!("<illegal>", 00000000000000000000000000000000),
       ];

       let mut show_dict = HashMap::new();
       show_dict.insert(String::from("rd"), show_register as ShowFun::<RV32Type>);
       show_dict.insert(String::from("rs1"), show_register as ShowFun::<RV32Type>);
       show_dict.insert(String::from("rs2"), show_register as ShowFun::<RV32Type>);

        ISARV32I { list, show_dict }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        let isa = ISARV32I::new();
        println!("{:?}", isa);
        assert!(true);
    }

    #[test]
    fn i1() {
        let i1 = riscv_dis::instruction32!("c.nop", 000, imm[5], 00000, imm[4:0], 01);
        let list = vec![ Item::Bits { len : 3, val : 0 },
                                 Item::Ident { name : String::from("imm"), bitspec : vec![5] },
                                 Item::Bits { len : 5, val : 0 },
                                 Item::Ident { name : String::from("imm"), bitspec : vec![4,3,2,1,0] },
                                 Item::Bits { len : 2, val : 1 },];
        let bin = BinaryInstruction { list };
        let text = TextInstruction { list : vec![ TextInstructionPart::Text(String::from("c.nop")) ] };
        assert_eq!(i1.text, text);
        assert_eq!(i1.bin, bin);
    }

}
