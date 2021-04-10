use crate::primitives::*;
use crate::isa::isa::*;
use std::collections::HashMap;

pub type ISARV32IMA = ISA<RV32Type>;

///helper to show register
fn show_register(v : u32) -> String
{
    format!("r{}", v)
}

impl ISARV32IMA {
    pub fn new() -> ISARV32IMA {

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
            riscv_dis::instruction32!("lr.w rd, rs1", 00010,00,00000,rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("lr.w.aq rd, rs1", 00010,10,00000,rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("lr.w.rl rd, rs1", 00010,01,00000,rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("lr.w.aq.rl rd, rs1", 00010,11,00000,rs1[4:0],010,rd[4:0], 0101111),

            //RV64 riscv_dis::instruction32!("lr.d rd, rs1", 00010,00,00000,rs1[4:0],011,rd[4:0], 0101111),
            //RV64 riscv_dis::instruction32!("lr.d.aq rd, rs1", 00010,10,00000,rs1[4:0],011,rd[4:0], 0101111),
            //RV64 riscv_dis::instruction32!("lr.d.rl rd, rs1", 00010,01,00000,rs1[4:0],011,rd[4:0], 0101111),
            //RV64 riscv_dis::instruction32!("lr.d.aq.rl rd, rs1", 00010,11,00000,rs1[4:0],011,rd[4:0], 0101111),

            riscv_dis::instruction32!("sc.w rd, rs1 (rs2)", 00011,00,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("sc.w.aq rd, rs1 (rs2)", 00011,10,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("sc.w.rl rd, rs1 (rs2)", 00011,01,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("sc.w.aq.rl rd, rs1 (rs2)", 00011,11,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),

            //RV64 riscv_dis::instruction32!("sc.d rd, rs1 (rs2)", 00011,00,rs2[4:0],rs1[4:0],011,rd[4:0], 0101111),
            //RV64 riscv_dis::instruction32!("sc.d.aq rd, rs1 (rs2)", 00011,10,rs2[4:0],rs1[4:0],011,rd[4:0], 0101111),
            //RV64 riscv_dis::instruction32!("sc.d.rl rd, rs1 (rs2)", 00011,01,rs2[4:0],rs1[4:0],011,rd[4:0], 0101111),
            //RV64 riscv_dis::instruction32!("sc.d.aq.rl rd, rs1 (rs2)", 00011,11,rs2[4:0],rs1[4:0],011,rd[4:0], 0101111),

            riscv_dis::instruction32!("amoswap.w rd, rs1, rs2", 00001,00,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoswap.w.aq rd, rs1, rs2", 00001,10,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoswap.w.rl rd, rs1, rs2", 00001,01,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoswap.w.aq.rl rd, rs1, rs2", 00001,11,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),

            riscv_dis::instruction32!("amoadd.w rd, rs1, rs2", 00000,00,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoadd.w.aq rd, rs1, rs2", 00000,10,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoadd.w.rl rd, rs1, rs2", 00000,01,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoadd.w.aq.rl rd, rs1, rs2", 00000,11,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),

            riscv_dis::instruction32!("amoand.w rd, rs1, rs2", 01100,00,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoand.w.aq rd, rs1, rs2", 01100,10,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoand.w.rl rd, rs1, rs2", 01100,01,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoand.w.aq.rl rd, rs1, rs2", 01100,11,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),

            riscv_dis::instruction32!("amoor.w rd, rs1, rs2", 01000,00,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoor.w.aq rd, rs1, rs2", 01000,10,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoor.w.rl rd, rs1, rs2", 01000,01,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoor.w.aq.rl rd, rs1, rs2", 01000,11,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),

            riscv_dis::instruction32!("amoxor.w rd, rs1, rs2", 00100,00,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoxor.w.aq rd, rs1, rs2", 00100,10,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoxor.w.rl rd, rs1, rs2", 00100,01,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amoxor.w.aq.rl rd, rs1, rs2", 00100,11,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),

            riscv_dis::instruction32!("amomax.w rd, rs1, rs2", 10100,00,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amomax.w.aq rd, rs1, rs2", 10100,10,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amomax.w.rl rd, rs1, rs2", 10100,01,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amomax.w.aq.rl rd, rs1, rs2", 10100,11,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),

            riscv_dis::instruction32!("amomaxu.w rd, rs1, rs2", 11100,00,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amomaxu.w.aq rd, rs1, rs2", 11100,10,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amomaxu.w.rl rd, rs1, rs2", 11100,01,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amomaxu.w.aq.rl rd, rs1, rs2", 11100,11,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),

            riscv_dis::instruction32!("amomin.w rd, rs1, rs2", 10000,00,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amomin.w.aq rd, rs1, rs2", 10000,10,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amomin.w.rl rd, rs1, rs2", 10000,01,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amomin.w.aq.rl rd, rs1, rs2", 10000,11,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),

            riscv_dis::instruction32!("amominu.w rd, rs1, rs2", 11000,00,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amominu.w.aq rd, rs1, rs2", 11000,10,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amominu.w.rl rd, rs1, rs2", 11000,01,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),
            riscv_dis::instruction32!("amominu.w.aq.rl rd, rs1, rs2", 11000,11,rs2[4:0],rs1[4:0],010,rd[4:0], 0101111),

            //for RV64 repeat last 9 parck of 4 commands with suffix .d, with changed with param from 010 to 011

            //Zifencei
            riscv_dis::instruction32!("fence.i",00000000000000000000000000001111),

            //Zicsr
            riscv_dis::instruction32!("csrrw rd, csr, rs1",csr[11:0],rs1[4:0],001,rd[4:0],1110011),
            riscv_dis::instruction32!("csrrs rd, csr, rs1",csr[11:0],rs1[4:0],010,rd[4:0],1110011),
            riscv_dis::instruction32!("csrrc rd, csr, rs1",csr[11:0],rs1[4:0],011,rd[4:0],1110011),
            riscv_dis::instruction32!("csrrwi rd, csr, imm",csr[11:0],imm[4:0],101,rd[4:0],1110011),
            riscv_dis::instruction32!("csrrsi rd, csr, imm",csr[11:0],imm[4:0],110,rd[4:0],1110011),
            riscv_dis::instruction32!("csrrci rd, csr, imm",csr[11:0],imm[4:0],111,rd[4:0],1110011),

            //MISC
            riscv_dis::instruction32!("<illegal.0>", 00000000000000000000000000000000),
            riscv_dis::instruction32!("<illegal.1>", 11111111111111111111111111111111),
       ];

       let mut show_dict = HashMap::new();
       show_dict.insert(String::from("rd"), show_register as ShowFun::<RV32Type>);
       show_dict.insert(String::from("rs1"), show_register as ShowFun::<RV32Type>);
       show_dict.insert(String::from("rs2"), show_register as ShowFun::<RV32Type>);

        ISARV32IMA { list, show_dict }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        let isa = ISARV32IMA::new();
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
