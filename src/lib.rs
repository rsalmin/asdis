use proc_macro::{TokenStream, TokenTree, Delimiter};
use quote::quote;

mod primitives;
use primitives::{Item, TextInstruction, BinaryInstruction, Instruction , Num, CompactType};
use std::convert::From;

fn bits_len<T:Num>( v : &Vec<Item<T>> ) -> usize {
    let mut r : usize = 0;
    for item in v {
        match item {
            Item::Bits {len, .. } => r += len,
            Item::Ident { name:_ , bitspec } => r += bitspec.len() ,
        };
    };
    r
}

//fn delimiter_string(d : &proc_macro::Delimiter) -> &str {
//    match d {
//        Delimiter::Parenthesis => "()",
//        Delimiter::Brace => "{}",
//        Delimiter::Bracket => "[]",
//        Delimiter::None => "NONE",
//    }
//}

fn parse_bitspec(ts : TokenStream) -> Vec::<u32> {

    enum State {
        None,
        Val( u32 ),
        First( u32 ),
        Pair(u32, u32),
    }

    let mut bitspec  = Vec::<u32>::new();
    let mut current = State::None;

    for tt in ts {
        match tt {
            TokenTree::Group(_) => panic!("Group inside Group is not allowed"),
            TokenTree::Ident(_) => panic!("Ident inside Group is not allowed"),
            TokenTree::Punct(g) => {
                let ch = g.as_char();
                match ch {
                    '|' => {
                        match current {
                            State::Val( n ) => { bitspec.push(n); current = State::None;},
                            State::Pair(a, b) => {
                                assert!(a >= b, "bitspec pair : first integer must be >= then second");
                                for i in (b..=a).rev() {
                                    bitspec.push(i);
                                };
                                current = State::None;
                            },
                            _ =>  panic!("| misplaced"),

                        }
                    },
                    ':' => {
                        match current {
                            State::Val( a ) => current = State::First( a ),
                            _ => panic!(": misplaced"),
                        }
                    }
                    _ => panic!("Only | or : are allowed in bit specificators"),
                }
            },
            TokenTree::Literal(g) => {
                match g.to_string().parse::<u32>() {
                    Ok( n ) => match current {
                        State::None => current = State::Val( n ),
                        State::First( a ) => current = State::Pair(a, n),
                        _ => (), //panic!("Missing delimeter in bit spec"),
                    }
                    Err( err ) => panic!("Only unsigned integer allowed as bit specificators : {}", err),
                }
            },
        }
    }

    match current {
        State::Val( n ) => bitspec.push(n),
        State::Pair(a, b) => {
                                assert!(a >= b, "bitspec pair : first integer must be >= then second");
                                for i in (b..=a).rev() {
                                    bitspec.push(i);
                                };
                            },
        _ => (),
    }

    return bitspec;
}

fn parse_token_string<T:Num>(ts : TokenStream) -> Instruction<T> {

    enum State {
        Empty,
        Ident(String),
    }

    let mut current = State::Empty;
    let mut r = Vec::<Item<T>>::new();

    let mut iter = ts.into_iter();

    let text = match iter.next() {
        None => panic!("Empty token stream!"),
        Some( tt ) => match tt {
            TokenTree::Literal(g) => g.to_string() ,
            _ => panic!("First argument must be a command description"),
        }
    };
    assert!(text.len() > 2, "Quotes for command description not found");
    let text = TextInstruction::from(&text[1..text.len() - 1]); //without quotes

    for tt in iter {
        match tt {
            TokenTree::Group(g) => {
                assert!(g.delimiter() == Delimiter::Bracket, "Only [] delimeters allowed for bitspecs");
                match current {
                    State::Ident( idnt ) => {
                        let bs = parse_bitspec( g.stream() );
                        r.push( Item::Ident { name : idnt, bitspec : bs } );
                        current = State::Empty;
                    }
                    _ => panic!("Missplaced bispecs"),
                };
             },
            TokenTree::Ident(g) => {
                match current {
                    State::Empty => current = State::Ident( g.to_string() ),
                    _ => panic!("misplaced Ident"),
                }
            }
            TokenTree::Punct(g) => {
                if g.as_char() != ',' {
                    panic!("Only ',' allowed as separator of items");
                }
            },
            TokenTree::Literal(g) => {
                let str = &g.to_string();
                match T::from_str_radix(str, 2) {
                    Err( err ) => panic!("Not a binary string : {}", err),
                    Ok( val ) => r.push( Item::Bits::<T> { len : str.len(), val  } ),
                }
            },
        }
    }

    let bl = bits_len(&r);
    assert_eq!(bl, 16);

    let bin = BinaryInstruction { list : r };

    Instruction { bin, text }
}

#[proc_macro]
pub fn instruction16(items: TokenStream) -> TokenStream {
    let r  = parse_token_string::<CompactType>(items);
    TokenStream::from( quote! { #r } )
}

