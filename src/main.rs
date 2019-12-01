extern crate clap;
use clap::{Arg, App};
// use std::fs;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

// mask don't cares off instruction to determine opcode
enum Masks {
    Add = 0b1111_1100_0000_0000_0000_0111_1111_1111, // add, sub, slt, sltu
    // Sub = 0b1111_1100_0000_0000_0000_0111_1111_1111,
    // Slt = 0b1111_1100_0000_0000_0000_0111_1111_1111,
    // Sltu,
    Mult = 0b1111_1100_0000_0000_1111_1111_1111_1111, // mult, multu, div, divu
    // Multu,
    // Div,
    // Divu,
    Lis = 0b1111_1111_1111_1111_0000_0111_1111_1111, // lis, mfhi, mflo
    // Mfhi,
    // Mflow,
    Branch = 0b1111_1100_0000_0000_0000_0000_0000_0000, // beq, bne, lw, sw
    // Lw,
    // Sw,
    // Beq,
    // Bne,
    Jump = 0b1111_1100_0001_1111_1111_1111_1111_1111, // jr, jalr
    // Jalr
}

// opcodes with all s, t, d, and i values masked off
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
    let matches = App::new("Mips Disassembler")
        .version("0.1.0")
        .author("Zac J. <zacharyjoffe@gmail.com>")
        .about("Disassembles mips")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("Disassembles a given mips binary")
            .takes_value(true),
        ).get_matches();

    if let Some(f) = matches.value_of("file") {
        let mut file = File::open(f).expect("Unable to open file");

        let mut bytes: Vec<u8> = Vec::new();
        for byte in file.bytes() {
            bytes.push(byte.unwrap());
        }

        println!("{:x?}", bytes);

        let instrs: Vec<u32> = convert_bytes(bytes);

        println!("{:08x?}", instrs);

        let mips: Vec<String> = disassemble(instrs);

        println!("{:?}", mips);
    }
}

fn convert_bytes(bytes: Vec<u8>) -> Vec<u32> {
    let mut instrs: Vec<u32> = Vec::new();

    let mut i = 0;

    while i < bytes.len() {
        let instr: u32 = (((bytes[i] as u32) << 24) | ((bytes[i + 1] as u32) << 16) | ((bytes[i + 2] as u32) << 8) | bytes[i + 3] as u32) as u32;
        instrs.push(instr);
        i += 4;
    }

    instrs
}

fn disassemble(instrs: Vec<u32>) -> Vec<String> {
    let mut mips: Vec<String> = Vec::new();

    for instr in instrs {
        let d: u8 = (instr >> 11) as u8 & 0x1f;
        let s: u8 = (instr >> 21) as u8 & 0x1f;
        let t: u8 = (instr >> 16) as u8 & 0x1f;
        let i: u16 = instr as u16;

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
                mips.push(format!(".word {}", instr));
            }
        }
    }

    mips
}
