use structopt::StructOpt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod primitives;
use primitives::*;
mod isa;
use isa::{ISARV32C, ISARV32IM};
mod decoder;
use decoder::decode;
use std::num::ParseIntError;

enum IData {
    Word(u32),
    Half(u16),
}

struct IDataStream {
    buf_reader : BufReader<File>,
}

impl IDataStream {
    fn new(buf_reader : BufReader<File>) -> IDataStream {
        IDataStream { buf_reader }
    }
}

impl Iterator for IDataStream {
    type Item = std::io::Result<IData>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf_reader.fill_buf() {
            Err ( e ) => return Some( Err( e ) ),
            Ok ( buffer ) => {
                if buffer.is_empty() {
                    return None;
                }

                if  self.buf_reader.buffer()[0] & 3 == 3 {   //4 bytes word
                   let mut buffer = [0; 4];
                   if self.buf_reader.read_exact(&mut buffer).is_ok() {
                       return Some(Ok( IData::Word( u32::from_le_bytes(buffer) ) ) );
                   } else {
                       return None;
                   }
                } else  { // 3 bytes Half-word
                   let mut buffer = [0; 2];
                   if self.buf_reader.read_exact(&mut buffer).is_ok() {
                       return Some(Ok( IData::Half( u16::from_le_bytes(buffer) ) ) );
                   } else {
                       return None;
                   }
                }
            }
        }
    }
}


fn parse_hex(src: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(src, 16)
}

#[derive(StructOpt, Default, Clone)]
struct Cli {
    #[structopt(default_value = "prog.bin")]
    file : String,
    #[structopt(short, parse(try_from_str = parse_hex), default_value="0")]
    start_addr : u32,
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    let isa16 = ISARV32C::new();
    let isa32 = ISARV32IM::new();

    let file = File::open(&args.file)?;
    let buf_reader = BufReader::new(file);
    let idata_stream = IDataStream::new(buf_reader);

    println!("Opened file: {}", &args.file);

    let mut start_addr : u32 = args.start_addr;
    for i in idata_stream {
        let i = i?;
        match i {
            IData::Word( v ) =>  {
                let dscr = decode(v, &isa32); //translate32(v, &isa);
                println!("{:#010X} {:40} {:#010X?}  ", start_addr, dscr, v);
                start_addr += 4;
           },
           IData::Half ( v ) => {
               let dscr = decode(v, &isa16);
               println!("{:#010X} {:40}     {:#06X}", start_addr, dscr, v);
               start_addr += 2;
           },
       }
    }

    Ok(())
}
