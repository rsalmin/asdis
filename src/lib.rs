use proc_macro::{TokenStream, TokenTree, Delimiter};
use std::fmt;

#[derive(Debug)]
enum Item {
    Bits {bits : String},
    Ident {name:String, bitspec:Vec<u8>},
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Bits { bits } => write!(f, "{}", bits),
            Item::Ident {name, bitspec} => write!(f, "{}{:?}", name, bitspec),
        }
    }
}


fn delimiter_string(d : &proc_macro::Delimiter) -> &str {
    match d {
        Delimiter::Parenthesis => "()",
        Delimiter::Brace => "{}",
        Delimiter::Bracket => "[]",
        Delimiter::None => "NONE",
    }
}

fn parseBitspec(ts : TokenStream) -> Vec::<u8> {

    enum State {
        None,
        Val( u8 ),
        First( u8 ),
        Pair(u8, u8),
    }

    let mut bitspec  = Vec::<u8>::new();
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
                match g.to_string().parse::<u8>() {
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

fn parseTokenString(ts : TokenStream) -> String {

    enum State {
        Empty,
        Ident(String),
    }

    let mut current = State::Empty;
    let mut r = Vec::<Item>::new();

    for tt in ts {
        match tt {
            TokenTree::Group(g) => {
                assert!(g.delimiter() == Delimiter::Bracket, "Only [] delimeters allowed for bitspecs");
                match current {
                    State::Ident( idnt ) => {
                        let bs = parseBitspec( g.stream() );
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
                r.push( Item::Bits { bits : g.to_string() } );
            },
        }
    }

    let mut str = String::new();
    for i in r {
        let s = format!("{}", i);
        str.push_str(&s);
        str.push_str(" ");
    }

    //panic!("WTF? {:?}", r);
    str
}

#[proc_macro]
pub fn instruction(items: TokenStream) -> TokenStream {
    let r = parseTokenString(items);

    let mut rr = String::from("\"");
    rr.push_str(&r);
    rr.push_str("\"");
    rr.parse().unwrap()
    //"10".parse().unwrap()
}

