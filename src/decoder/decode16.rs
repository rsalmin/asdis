use crate::primitives::*;
use crate::isa::*;

/// decode given word(u16) using instruction list, returns text description of instruction
pub fn decode16(v : u16, isa : &ISARV32C) -> String {
    for i in &isa.list {
        if let Some( s ) = try_instruction(v, &i)  {
            return s;
        }
    }
    String::from("c. Not Found!")
}


/// checks if given word(u16) is encode given binary instruction
fn check16( v : u16, instr : &BinaryInstruction) -> bool {
    let mut mask : u16 = 0;
    let mut pattern : u16 = 0;
    for item in &instr.list {
        match item {
            Item::Bits { len, val } => {
                for i in 0..*len {
                    mask = mask << 1;
                    mask = mask | 1;
                }
                pattern = pattern.rotate_left( *len as u32 );
                pattern = pattern | val;
            },
            Item::Ident { name:_, bitspec } => {
                let l = bitspec.len();
                mask = mask.rotate_left( l as u32);
                pattern = pattern.rotate_left( l as u32);
            }
        }
    }
    v & mask == pattern
}

/// try to find corespondence between given word and given instruction,
/// if found return text description of instruction, otherwise None
pub fn try_instruction( v : u16, i : &Instruction ) -> Option<String> {
    if !check16(v, &i.bin) {
        return None;
    }

    let mut str = String::new();
    for t in &i.text.list {
        match t {
            TextInstructionPart::Text( s ) => str = str + &s[..],
            TextInstructionPart::TextIdent( s1, s2 ) => str = str + &s1[..] + &s2[..],
        }
    }
    Some( str  )
}
