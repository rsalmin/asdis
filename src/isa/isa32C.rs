use std::collections::HashMap;
use std::hash::Hash;
use ux;

use crate::primitives::*;

struct ISARV32C;

impl ISARV32C {
    pub fn new() -> ISARV32C {

        let ilist = [
            riscv_dis::instruction!("c.addi4spn rd, imm", 000, imm[5:4|9:6|2|3], rd[2:0], 00),
            riscv_dis::instruction!("c.lw rd, rs1, imm", 001, imm[5:3], rs1[2:0], imm[2|6], rd[2:0], 00),
            riscv_dis::instruction!("c.sw rs1, imm (rs2)", 001, imm[5:3], rs1[2:0], imm[2|6], rd[2:0], 00),
            riscv_dis::instruction!("c.nop", 000, imm[5], 00000, imm[4:0], 01),
            riscv_dis::instruction!("c.addi rd, imm", 000, imm[5], rd[4:0], imm[4:0], 01),
            riscv_dis::instruction!("c.jal imm", 001, imm[11|4|9:8|10|6|7|3:1|5], 01),
            riscv_dis::instruction!("c.li rd, imm", 010, imm[5], rd[4:0], imm[4:0], 01),
            riscv_dis::instruction!("c.addi16sp imm", 011, imm[9], 00010, imm[4|6|8:7|5], 01),
            riscv_dis::instruction!("c.lui rd, imm", 011, imm[17], rd[4:0], imm[16:12], 01),
            riscv_dis::instruction!("c.srli rd, imm", 100100, rd[2:0], imm[4:0], 01),
            riscv_dis::instruction!("c.srai rd, imm", 100101, rd[2:0], imm[4:0], 01),
            riscv_dis::instruction!("c.andi rd, imm", 100, imm[5], 10, rd[2:0], imm[4:0], 01),
            riscv_dis::instruction!("c.sub rd, rs2", 100011, rd[2:0], 00, rs2[2:0], 01),
            riscv_dis::instruction!("c.xor rd, rs2", 100011, rd[2:0], 01, rs2[2:0], 01),
            riscv_dis::instruction!("c.or rd, rs2", 100011, rd[2:0], 10, rs2[2:0], 01),
            riscv_dis::instruction!("c.and rd, rs2", 100011, rd[2:0], 11, rs2[2:0], 01),
            riscv_dis::instruction!("c.j imm", 101, imm[11|4|9:8|10|6|7|3:1|5], 01),
            riscv_dis::instruction!("c.beqz rs1, imm", 110, imm[8|4:3], rs1[2:0], imm[7:6|2:1|5], 01),
            riscv_dis::instruction!("c.bnez rs1, imm", 110, imm[8|4:3], rs1[2:0], imm[7:6|2:1|5], 01),

            riscv_dis::instruction!("c.slli rd, imm", 000, imm[5], rd[4:0], imm[4:0], 10),
            riscv_dis::instruction!("c.lwsp rd, imm", 010, imm[5], rd[4:0], imm[4:2|7:6], 10),
            riscv_dis::instruction!("c.jr rs1", 1000, rs1[4:0], 0000010),
            riscv_dis::instruction!("c.mv rd, rs2", 1000, rd[4:0], rs2[4:0], 10),
            riscv_dis::instruction!("c.ebreak", 1001000000000010),
            riscv_dis::instruction!("c.jalr rs1", 1001, rs1[4:0], 0000010),
            riscv_dis::instruction!("c.add rd, rs2", 1001, rd[4:0], rs2[4:0], 10),
            riscv_dis::instruction!("c.swsp rs2, imm", 110, imm[5:2|7:6], rs2[4:0], 10),
       ];

        ISARV32C
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        let r = riscv_dis::instruction!("c.addi4spn rd imm", 000, imm[5:4|9:6|2|3], rd[2:0], 00);
        //let r = riscv_dis::instruction!(000, imm[5:4|9:6|2|3], rd[2:0], 00);
        println!("{:?}", r);
        assert!(true);
    }
}
