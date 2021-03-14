use structopt::StructOpt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod primitives;
use primitives::*;
mod isa;
use isa::{ISAHelper, ISARV32C};
mod decoder;
use decoder::{decode, decode16};
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

fn translate32(v : u32, isa : &ISAHelper) -> String {
    //Note that all ones illegal at least for for RV32I, but may be not illegal for other extensions
    if v as u16 == 0 || v == 0xFFFFFFFF {
        return format!("<illegal>");
    }
    let op = Opcode::new( ( v & 0x7F ) as u8 );
    match isa.op2fmt.get(&op) {
        Some( ifmt ) => i2string( &decode( ifmt, v, &op ), isa),
        None => format!("(op = {})", op),
    }
}

fn i2string(i : &Instruction32, isa: &ISAHelper) -> String {
    let mn = isa.mnemonic( i );
    match i {
            Instruction32::R {func7:_, rs2, rs1, func3:_, rd, opcode:_} =>
                format!("{:8} {}, {}, {}", mn, rd, rs1, rs2),
            Instruction32::I { imm, rs1, func3:_, rd, opcode:_} =>
                format!("{:8} {}, {}, {:#X}", mn, rd, rs1, imm),
            Instruction32::IC { cst:_, rs1, func3:_, rd, opcode:_} =>
                 format!("{:8} {}, {}", mn, rd, rs1),
            Instruction32::IF { func7:_, imm, rs1, func3:_, rd, opcode:_} =>
                 format!("{:8} {}, {}, {:#X}", mn, rd, rs1, imm),
             Instruction32::S { imm, rs2, rs1, func3:_, opcode:_} =>
                format!("{:8} {}, {}, {:#X}", mn, rs1, rs2, imm),
            Instruction32::SB { imm, rs2, rs1, func3:_, opcode:_} =>
                format!("{:8} {}, {}, {:#X}", mn, rs1, rs2, imm),
            Instruction32::U { imm, rd, opcode:_} =>
                format!("{:8} {}, {:#X}", mn, rd, imm),
            Instruction32::UJ { imm, rd, opcode:_} =>
                format!("{:8} {}, {:#X}", mn, rd, imm),
        }
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    let isa = ISAHelper::new();
    let isa16 = ISARV32C::new();

    let file = File::open(&args.file)?;
    let buf_reader = BufReader::new(file);
    let idata_stream = IDataStream::new(buf_reader);

    println!("Opened file: {}", &args.file);

    let mut start_addr : u32 = args.start_addr;
    for i in idata_stream {
        let i = i?;
        match i {
            IData::Word( v ) =>  {
                let dscr = translate32(v, &isa);
                println!("{:#010X} {:40} {:#010X?}  ", start_addr, dscr, v);
                start_addr += 4;
           },
           IData::Half ( v ) => {
               let dscr = decode16(v, &isa16);
               println!("{:#010X} {:40}     {:#06X}", start_addr, dscr, v);
               start_addr += 2;
           },
       }
    }

    Ok(())
}
