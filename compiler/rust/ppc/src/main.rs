extern crate clap;
extern crate term_painter;

mod lib;

use clap::{Arg, App};

use term_painter::Color::*;
use term_painter::ToStyle;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use std::process::Command;

use std::str;

use lib::{lex, lex2, parse, compile};

fn get_dir_from_path(input: &str) -> String {
	let mut file_start = 0;
	for (i, item) in input.chars().rev().enumerate() {
		if item == '/' {
			file_start = input.len() - i;
			break;
		}
	}
	
	(&input[..file_start]).to_owned()
}

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
		.arg(Arg::with_name("run")
			.long("run")
			.help("Runs file after compiling"))
		.arg(Arg::with_name("rust")
			.short("r")
			.long("rust")
			.help("Compiles into Rust instead of executable"))
		.get_matches();
	
	let debugging = matches.is_present("debug");
	
	let input = matches.value_of("input").unwrap();
	let default_out = get_default_output(input);
	let (output, output_dir) = (matches.value_of("output").unwrap_or(&default_out), get_dir_from_path(matches.value_of("output").unwrap_or(&default_out))); // Probably can be improved performance-wise
	
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
	
	let mut parsed_tokens = parse(tokens);
	if debugging {
//		println!("{} PARSE: {:#?}\n", BrightYellow.paint("[DEBUG]"), parsed_tokens);
	}
	
	let mut out_contents = String::new();
	let mut i = 0;
	while i < parsed_tokens.len() {
		out_contents = compile(&mut parsed_tokens, &mut i, out_contents);
		i += 1;
	}
	
	if debugging {
		println!("{} OUTPUT DIR: {}", BrightYellow.paint("[DEBUG]"), output_dir);
		println!("{} OUTPUT FILE: {}", BrightYellow.paint("[DEBUG]"), output);
	}
	
	match fs::create_dir_all(&output_dir) {
		Err(e) => panic!("{}", e),
		_ => ()
	};
	
	let mut out_file = match File::create(output) {
		Err(e) => panic!("{}", e),
		Ok(t) => t
	};
	
	match out_file.write_all(out_contents.as_bytes()) {
		Err(e) => panic!("{}", e),
		_ => ()
	};
	
	let output = Command::new("rustc")
				.args(&["--out-dir", &output_dir, output]) // CHANGE '&output_dir' to final output directory ('.../bin/')
				.output()
				.expect("failed to execute process");
	
	if output.stdout.len() > 0 {
		println!("{}", str::from_utf8(&output.stdout).unwrap());
	}
	
	if output.stderr.len() > 0 {
		println!("{}", str::from_utf8(&output.stderr).unwrap());
	}
	
	if matches.is_present("run") {
		// WIP: Run file
	}
	
	// WIP: Delete Rust file unless specified not
}