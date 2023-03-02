extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use structopt::StructOpt;
use options::Options;
use std::fs;
use std::io;


#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct RoxParser;

mod options;

fn repl() {
    let stdin = io::stdin();
    let mut buf = String::new();
    loop {
        stdin.read_line(&mut buf).expect("cannot read from stdin");
        if let Ok(program) = RoxParser::parse(Rule::program, &buf) {
            dbg!(&program);
        } else {
            println!("Error parsing program");
        }
        buf.clear();
    }
}


fn main() {
    let options = Options::from_args();
    if let Some(filename) = options.filename {
        let unparsed_file = fs::read_to_string(filename).expect("cannot read file");
        let program = RoxParser::parse(Rule::program, &unparsed_file).unwrap_or_else(|e| panic!("{}", e));
        dbg!(&program);
    } else {
        repl();
    }
}
