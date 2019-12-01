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
    }
}


fn disassemble() {

}
