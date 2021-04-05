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
/// (we can build mask and pattern in compile time in the future)
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

/// extract from given instruction bit for idents and return tuples of (ident, val, start_bit)
fn extract_idents( val : u16, instr : &BinaryInstruction) -> Vec<(String, u32, u8)> {
    let mut current_bit = 15_u16;
    let mut result = Vec::<(String, u32, u8)>::new();

    for item in &instr.list {
        match item {
            Item::Bits { len, val:_ } => {
                let len = *len as u16;
                if len > current_bit {
                    break;
                }
                current_bit -= len;
            },
            Item::Ident { name, bitspec } => {
                let l = bitspec.len();
                let m = bitspec.iter().min().expect("Bitspec can't be empty!");
                let mut v  = 0_u32;
                for sbit in bitspec {
                     //take bit from current_bit and put bit to sbit bit
                     let bit = (val & (2_u16.pow(current_bit.into()))) >> current_bit;
                     let bit = (bit as u32) << sbit;
                     v |= bit;
                     if current_bit == 0 {
                         break;
                     }
                     current_bit -= 1;
                }
                result.push((name.clone(), v, *m));
            }
        }
    }
    result
}

/// try to find corespondence between given word and given instruction,
/// if found return text description of instruction, otherwise None
pub fn try_instruction( v : u16, i : &Instruction ) -> Option<String> {
    if !check16(v, &i.bin) {
        return None;
    }

    let vars = extract_idents( v, &i.bin);

    let mut str = String::new();
    for t in &i.text.list {
        match t {
            TextInstructionPart::Text( s ) => str = str + &s[..],
            TextInstructionPart::TextIdent( s1, s2 ) => {
                let attr = match vars.iter().find(|(n, _, _)| n == s2) {
                    None => String::from("none"),
                    Some((n_, v, s_)) => format!("{:#X}", *v),
                };
                str = str + &s1[..] + &s2[..] + ":" + &attr[..];
            },
        }
    }
    Some( str  )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_id_1() {
        //c.jal
        let list = vec![ Item::Bits { len : 3, val : 1 },
                                 Item::Ident { name : String::from("imm"), bitspec : vec![11,4,9,8,10,6,7,3,2,1,5] },
                                 Item::Bits { len : 2, val : 1 },
                              ];
        let jal_bin = BinaryInstruction { list };
        let instr = 0b0011010001101101_u16;
        let r = extract_idents( instr, &jal_bin);
        assert_eq!(r.len(), 1);
        let (name, v, start) = &r[0];
        assert_eq!(name, "imm");
        assert_eq!(*v, 0xAAA); //Note: value not shifted by start bit
        assert_eq!(*start, 1);
    }
}