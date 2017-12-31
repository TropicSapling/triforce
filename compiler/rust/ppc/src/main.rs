extern crate clap;
extern crate term_painter;

mod lib;

use clap::{Arg, App};

use term_painter::Color::*;
use term_painter::ToStyle;

use std::fs::File;
use std::io::prelude::*;

use lib::{lex, lex2, parse, compile};

fn get_default_output(input: &str) -> String {
	let mut file_start = 0;
	let mut file_end = input.len() - 1;
	
	for (i, item) in input.chars().rev().enumerate() {
		if item == '/' {
			file_start = input.len() - i;
			break;
		} else if item == '.' {
			file_end = input.len() - i - 1;
		}
	}
	
	(&input[..file_start]).to_owned() + "rust/" + &input[file_start..file_end] + ".rs"
}

fn main() {
	let matches = App::new("ppc")
		.version("0.1.0-alpha")
		.about("P+ compiler written in Rust.")
		.author("TropicSapling")
		.arg(Arg::with_name("input")
			.short("i")
			.long("input")
			.value_name("file")
			.help("Specifies an input file")
			.required(true))
		.arg(Arg::with_name("output")
			.short("o")
			.long("output")
			.value_name("file")
			.help("Specifies an output file"))
		.arg(Arg::with_name("debug")
			.short("d")
			.long("debug")
			.help("Enables debug mode"))
		.get_matches();
	
	let debugging = matches.is_present("debug");
	
	let input = matches.value_of("input").unwrap();
	let default_out = &get_default_output(input);
	let output = matches.value_of("output").unwrap_or(default_out);
	
	if debugging {
		println!("{} INPUT FILE: {}", BrightYellow.paint("[DEBUG]"), input);
	}
	
	let mut in_file = File::open(input).expect("file not found");
	let mut in_contents = String::new();
	
	in_file.read_to_string(&mut in_contents).expect("failed to read file");
	
	let lexed_contents = lex(&in_contents);
	if debugging {
		println!("{} LEX1: {:#?}\n", BrightYellow.paint("[DEBUG]"), lexed_contents);
	}
	
	let tokens = lex2(lexed_contents);
	if debugging {
		println!("{} LEX2: {:#?}\n", BrightYellow.paint("[DEBUG]"), tokens);
	}
	
	let parsed_tokens = parse(tokens);
	if debugging {
		println!("{} PARSE: {:#?}\n", BrightYellow.paint("[DEBUG]"), parsed_tokens);
	}
	
	let mut out_contents = compile(parsed_tokens);
/*	let mut out_file = File::create(output)?;
	
	out_file.write_all(out_contents); */
	if debugging {
		println!("{} OUTPUT FILE: {}", BrightYellow.paint("[DEBUG]"), output);
		println!("{} Result: {}", BrightYellow.paint("[DEBUG]"), out_contents);
	}
}