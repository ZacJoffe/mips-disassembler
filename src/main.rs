extern crate clap;
use clap::{Arg, App};
use std::fs;
// use std::fs::File;

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
        let data = fs::read_to_string(f).expect("Unable to open file");
    }
}
