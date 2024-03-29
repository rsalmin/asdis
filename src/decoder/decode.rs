use crate::primitives::*;
use crate::isa::*;

/// decode given word(u16) using instruction list, returns text description of instruction
pub fn decode<T:Num>(v : T::IType, isa : &ISA::<T>) -> String {
    for i in &isa.list {
        if let Some( s ) = try_instruction::<T>(v, &i, &isa.show_dict)  {
            return s;
        }
    }
    format!("{}. Not found!", T::type_name())
}

/// extract from given instruction bit for idents and return tuples of (ident, val, start_bit)
fn extract_idents<T:Num>( val : T::IType, instr : &BinaryInstruction::<T>) -> Vec<(String, T::DType, u32)> {
    let mut current_bit = T::i_max_bit();
    let mut result = Vec::<(String, T::DType, u32)>::new();

    for item in &instr.list {
        match item {
            Item::Bits { len, val:_ } => {
                let len = *len as u32;
                if len > current_bit {
                    break;
                }
                current_bit -= len;
            },
            Item::Ident { name, bitspec } => {
                let m = bitspec.iter().min().expect("Bitspec can't be empty!");
                let mut v : T::DType = T::d_zero();
                for sbit in bitspec {
                     //take bit from current_bit and put bit to sbit bit
                     let bit = T::get_bit(val, current_bit);
                     let bit = bit  << (*sbit as usize);
                     v = v | bit;
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
pub fn try_instruction<T:Num>( v : T::IType, i : &Instruction::<T>, show_dict : &ShowDict::<T> ) -> Option<String> {
    if ! ( v & i.mask() == i.pattern() ) {
        return None;
    }

    let vars = extract_idents( v, &i.bin);

    let mut str = String::new();
    for t in &i.text.list {
        match t {
            TextInstructionPart::Text( s ) => str = str + &s[..],
            TextInstructionPart::TextIdent( s1, ident ) => {
                let attr = match vars.iter().find(|(n, _, _)| n == ident) {
                    None => String::from("****"),
                    Some((_n, v, _s)) => {
                        match show_dict.get(ident) {
                            None => format!("{:#X}", *v),
                            Some( f ) => f(*v),
                        }
                    },
                };
                str = str + &s1[..] + &attr[..];
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
        let list = vec![ Item::<CompactType>::Bits { len : 3, val : 1 },
                                 Item::<CompactType>::Ident { name : String::from("imm"), bitspec : vec![11,4,9,8,10,6,7,3,2,1,5] },
                                 Item::<CompactType>::Bits { len : 2, val : 1 },
                              ];
        let jal_bin = BinaryInstruction::<CompactType> { list };
        let instr = 0b0011010001101101_u16;
        let r = extract_idents( instr, &jal_bin);
        assert_eq!(r.len(), 1);
        let (name, v, start) = &r[0];
        assert_eq!(name, "imm");
        assert_eq!(*v, 0xAAA); //Note: value not shifted by start bit
        assert_eq!(*start, 1);
    }
}