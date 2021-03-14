use std::fmt;
use std::hash::Hash;

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
