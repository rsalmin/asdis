use crate::primitives::*;

pub fn decode(fmt : &InstructionFmt, v : u32, op: &Opcode) -> Instruction {
    match fmt{
        InstructionFmt::R => decode_r(v, op),
        InstructionFmt::I => decode_i(v, op),
        InstructionFmt::S => decode_s(v, op),
        InstructionFmt::SB => decode_sb(v, op),
        InstructionFmt::U => decode_u(v, op),
        InstructionFmt::UJ => decode_uj(v, op),
    }
}

fn decode_r(v : u32, op: &Opcode) -> Instruction {
     let v = v >> 7;
     let rd = Register::new( (v & 0x1F) as u8 );
     let v = v >> 5;
     let func3 = Func3::new( ( v & 0x7 ) as u8);
     let v = v >> 3;
     let rs1 = Register::new( (v & 0x1F) as u8 );
     let v = v >> 5;
     let rs2 = Register::new( (v & 0x1F) as u8 );
     let v = v >> 5;
     let func7 = Func7::new( (v & 0x5F ) as u8 );
     Instruction::R { func7, rs2, rs1, func3, rd, opcode: *op }
}

fn decode_i(v : u32, op: &Opcode) -> Instruction {
    let rd = Register::new( ( ( v & 0xF80 ) >> 7 ) as u8 );
    let func3 = Func3::new( ( (v & 0x7000) >> 12 ) as u8 );
    let rs1 = Register::new( ( (v & 0xF8000) >> 15 ) as u8 );
    let imm = ( ( v & 0xFFF00000 ) >> 20 ) as u16; // actually u12
    Instruction::I {immfunc : ImmFunc::from_imm(imm), rs1, func3, rd,  opcode: *op }
}

fn decode_s(v : u32, op: &Opcode) -> Instruction {
     let v = v >> 7;
     let bit0to4 = v & 0x1F ;
     let v = v >> 5;
     let func3 = Func3::new( ( v & 0x7 ) as u8);
     let v = v >> 3;
     let rs1 = Register::new( (v & 0x1F) as u8 );
     let v = v >> 5;
     let rs2 = Register::new( (v & 0x1F) as u8 );
     let v = v >> 5;
     let bit5to11 = v & 0x5F ;
     let imm = ( (bit5to11 << 5) | bit0to4 ) as u16;
     Instruction::S {imm: ux::u12::new(imm), rs2, rs1, func3, opcode: *op}
}

fn decode_sb(v : u32, op: &Opcode) -> Instruction {
     let v = v >> 7;
     let bit11 = v & 1;
     let v = v >> 1;
     let bit1to4 =  v & 0xF ;
     let v = v >> 4;
     let func3 = Func3::new( ( v & 0x7 ) as u8);
     let v = v >> 3;
     let rs1 = Register::new( (v & 0x1F) as u8 );
     let v = v >> 5;
     let rs2 = Register::new( (v & 0x1F) as u8 );
     let v = v >> 5;
     let bit5to10 = v & 0x3F ;
     let v = v >> 6;
     let bit12 = v & 1;
     let imm = ( (bit12 << 12) | (bit11 << 11) | (bit5to10 << 5) | (bit1to4 << 1) ) as u16; // note first 0 bit
     Instruction::SB {imm: ux::u13::new(imm), rs2, rs1, func3, opcode: *op}
}

fn decode_u(v : u32, op: &Opcode) -> Instruction {
    let rd = Register::new( ( ( v & 0xF80 ) >>7 ) as u8 );
    let imm = v & 0xFFFFF000;
    Instruction::U { imm, rd, opcode : *op}
}

fn decode_uj(v : u32, op: &Opcode) -> Instruction {
    let rd = Register::new( ( ( v & 0xF80 ) >>7 ) as u8 );
    let v = v >> 12; // remove op and rd
    let bit12to19 =  v &  0xFF ;
    let v = v >> 8;
    let bit11 = v & 1;
    let v = v >> 1;
    let bit1to10 = v & 0x3FF;
    let v = v >> 10;
    let bit20 = v & 1 ;

    let imm = (bit20 << 20) | (bit12to19 << 12) | (bit11 << 11) | (bit1to10 << 1); // note first bit is 0

    Instruction::UJ { imm : ux::u21::new(imm), rd, opcode : *op}
}

