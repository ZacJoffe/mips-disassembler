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

        println!("{:08x?}", instrs)
    }
}

fn convert_bytes(bytes: Vec<u8>) -> Vec<u32> {
    let mut instrs: Vec<u32> = Vec::new();

    let mut i = 0;

    while (i < bytes.len()) {
        let instr: u32 = (((bytes[i] as u32) << 24) | ((bytes[i + 1] as u32) << 16) | ((bytes[i + 2] as u32) << 8) | bytes[i + 3] as u32) as u32;
        instrs.push(instr);
        i += 4;
    }

    instrs
}

fn disassemble(instrs: Vec<u32>) -> Vec<String> {
    let mut mips: Vec<String> = Vec::new();

    for instr in instrs {
        match instr {
            _ => panic!{"Invalid binary"}
        }
    }

    mips
}
