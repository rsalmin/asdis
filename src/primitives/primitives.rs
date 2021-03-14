use std::fmt;
use std::hash::Hash;
use std::convert::From;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
pub enum Item {
    Bits {len : usize, val : u16},
    Ident {name:String, bitspec:Vec<u8>},
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Bits { len, val } => write!(f, "{:0width$b}", val, width = len),
            Item::Ident {name, bitspec} => write!(f, "{}{:?}", name, bitspec),
        }
    }
}

#[derive(Debug)]
pub struct BinaryInstruction {
    pub list : Vec<Item>,
}

#[derive(Debug)]
pub struct Instruction {
    pub bin : BinaryInstruction,
    pub text : TextInstruction,
}



#[derive(PartialEq, Eq, Debug)]
enum TextInstructionPart {
    Text(String),
    TextIdent(String, String),
}

impl TextInstructionPart {
    fn text(s : &str) -> TextInstructionPart {
        TextInstructionPart::Text(String::from(s))
    }
    fn text_ident(s1 : &str, s2 : &str) -> TextInstructionPart {
        TextInstructionPart::TextIdent(String::from(s1), String::from(s2))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct TextInstruction {
   list : Vec<TextInstructionPart>,
}

impl From<&str> for TextInstruction {
    fn from(text : &str) -> TextInstruction {
        lazy_static! {
            static ref  re : Regex = Regex::new(r"[[:alpha:]]([[:alnum:]]|\.)*").unwrap();
        }
        enum State<'a> {
            Start,
            First(&'a str, usize),
            Next(usize),
        };

        let mut state = State::Start;

        let mut list = Vec::<TextInstructionPart>::new();

        for m in re.find_iter(text) {
            match state {
                State::Start => state = State::First(m.as_str(), m.end()),
                State::First(s1, e1) => {
                    let mut txt = String::from(s1);
                    txt.push_str( &text[e1..m.start()] );
                    list.push( TextInstructionPart::TextIdent(txt, String::from(m.as_str())) );
                    state = State::Next( m.end() );
                },
                State::Next(e1) => {
                    list.push( TextInstructionPart::text_ident(&text[e1..m.start()], m.as_str()) );
                    state = State::Next( m.end() );
                },
            }
        }

        match state {
            State::Start => (),
            State::First(s1, e1) => list.push( TextInstructionPart::text(s1) ),
            State::Next(e1) => if text.len() > e1 {
                list.push( TextInstructionPart::text(&text[e1..text.len()]) )
            },
        };

        TextInstruction { list }
    }
}

macro_rules! make_u_type {
    ($name:ident, $bits:literal) => {
        #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
        pub struct $name(u8);

        impl $name {
            pub fn new(v : u8) -> Self {
                let max_val = 2u8.pow( $bits );
                assert!( v < max_val );
                $name(v)
           }
       }

       impl fmt::Display for $name {
           fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
               write!(f, "{:#04X}", self.0)
           }
       }

    }
}

make_u_type!(Opcode, 7);
make_u_type!(Func3, 3);
make_u_type!(Register, 5);
make_u_type!(Func7, 7);

make_u_type!(OpcodeC, 2);
make_u_type!(Func2, 2);
make_u_type!(Func4, 4);
make_u_type!(Func6, 6);
make_u_type!(Register3, 3);


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ti1() {
        assert_eq!( TextInstruction::from("c.nop"),
                            TextInstruction { list : vec![ TextInstructionPart::text("c.nop") ] } );
    }

    #[test]
    fn ti2() {
        assert_eq!( TextInstruction::from("mv rd, rs1"),
                            TextInstruction { list : vec![ TextInstructionPart::text_ident("mv ", "rd"),
                                                                           TextInstructionPart::text_ident(", ", "rs1") ] } );
    }
    #[test]
    fn ti3() {
        assert_eq!( TextInstruction::from("c.sw rs1, imm (rs2)"),
                            TextInstruction { list : vec![ TextInstructionPart::text_ident("c.sw ", "rs1"),
                                                                           TextInstructionPart::text_ident(", ", "imm"),
                                                                           TextInstructionPart::text_ident(" (", "rs2"),
                                                                           TextInstructionPart::text(")")
                                                                            ] } );
    }
}