extern crate clap;
extern crate term_painter;

mod lib;
mod lexer;
mod compiler;

use clap::{Arg, App};

use term_painter::{ToStyle, Color::*};

use std::{
	fs,
	fs::File,
	io::{
		prelude::*,
		ErrorKind::{NotFound, PermissionDenied}
	},
	process::Command,
	path::PathBuf,
	str
};

use lib::get_io;
use lexer::{lex, lex2, lex3};
use compiler::{parse, parse2, compile};

fn count_newlines(s: &str) -> usize {
	s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

fn main() {
	let status = init();
	
	if status != 0 {
		println!("\nProcess exited with error code {}.", status);
	}
}

fn init() -> i32 {
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
		Err(e) => if !input.extension().is_some() {
			input.set_extension("ppl");
			
			match File::open(&input) {
				Ok(file) => file,
				Err(err) => match err.kind() {
					NotFound => {
						println!("{} File not found: {:?}.", BrightRed.paint("[ERROR]"), input);
						return 1;
					},
					PermissionDenied => {
						println!("{} Access denied when trying to open file {:?}.", BrightRed.paint("[ERROR]"), input);
						return 2;
					},
					_ => panic!("failed to open file")
				}
			}
		} else {
			match e.kind() {
				NotFound => {
					println!("{} File not found: {:?}.", BrightRed.paint("[ERROR]"), input);
					return 1;
				},
				PermissionDenied => {
					println!("{} Access denied when trying to open file {:?}.", BrightRed.paint("[ERROR]"), input);
					return 2;
				},
				_ => panic!("failed to open file")
			}
		},
		Ok(t) => t
	};
	
	let mut in_contents = String::from("
		#[allow(unused)]
		func (int a) ++ -> int {
			a + 1 // TMP
		}
		
		#[allow(unused)]
		func (int a) -- -> int {
			a - 1 // TMP
		}
	");
	
	let line_offset = count_newlines(&in_contents);
	
	match in_file.read_to_string(&mut in_contents) {
		Ok(t) => t,
		Err(_e) => {
			println!("{} Failed to read file {:?}; make sure the file contains valid UTF-8 data.", BrightRed.paint("[ERROR]"), input);
			return 3;
		}
	};
	
	
	//////// LEX, PARSE & COMPILE ////////
	
	let lexed_contents = lex(&in_contents);
	if debugging {
//		println!("{} LEX1: {:#?}\n", BrightYellow.paint("[DEBUG]"), lexed_contents);
	}
	
	let mut tokens = lex2(lexed_contents, line_offset);
	if debugging {
//		println!("{} LEX2: {:#?}\n", BrightYellow.paint("[DEBUG]"), tokens);
	}
	
	lex3(&mut tokens);
	if debugging {
//		println!("{} LEX3: {:#?}\n", BrightYellow.paint("[DEBUG]"), tokens);
	}
	
	// These strings would not be necessary if Rust had <scope> or <lifetime> properties like P+, but oh well...
	let func_name_a = String::from("a");
	let func_name_b = String::from("b");
	
	let functions = parse(&tokens, &func_name_a, &func_name_b);
	
	let mut i = 0;
	while i < tokens.len() {
		parse2(&tokens, &functions, &mut i);
		
		i += 1;
	}
	
	if debugging {
		println!("{} PARSE: {:#?}\n", BrightYellow.paint("[DEBUG]"), tokens);
	}
	
	let mut out_contents = String::new();
	let mut taken = Vec::new();
	let mut i = 0;
	let mut func_def = false;
	while i < tokens.len() {
		out_contents = compile(&tokens, &functions, &mut i, &mut func_def, out_contents, &mut taken);
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
	
	0
}
