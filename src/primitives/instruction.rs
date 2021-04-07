use std::fmt;
use std::convert::From;
use regex::Regex;
use lazy_static::lazy_static;
use quote::{quote, ToTokens, TokenStreamExt};
use proc_macro2::{TokenStream, TokenTree, Ident, Group, Delimiter, Span, Punct, Literal, Spacing};
use std::num::ParseIntError;
use num_traits::int::PrimInt;


pub trait Num {
    type IType : std::fmt::Binary + ToTokens  + fmt::Debug + PrimInt;
    fn from_str_radix(src: &str, radix: u32) -> Result<Self::IType, ParseIntError>;
    fn one() -> Self::IType;
    fn zero() -> Self::IType;
}

impl Num for u16
{
    type IType = u16;
    fn from_str_radix(src: &str, radix: u32) -> Result<Self::IType, ParseIntError> {
        u16::from_str_radix(src, radix)
    }
    fn one() -> Self::IType { 1 }
    fn zero() -> Self::IType { 0 }
}

/// Item represents part of binary encoded instruction, it is either just bits, or ident with bit sepcification
#[derive(PartialEq, Eq, Debug)]
pub enum Item<T : Num> {
    Bits {len : usize, val : T::IType},
    Ident {name:String, bitspec:Vec<u8>},
}

impl<T : Num> fmt::Display for Item<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Bits { len, val } => write!(f, "{:0width$b}", val, width = len),
            Item::Ident {name, bitspec} => write!(f, "{}{:?}", name, bitspec),
        }
    }
}



///helper function, to append given string as String in TokenStream
fn app_string_from(tokens: &mut TokenStream, s : &str) {
    tokens.append( TokenTree::Ident( Ident::new("String", Span::call_site())) );
    tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
    tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
    tokens.append( TokenTree::Ident( Ident::new("from", Span::call_site())) );
    let mut in_str_group = TokenStream::new();
    in_str_group.append( TokenTree::Literal( Literal::string(s) ) );
    tokens.append( TokenTree::Group( Group::new( Delimiter::Parenthesis, in_str_group ) ) );
}

impl<T:Num> ToTokens for Item<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append( TokenTree::Ident( Ident::new("Item", Span::call_site())) );

        tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
        tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
        tokens.append( TokenTree::Punct( Punct::new('<', Spacing::Joint) ) );
        tokens.append( TokenTree::Ident( Ident::new(std::any::type_name::<T>(), Span::call_site())) );
        tokens.append( TokenTree::Punct( Punct::new('>', Spacing::Joint) ) );

        tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
        tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
        match self {
            Item::Bits {len, val}  => {
                tokens.append( TokenTree::Ident( Ident::new("Bits", Span::call_site())) );
                let mut inside_braces = TokenStream::new();
                inside_braces.append( TokenTree::Ident( Ident::new("len", Span::call_site())) );
                inside_braces.append( TokenTree::Punct( Punct::new(':', Spacing::Alone) ) );
                inside_braces.append( TokenTree::Literal( Literal::usize_suffixed(*len) ) );
                inside_braces.append( TokenTree::Punct( Punct::new(',', Spacing::Alone) ) );
                inside_braces.append( TokenTree::Ident( Ident::new("val", Span::call_site())) );
                inside_braces.append( TokenTree::Punct( Punct::new(':', Spacing::Alone) ) );
                val.to_tokens( &mut inside_braces );
                tokens.append( TokenTree::Group( Group::new( Delimiter::Brace, inside_braces ) ) );
            },
            Item::Ident {name, bitspec }=> {
                tokens.append( TokenTree::Ident( Ident::new("Ident", Span::call_site())) );
                let mut inside_braces = TokenStream::new();
                inside_braces.append( TokenTree::Ident( Ident::new("name", Span::call_site())) );
                inside_braces.append( TokenTree::Punct( Punct::new(':', Spacing::Alone) ) );
                app_string_from( &mut inside_braces, &name[..] );
                inside_braces.append( TokenTree::Punct( Punct::new(',', Spacing::Alone) ) );
                inside_braces.append( TokenTree::Ident( Ident::new("bitspec", Span::call_site())) );
                inside_braces.append( TokenTree::Punct( Punct::new(':', Spacing::Alone) ) );
                inside_braces.append( TokenTree::Ident( Ident::new("vec", Span::call_site()) ) );
                inside_braces.append( TokenTree::Punct( Punct::new('!', Spacing::Joint) ) );
                let mut item_list = TokenStream::new();
                item_list.append_separated( bitspec.iter(), Punct::new(',', Spacing::Alone));
                inside_braces.append( TokenTree::Group( Group::new( Delimiter::Bracket, item_list ) ) );
                tokens.append( TokenTree::Group( Group::new( Delimiter::Brace, inside_braces ) ) );
            },
        };
    }
}


/// Binary Instruction represents description of instruction binary representation, it consists of list of Items
#[derive(PartialEq, Eq, Debug)]
pub struct BinaryInstruction<T:Num> {
    pub list : Vec<Item<T>>,
}

