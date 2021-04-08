use crate::primitives::*;
use crate::isa::isa::*;
use std::collections::HashMap;

pub type ISARV32C = ISA<CompactType>;

///helper to show register
fn show_register(v : u32) -> String
{
    format!("r{}", v)
}

impl ISARV32C {
    pub fn new() -> ISARV32C {

        let list = vec! [
            riscv_dis::instruction16!("c.addi4spn rd, imm", 000, imm[5:4|9:6|2|3], rd[2:0], 00),
            riscv_dis::instruction16!("c.lw rd, rs1, imm", 010, imm[5:3], rs1[2:0], imm[2|6], rd[2:0], 00),
            riscv_dis::instruction16!("c.sw rs1, imm (rs2)", 110, imm[5:3], rs1[2:0], imm[2|6], rs2[2:0], 00),
            riscv_dis::instruction16!("c.nop", 000, imm[5], 00000, imm[4:0], 01),
            riscv_dis::instruction16!("c.addi rd, imm", 000, imm[5], rd[4:0], imm[4:0], 01),
            riscv_dis::instruction16!("c.jal imm", 001, imm[11|4|9:8|10|6|7|3:1|5], 01),
            riscv_dis::instruction16!("c.li rd, imm", 010, imm[5], rd[4:0], imm[4:0], 01),
            riscv_dis::instruction16!("c.addi16sp imm", 011, imm[9], 00010, imm[4|6|8:7|5], 01),
            riscv_dis::instruction16!("c.lui rd, imm", 011, imm[17], rd[4:0], imm[16:12], 01),
            riscv_dis::instruction16!("c.srli rd, imm", 100100, rd[2:0], imm[4:0], 01),
            riscv_dis::instruction16!("c.srai rd, imm", 100101, rd[2:0], imm[4:0], 01),
            riscv_dis::instruction16!("c.andi rd, imm", 100, imm[5], 10, rd[2:0], imm[4:0], 01),
            riscv_dis::instruction16!("c.sub rd, rs2", 100011, rd[2:0], 00, rs2[2:0], 01),
            riscv_dis::instruction16!("c.xor rd, rs2", 100011, rd[2:0], 01, rs2[2:0], 01),
            riscv_dis::instruction16!("c.or rd, rs2", 100011, rd[2:0], 10, rs2[2:0], 01),
            riscv_dis::instruction16!("c.and rd, rs2", 100011, rd[2:0], 11, rs2[2:0], 01),
            riscv_dis::instruction16!("c.j imm", 101, imm[11|4|9:8|10|6|7|3:1|5], 01),
            riscv_dis::instruction16!("c.beqz rs1, imm", 110, imm[8|4:3], rs1[2:0], imm[7:6|2:1|5], 01),
            riscv_dis::instruction16!("c.bnez rs1, imm", 111, imm[8|4:3], rs1[2:0], imm[7:6|2:1|5], 01),

            riscv_dis::instruction16!("c.slli rd, imm", 000, imm[5], rd[4:0], imm[4:0], 10),
            riscv_dis::instruction16!("c.lwsp rd, imm", 010, imm[5], rd[4:0], imm[4:2|7:6], 10),
            riscv_dis::instruction16!("c.jr rs1", 1000, rs1[4:0], 0000010),
            riscv_dis::instruction16!("c.mv rd, rs2", 1000, rd[4:0], rs2[4:0], 10),
            riscv_dis::instruction16!("c.ebreak", 1001000000000010),
            riscv_dis::instruction16!("c.jalr rs1", 1001, rs1[4:0], 0000010),
            riscv_dis::instruction16!("c.add rd, rs2", 1001, rd[4:0], rs2[4:0], 10),
            riscv_dis::instruction16!("c.swsp rs2, imm", 110, imm[5:2|7:6], rs2[4:0], 10),

            riscv_dis::instruction16!("<illegal>", 0000000000000000),
       ];

       let mut show_dict = HashMap::new();
       show_dict.insert(String::from("rd"), show_register as ShowFun::<CompactType>);
       show_dict.insert(String::from("rs1"), show_register as ShowFun::<CompactType>);
       show_dict.insert(String::from("rs2"), show_register as ShowFun::<CompactType>);

        ISARV32C { list, show_dict }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        let isa = ISARV32C::new();
        println!("{:?}", isa);
        assert!(true);
    }

    #[test]
    fn i1() {
        let i1 = riscv_dis::instruction16!("c.nop", 000, imm[5], 00000, imm[4:0], 01);
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
