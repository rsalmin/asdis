use std::collections::HashMap;
use std::hash::Hash;
use ux;

use crate::primitives::*;

struct ISARV32C;

impl ISARV32C {
    pub fn new() -> ISARV32C {

        //let ilist = [
            //instruction!(000, imm[5:4|9:6|2|3], rd3, 00),
            //instruction!(010, imm[5:3], rs13, imm[2|6], rd3, 00),
        //];

        ISARV32C
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        let r = riscv_dis::instruction!(000, imm[5:4|9:6|2|3], rd[2:0], 00);
        println!("{}", r);
        assert!(true);
    }
}
