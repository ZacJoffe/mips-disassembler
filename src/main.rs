extern crate clap;
use clap::{Arg, App};
// use std::fs;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let matches = App::new("Mips Disassembler")
        .version("0.1.0")
        .author("Zac J. <zacharyjoffe@gmail.com>")
        .about("Memes strings")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("Dissassembles a given mips binary")
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

fn disassemble() {

}
