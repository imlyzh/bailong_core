mod ast;
mod pyast_filter;
mod type_system;
mod error;
mod utils;

use std::io::{Read, Write};
use parser::parse;
use rustpython_parser::{parser, mode::Mode};


fn main() {
    // print!(">>> ");
	// std::io::stdout().flush().unwrap();
	// let mut src = String::new();
	// std::io::stdin().read_line(&mut src).unwrap();
	// println!(": {:?}", parse(&src, Mode::Program))
}
