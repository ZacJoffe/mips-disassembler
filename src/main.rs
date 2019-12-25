use std::{
    env,
    fs::File,
    io::{prelude::*}
};

// mask "don't cares" off instruction to decode opcode
enum Masks {
    Add = 0b1111_1100_0000_0000_0000_0111_1111_1111, // add, sub, slt, sltu
    Mult = 0b1111_1100_0000_0000_1111_1111_1111_1111, // mult, multu, div, divu
    Lis = 0b1111_1111_1111_1111_0000_0111_1111_1111, // lis, mfhi, mflo
    Branch = 0b1111_1100_0000_0000_0000_0000_0000_0000, // beq, bne, lw, sw
    Jump = 0b1111_1100_0001_1111_1111_1111_1111_1111, // jr, jalr
}

// opcodes with all s, t, d, and i values masked off
// these were derived from the mips manual
enum Opcodes {
    Add = 0b0000_0000_0000_0000_0000_0000_0010_0000,
    Sub = 0b0000_0000_0000_0000_0000_0000_0010_0010,
    Mult = 0b0000_0000_0000_0000_0000_0000_0001_1000,
    Multu = 0b0000_0000_0000_0000_0000_0000_0001_1001,
    Div = 0b0000_0000_0000_0000_0000_0000_0001_1010,
    Divu = 0b0000_0000_0000_0000_0000_0000_0001_1011,
    Mfhi = 0b0000_0000_0000_0000_0000_0000_0001_0000,
    Mflo = 0b0000_0000_0000_0000_0000_0000_0001_0010,
    Lis = 0b0000_0000_0000_0000_0000_0000_0001_0100,
    Lw = 0b1000_1100_0000_0000_0000_0000_0000_0000,
    Sw = 0b1010_1100_0000_0000_0000_0000_0000_0000,
    Slt = 0b0000_0000_0000_0000_0000_0000_0010_1010,
    Sltu = 0b0000_0000_0000_0000_0000_0000_0010_1011,
    Beq = 0b0001_0000_0000_0000_0000_0000_0000_0000,
    Bne = 0b0001_0100_0000_0000_0000_0000_0000_0000,
    Jr = 0b0000_0000_0000_0000_0000_0000_0000_1000,
    Jalr = 0b0000_0000_0000_0000_0000_0000_0000_1001,
}

fn main() {
    // get args, if the file isn't found then panic
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // note: probably not the best way to do this!
        panic!{"File location not given!"};
    }

    // attempt to open the file name
    let file_name = String::from(&args[1]);

    let file = File::open(file_name);
    let file = match file {
        Ok(f) => f,
        Err(e) => panic!("Couldn't load file: {}", e)
    };

    // get a vector of bytes
    let mut bytes: Vec<u8> = Vec::new();
    for byte in file.bytes() {
        bytes.push(byte.unwrap());
    }

    let instrs: Vec<u32> = convert_bytes(bytes);
    let mips: Vec<String> = disassemble(instrs);

    print_mips(mips);
}

// coalesce 4 bytes to form a vector of u32, with each elem representing an instruction
fn convert_bytes(bytes: Vec<u8>) -> Vec<u32> {
    // create empty vector and looping var
    let mut instrs: Vec<u32> = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        // join 4 contiguous bytes
        let instr: u32 = (((bytes[i] as u32) << 24) | ((bytes[i + 1] as u32) << 16) | ((bytes[i + 2] as u32) << 8) | bytes[i + 3] as u32) as u32;
        instrs.push(instr);
        i += 4;
    }

    instrs
}

// disassembles the instructions
fn disassemble(instrs: Vec<u32>) -> Vec<String> {
    let mut mips: Vec<String> = Vec::new();

    // loop through given instructions, use pattern matching to determine the instruction
    // append generated code to the mips vector
    for instr in instrs {
        // calculate opcode "constants"
        // we don't necessarily need these for every instruction but it cleans up the code doing it once
        let d: u8 = (instr >> 11) as u8 & 0x1f;
        let s: u8 = (instr >> 21) as u8 & 0x1f;
        let t: u8 = (instr >> 16) as u8 & 0x1f;
        let i: i16 = instr as i16;

        match instr {
            x if x & Masks::Add as u32 == Opcodes::Add as u32 => {
                mips.push(format!("add ${}, ${}, ${}", d, s, t));
            }
            x if x & Masks::Add as u32 == Opcodes::Sub as u32 => {
                mips.push(format!("sub ${}, ${}, ${}", d, s, t));
            }
            x if x & Masks::Mult as u32 == Opcodes::Mult as u32 => {
                mips.push(format!("mult ${}, ${}", s, t));
            }
            x if x & Masks::Mult as u32 == Opcodes::Multu as u32 => {
                mips.push(format!("multu ${}, ${}", s, t));
            }
            x if x & Masks::Mult as u32 == Opcodes::Div as u32 => {
                mips.push(format!("div ${}, ${}", s, t));
            }
            x if x & Masks::Mult as u32 == Opcodes::Divu as u32 => {
                mips.push(format!("divu ${}, ${}", s, t));
            }
            x if x & Masks::Lis as u32 == Opcodes::Mfhi as u32 => {
                mips.push(format!("mfhi ${}", d));
            }
            x if x & Masks::Lis as u32 == Opcodes::Mflo as u32 => {
                mips.push(format!("mflo ${}", d));
            }
            x if x & Masks::Lis as u32 == Opcodes::Lis as u32 => {
                mips.push(format!("lis ${}", d));
            }
            x if x & Masks::Branch as u32 == Opcodes::Lw as u32 => {
                mips.push(format!("lw ${}, {}(${})", t, i, s));
            }
            x if x & Masks::Branch as u32 == Opcodes::Sw as u32 => {
                mips.push(format!("sw ${}, {}(${})", t, i, s));
            }
            x if x & Masks::Add as u32 == Opcodes::Slt as u32 => {
                mips.push(format!("slt ${}, ${}, ${}", d, s, t));
            }
            x if x & Masks::Add as u32 == Opcodes::Sltu as u32 => {
                mips.push(format!("sltu ${}, ${}, ${}", d, s, t));
            }
            x if x & Masks::Branch as u32 == Opcodes::Beq as u32 => {
                mips.push(format!("beq ${}, ${}, ${}", s, t, i));
            }
            x if x & Masks::Branch as u32 == Opcodes::Bne as u32 => {
                mips.push(format!("bne ${}, ${}, ${}", s, t, i));
            }
            x if x & Masks::Jump as u32 == Opcodes::Jr as u32 => {
                mips.push(format!("jr ${}", s));
            }
            x if x & Masks::Jump as u32 == Opcodes::Jalr as u32 => {
                mips.push(format!("jalr ${}", s));
            }
            // .word
            _ => {
                // we only need the instr as a 32 bit integer for the .word directive
                // no bit shifting necessary!
                mips.push(format!(".word {}", instr as i32));
            }
        }
    }

    mips
}

// function to print the disassembled code
// we could improve this by adding address per line
fn print_mips(mips: Vec<String>) {
    for line in mips {
        println!("{}", line);
    }
}
