use crate::primitives::*;
use crate::isa::isa::*;
use std::collections::HashMap;

pub type ISARV32IM = ISA<RV32Type>;

///helper to show register
fn show_register(v : u32) -> String
{
    format!("r{}", v)
}

impl ISARV32IM {
    pub fn new() -> ISARV32IM {

        let list = vec! [
            // RV32I
            riscv_dis::instruction32!("addi rd, rs1, imm", imm[11:0], rs1[4:0], 000 ,rd[4:0], 0010011),
            riscv_dis::instruction32!("andi rd, rs1, imm", imm[11:0], rs1[4:0], 111 ,rd[4:0], 0010011),
            riscv_dis::instruction32!("slti rd, rs1, imm", imm[11:0], rs1[4:0], 010 ,rd[4:0], 0010011),
            riscv_dis::instruction32!("sltiu rd, rs1, imm", imm[11:0], rs1[4:0], 011 ,rd[4:0], 0010011),
            riscv_dis::instruction32!("ori rd, rs1, imm", imm[11:0], rs1[4:0], 110 ,rd[4:0], 0010011),
            riscv_dis::instruction32!("xori rd, rs1, imm", imm[11:0], rs1[4:0], 100 ,rd[4:0], 0010011),

            riscv_dis::instruction32!("slli rd, rs1, imm", 0000000, imm[4:0], rs1[4:0], 001 ,rd[4:0], 0010011),
            riscv_dis::instruction32!("srli rd, rs1, imm", 0000000, imm[4:0], rs1[4:0], 101 ,rd[4:0], 0010011),
            riscv_dis::instruction32!("srai rd, rs1, imm", 0100000, imm[4:0], rs1[4:0], 101 ,rd[4:0], 0010011),

            riscv_dis::instruction32!("lui rd, imm", imm[31:12], rd[4:0], 0110111),
            riscv_dis::instruction32!("auipc rd, imm", imm[31:12], rd[4:0], 0010111),

            riscv_dis::instruction32!("add rd, rs1, rs2", 0000000, rs2[4:0], rs1[4:0], 000 ,rd[4:0], 0110011),
            riscv_dis::instruction32!("slt rd, rs1, rs2", 0000000, rs2[4:0], rs1[4:0], 010 ,rd[4:0], 0110011),
            riscv_dis::instruction32!("sltu rd, rs1, rs2", 0000000, rs2[4:0], rs1[4:0], 011 ,rd[4:0], 0110011),
            riscv_dis::instruction32!("and rd, rs1, rs2", 0000000, rs2[4:0], rs1[4:0], 111 ,rd[4:0], 0110011),
            riscv_dis::instruction32!("or rd, rs1, rs2", 0000000, rs2[4:0], rs1[4:0], 110 ,rd[4:0], 0110011),
            riscv_dis::instruction32!("xor rd, rs1, rs2", 0000000, rs2[4:0], rs1[4:0], 100 ,rd[4:0], 0110011),
            riscv_dis::instruction32!("sll rd, rs1, rs2", 0000000, rs2[4:0], rs1[4:0], 001 ,rd[4:0], 0110011),
            riscv_dis::instruction32!("srl rd, rs1, rs2", 0000000, rs2[4:0], rs1[4:0], 101 ,rd[4:0], 0110011),
            riscv_dis::instruction32!("sub rd, rs1, rs2", 0100000, rs2[4:0], rs1[4:0], 000 ,rd[4:0], 0110011),
            riscv_dis::instruction32!("sra rd, rs1, rs2", 0100000, rs2[4:0], rs1[4:0], 101 ,rd[4:0], 0110011),

            riscv_dis::instruction32!("nop", 00000000000000000000000000010011),

            riscv_dis::instruction32!("jal", imm[10|10:1|11|19:12] ,rd[4:0], 1101111),
            riscv_dis::instruction32!("jalr", imm[11:0],rs1[4:0], 000, rd[4:0], 1100111),

            riscv_dis::instruction32!("beq rs1, rs2, imm",imm[12|10:5],rs2[4:0],rs1[4:0],000,imm[11|4:1], 1100011),
            riscv_dis::instruction32!("bne rs1, rs2, imm",imm[12|10:5],rs2[4:0],rs1[4:0],001,imm[11|4:1], 1100011),
            riscv_dis::instruction32!("blt rs1, rs2, imm",imm[12|10:5],rs2[4:0],rs1[4:0],100,imm[11|4:1], 1100011),
            riscv_dis::instruction32!("bltu rs1, rs2, imm",imm[12|10:5],rs2[4:0],rs1[4:0],110,imm[11|4:1], 1100011),
            riscv_dis::instruction32!("bge rs1, rs2, imm",imm[12|10:5],rs2[4:0],rs1[4:0],101,imm[11|4:1], 1100011),
            riscv_dis::instruction32!("bgeu rs1, rs2, imm",imm[12|10:5],rs2[4:0],rs1[4:0],111,imm[11|4:1], 1100011),

            riscv_dis::instruction32!("lb rd, imm (rs1)", imm[11:0],rs1[4:0], 000 ,rd[4:0], 0000011),
            riscv_dis::instruction32!("lh rd, imm (rs1)", imm[11:0],rs1[4:0], 001 ,rd[4:0], 0000011),
            riscv_dis::instruction32!("lw rd, imm (rs1)", imm[11:0],rs1[4:0], 010 ,rd[4:0], 0000011),
            riscv_dis::instruction32!("ld rd, imm (rs1)", imm[11:0],rs1[4:0], 011 ,rd[4:0], 0000011),
            riscv_dis::instruction32!("lbu rd, imm (rs1)", imm[11:0],rs1[4:0], 100 ,rd[4:0], 0000011),
            riscv_dis::instruction32!("lhu rd, imm (rs1)", imm[11:0],rs1[4:0], 101 ,rd[4:0], 0000011),
            riscv_dis::instruction32!("lwu rd, imm (rs1)", imm[11:0],rs1[4:0], 110 ,rd[4:0], 0000011),

            riscv_dis::instruction32!("sb rs2, imm (rs1)", imm[11:5],rs2[4:0],rs1[4:0],000,imm[4:0], 0100011),
            riscv_dis::instruction32!("sh rs2, imm (rs1)", imm[11:5],rs2[4:0],rs1[4:0],001,imm[4:0], 0100011),
            riscv_dis::instruction32!("sw rs2, imm (rs1)", imm[11:5],rs2[4:0],rs1[4:0],010,imm[4:0], 0100011),
            riscv_dis::instruction32!("sd rs2, imm (rs1)", imm[11:5],rs2[4:0],rs1[4:0],011,imm[4:0], 0100011),

            riscv_dis::instruction32!("fence.tso", 10000011001100000000000000001111),
            riscv_dis::instruction32!("fence imm", imm[11:0],00000000000000001111),

            riscv_dis::instruction32!("ecall", 00000000000000000000000001110011),
            riscv_dis::instruction32!("ebreak", 00000000000100000000000001110011),

            //RV32M
            riscv_dis::instruction32!("mul rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],000,rd[4:0], 0110011),
            riscv_dis::instruction32!("mulh rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],001,rd[4:0], 0110011),
            riscv_dis::instruction32!("mulhsu rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],010,rd[4:0], 0110011),
            riscv_dis::instruction32!("mulhu rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],011,rd[4:0], 0110011),
            //RV64 riscv_dis::instruction32!("mulw rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],000,rd[4:0], 0111011),

            riscv_dis::instruction32!("div rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],100,rd[4:0], 0110011),
            riscv_dis::instruction32!("divu rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],101,rd[4:0], 0110011),
            riscv_dis::instruction32!("rem rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],110,rd[4:0], 0110011),
            riscv_dis::instruction32!("remu rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],111,rd[4:0], 0110011),
            //RV64 riscv_dis::instruction32!("divw rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],100,rd[4:0], 0111011),
            //RV64 riscv_dis::instruction32!("divuw rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],101,rd[4:0], 0111011),
            //RV64 riscv_dis::instruction32!("remw rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],110,rd[4:0], 0111011),
            //RV64 riscv_dis::instruction32!("remuw rd, rs1, rs2", 0000001,rs2[4:0],rs1[4:0],111,rd[4:0], 0111011),

            //RV32A

            riscv_dis::instruction32!("<illegal>", 00000000000000000000000000000000),
       ];

       let mut show_dict = HashMap::new();
       show_dict.insert(String::from("rd"), show_register as ShowFun::<RV32Type>);
       show_dict.insert(String::from("rs1"), show_register as ShowFun::<RV32Type>);
       show_dict.insert(String::from("rs2"), show_register as ShowFun::<RV32Type>);

        ISARV32IM { list, show_dict }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        let isa = ISARV32IM::new();
        println!("{:?}", isa);
        assert!(true);
    }

    #[test]
    fn i1() {
        let i1 = riscv_dis::instruction32!("nop", 00000000000000000000000000010011);

        let list = vec![ Item::Bits { len : 32, val : 0x13 }];
        let bin = BinaryInstruction { list };
        let text = TextInstruction { list : vec![ TextInstructionPart::Text(String::from("nop")) ] };
        assert_eq!(i1.text, text);
        assert_eq!(i1.bin, bin);
    }

}
