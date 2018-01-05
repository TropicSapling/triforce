extern crate clap;
extern crate term_painter;

mod lib;
mod lexer;
mod compiler;

use clap::{Arg, App};

use term_painter::Color::*;
use term_painter::ToStyle;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use std::process;
use std::process::Command;
use std::path::PathBuf;

use std::str;

use lib::get_io;
use lexer::{lex, lex2};
use compiler::{parse, compile};

fn main() {
	let matches = App::new("ppc")
		.version("0.1.0-alpha")
		.about("P+ compiler written in Rust.")
		.author("TropicSapling")
		.arg(Arg::with_name("input")
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
	
	let mut input = PathBuf::from(matches.value_of("input").unwrap());
	
	if debugging {
		println!("{} INPUT FILE: {:?}", BrightYellow.paint("[DEBUG]"), input);
	}
	
	//////// GET OUTPUT PATHS ////////
	
	let io;
	
	let (output, output_dir, final_output, final_output_dir) = if matches.value_of("output").is_some() {
		io = get_io(&PathBuf::from(matches.value_of("output").unwrap()));
		(io.0.to_str().unwrap(), io.1.to_str().unwrap(), io.2.to_str().unwrap(), io.3.to_str().unwrap())
	} else {
		io = get_io(&input);
		(io.0.to_str().unwrap(), io.1.to_str().unwrap(), io.2.to_str().unwrap(), io.3.to_str().unwrap())
	};
	
	//////// OPEN INPUT FILE ////////
	
	let mut in_file = match File::open(&input) {
		Err(_e) => if !input.extension().is_some() {
			input.set_extension("ppl");
			
			match File::open(&input) {
				Ok(file) => file,
				Err(_err) => {
					println!("{} Failed to find given file, make sure the file exists. File: {:?}", BrightRed.paint("[ERROR]"), input.file_name().unwrap());
					process::exit(1);
				}
			}
		} else {
			println!("{} Failed to find given file, make sure the file exists. File: {:?}", BrightRed.paint("[ERROR]"), input.file_name().unwrap());
			process::exit(1);
		},
		Ok(t) => t
	};
	let mut in_contents = String::new();
	
	match in_file.read_to_string(&mut in_contents) {
		Ok(t) => t,
		Err(_er) => {
			println!("{} Failed to open given file, make sure the file is UTF-8. File: {:?}", BrightRed.paint("[ERROR]"), input.file_name().unwrap());
			process::exit(1);
		}
	};
	
	
	//////// LEX, PARSE & COMPILE ////////
	
	let lexed_contents = lex(&in_contents);
	if debugging {
//		println!("{} LEX1: {:#?}\n", BrightYellow.paint("[DEBUG]"), lexed_contents);
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
	
	//////// CREATE RUST OUTPUT ////////
	
	if debugging {
		println!("{} OUTPUT DIR: {:?}", BrightYellow.paint("[DEBUG]"), output_dir);
		println!("{} OUTPUT FILE: {:?}", BrightYellow.paint("[DEBUG]"), output);
	}
	
	match fs::create_dir_all(&output_dir) {
		Err(e) => panic!("{}", e),
		_ => ()
	}
	
	let mut out_file = match File::create(output) {
		Err(e) => panic!("{}", e),
		Ok(t) => t
	};
	
	match out_file.write_all(out_contents.as_bytes()) {
		Err(e) => panic!("{}", e),
		_ => ()
	}
	
	//////// CREATE BINARY OUTPUT ////////
	
	if debugging {
		println!("{} FINAL OUTPUT DIR: {:?}", BrightYellow.paint("[DEBUG]"), final_output_dir);
		println!("{} FINAL OUTPUT FILE: {:?}", BrightYellow.paint("[DEBUG]"), final_output);
	}
	
	match fs::create_dir_all(&final_output_dir) {
		Err(e) => panic!("{}", e),
		_ => ()
	}
	
	let out = Command::new("rustc")
		.args(&["--out-dir", &final_output_dir, &output])
		.output()
		.expect("failed to execute process");
	
	if out.stdout.len() > 0 {
		print!("{}", str::from_utf8(&out.stdout).unwrap());
	}
	
	if out.stderr.len() > 0 {
		print!("{}", str::from_utf8(&out.stderr).unwrap());
	}
	
	//////// RUN COMPILED BINARY ////////
	
	if matches.is_present("run") {
		let out = if cfg!(target_os = "windows") {
			Command::new(&final_output)
				.output()
				.expect("failed to execute process")
		} else {
			Command::new(String::from("./") + &final_output)
				.output()
				.expect("failed to execute process")
		};
		
		if out.stdout.len() > 0 {
			print!("{}", str::from_utf8(&out.stdout).unwrap());
		}
		
		if out.stderr.len() > 0 {
			print!("{}", str::from_utf8(&out.stderr).unwrap());
		}
	}
	
	//////// DELETE RUST FILES ////////
	
	if !matches.is_present("rust") {
		match fs::remove_file(&output) {
			Err(e) => panic!("{}", e),
			_ => ()
		}
		
/*		match fs::remove_dir(&output_dir) { // Doesn't work (on Windows) for some reason?
			Err(e) => panic!("{}", e),
			_ => ()
		} */
	}
}