impl<T:Num> ToTokens for BinaryInstruction<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append( TokenTree::Ident( Ident::new("BinaryInstruction", Span::call_site()) ) );

        tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
        tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
        tokens.append( TokenTree::Punct( Punct::new('<', Spacing::Joint) ) );
        tokens.append( TokenTree::Ident( Ident::new(std::any::type_name::<T>(), Span::call_site())) );
        tokens.append( TokenTree::Punct( Punct::new('>', Spacing::Joint) ) );

        let mut inside_braces = TokenStream::new();
        inside_braces.append( TokenTree::Ident( Ident::new("list", Span::call_site())) );
        inside_braces.append( TokenTree::Punct( Punct::new(':', Spacing::Alone) ) );
        inside_braces.append( TokenTree::Ident( Ident::new("vec", Span::call_site()) ) );
        inside_braces.append( TokenTree::Punct( Punct::new('!', Spacing::Joint) ) );

        let mut item_list = TokenStream::new();
        item_list.append_separated( self.list.iter(), Punct::new(',', Spacing::Alone));

        inside_braces.append( TokenTree::Group( Group::new( Delimiter::Bracket, item_list ) ) );
        tokens.append( TokenTree::Group( Group::new( Delimiter::Brace, inside_braces ) ) );
    }
}


///Text instruction part is represent part of textual description of instruction, it either just text, or text followed by some variable name
#[derive(PartialEq, Eq, Debug)]
pub enum TextInstructionPart {
    Text(String),
    TextIdent(String, String),
}


impl ToTokens for TextInstructionPart {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append( TokenTree::Ident( Ident::new("TextInstructionPart", Span::call_site())) );
        tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
        tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
        match self {
            TextInstructionPart::Text(s) => {
                tokens.append( TokenTree::Ident( Ident::new("Text", Span::call_site())) );
                let mut string = TokenStream::new();
                app_string_from(&mut string, &s[..]);
                tokens.append( TokenTree::Group( Group::new( Delimiter::Parenthesis, string ) ) );
            },
            TextInstructionPart::TextIdent(s1, s2) => {
                tokens.append( TokenTree::Ident( Ident::new("TextIdent", Span::call_site())) );
                let mut strings = TokenStream::new();
                app_string_from(&mut strings, &s1[..]);
                strings.append( TokenTree::Punct( Punct::new(',', Spacing::Alone) ) );
                app_string_from(&mut strings, &s2[..]);
                tokens.append( TokenTree::Group( Group::new( Delimiter::Parenthesis, strings ) ) );
            }
        }
    }
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
   pub list : Vec<TextInstructionPart>,
}

impl From<&str> for TextInstruction {
    fn from(text : &str) -> TextInstruction {
        lazy_static! {
            /// asm ident must start with a letter, then letter, number of '.' any number of times
            static ref  RE : Regex = Regex::new(r"[[:alpha:]]([[:alnum:]]|\.)*").unwrap();
        }
        enum State<'a> {
            Start,
            First(&'a str, usize),
            Next(usize),
        }

        let mut state = State::Start;

        let mut list = Vec::<TextInstructionPart>::new();

        for m in RE.find_iter(text) {
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
            State::First(s1, e1) =>
                if text.len() > e1 {
                    let mut s = String::from(s1);
                    s.push_str(&text[e1..text.len()]);
                    list.push( TextInstructionPart::text(&s[..]) );
                } else {
                        list.push( TextInstructionPart::text(s1) );
               },
            State::Next(e1) => if text.len() > e1 {
                list.push( TextInstructionPart::text(&text[e1..text.len()]) )
            },
        };

        TextInstruction { list }
    }
}

impl ToTokens for TextInstruction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append( TokenTree::Ident( Ident::new("TextInstruction", Span::call_site())) );
        let mut inside_braces = TokenStream::new();
        inside_braces.append( TokenTree::Ident( Ident::new("list", Span::call_site())) );
        inside_braces.append( TokenTree::Punct( Punct::new(':', Spacing::Alone) ) );
        inside_braces.append( TokenTree::Ident( Ident::new("vec", Span::call_site()) ) );
        inside_braces.append( TokenTree::Punct( Punct::new('!', Spacing::Joint) ) );
        let mut item_list = TokenStream::new();
        item_list.append_separated( self.list.iter(), Punct::new(',', Spacing::Alone));
        inside_braces.append( TokenTree::Group( Group::new( Delimiter::Bracket, item_list ) ) );
        tokens.append( TokenTree::Group( Group::new( Delimiter::Brace, inside_braces ) ) );
    }
}

///full description of instruction, binary part for processor and textual representation for human
#[derive(Debug)]
pub struct Instruction<T:Num> {
    pub bin : BinaryInstruction<T>,
    pub text : TextInstruction,
}

impl<T:Num> ToTokens for Instruction<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {

        tokens.append( TokenTree::Ident( Ident::new("Instruction", Span::call_site())) );

        tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
        tokens.append( TokenTree::Punct( Punct::new(':', Spacing::Joint) ) );
        tokens.append( TokenTree::Punct( Punct::new('<', Spacing::Joint) ) );
        tokens.append( TokenTree::Ident( Ident::new(std::any::type_name::<T>(), Span::call_site())) );
        tokens.append( TokenTree::Punct( Punct::new('>', Spacing::Joint) ) );

        let mut inside_braces = TokenStream::new();
        inside_braces.append( TokenTree::Ident( Ident::new("bin", Span::call_site())) );
        inside_braces.append( TokenTree::Punct( Punct::new(':', Spacing::Alone) ) );
        self.bin.to_tokens( &mut inside_braces );
        inside_braces.append( TokenTree::Punct( Punct::new(',', Spacing::Alone) ) );
        inside_braces.append( TokenTree::Ident( Ident::new("text", Span::call_site())) );
        inside_braces.append( TokenTree::Punct( Punct::new(':', Spacing::Alone) ) );
        self.text.to_tokens( &mut inside_braces );

        tokens.append( TokenTree::Group( Group::new( Delimiter::Brace, inside_braces ) ) );
    }
}

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