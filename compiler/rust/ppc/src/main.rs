extern crate clap;
extern crate term_painter;

#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;

mod library;
mod lexer;
mod compiler;

use clap::{Arg, App};
use term_painter::{ToStyle, Color::*};
use kernel32::{GetConsoleMode, SetConsoleMode};

use std::{
	fs,
	fs::File,
	io::prelude::*,
	process::Command,
	path::PathBuf,
	str
};

use crate::library::{get_io, Token};
use crate::lexer::{lex, lex_ops, lex2, lex3};
use crate::compiler::{def_functions, parse, parse2, parse3, compile};

fn count_newlines(s: &str) -> usize {
	s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

fn get_tok_offset(tokens: &Vec<Token>, line_offset: usize) -> usize {
	let mut i = 0;
	while i < tokens.len() {
		if tokens[i].pos.line > line_offset {
			break;
		}
		
		i += 1;
	}
	
	i
}

fn main() -> Result<(), std::io::Error> {
	let matches = App::new("ppc")
		.version("0.9.1-alpha")
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
		.arg(Arg::with_name("optimise")
			.short("O")
			.help("Optimises executable"))
		.get_matches();
	
	let debugging = matches.is_present("debug");
	
	let mut input = PathBuf::from(matches.value_of("input").unwrap());
	
	if cfg!(target_os = "windows") {
		// Makes sure colours are displayed correctly on Windows
		
		unsafe {
			let handle = kernel32::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
			let mut mode = 0;
			GetConsoleMode(handle, &mut mode);
			SetConsoleMode(handle, mode | 0x0004);
		}
	}
	
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
			
			File::open(&input)?
		} else {
			return Err(e);
		},
		
		Ok(t) => t
	};
	
	let mut in_contents = String::from("();
		operator +;
		operator -;
		operator *;
		operator /;
		operator %;
		operator =;
		operator &;
		operator |;
		operator ^;
		operator <;
		operator >;
		operator !;
		operator ~;
		operator ?;
		operator :;
		operator .;
		operator ,;
		operator @;
		
		#precedence 242
		macro (int a) != (int b) {
			return !(a == b);
		}
		
		#precedence 243
		macro (int a) <= (int b) {
			return a < b || a == b;
		}
		
		#precedence 243
		macro (int a) >= (int b) {
			return a > b || a == b;
		}
		
		#precedence 0
		macro (int var) += (int n) {
			return var = var + n;
		}
		
		#precedence 0
		macro (int var) -= (int n) {
			return var = var - n;
		}
		
		#precedence 0
		macro (int var) *= (int n) {
			return var = var * n;
		}
		
		#precedence 0
		macro (int var) /= (int n) {
			return var = var / n;
		}
		
		#precedence 0
		macro (int var) %= (int n) {
			return var = var % n;
		}
		
		#precedence 0
		macro (int var) >>= (int n) {
			return var = var >> n;
		}
		
		#precedence 0
		macro (int var) <<= (int n) {
			return var = var << n;
		}
		
		#precedence 0
		macro (int var) ^= (int n) {
			return var = var ^ n;
		}
		
		macro (int a)++ {
			return {
				a += 1;
				a - 1
			};
		}
		
		macro ++(int a) {
			return {
				a += 1;
				a
			};
		}
		
		macro (int a)-- {
			return {
				a -= 1;
				a + 1
			};
		}
		
		macro --(int a) {
			return {
				a -= 1;
				a
			};
		}
		
		macro if (bool cond) (int body) {
			return unsafe {
				let res = __uninit__; // '__uninit__' won't be necessary in the future
				cond && (res = body);
				res
			};
		}
		
		macro if (bool cond) (int body) else (int expr) {
			return unsafe {
				let res = __uninit__; // '__uninit__' won't be necessary in the future
				cond && (res = body) || (res = expr);
				res
			};
		}
		
		#[allow(unused)]
		#precedence 247
		func (int base) ** (unsigned int exp) -> int {
			if exp == 0
				1
			else if exp % 2 == 0
				base ** (exp / 2) * base ** (exp / 2)
			else
				base * base ** (exp / 2) * base ** (exp / 2)
		}
	");
	
	let line_offset = count_newlines(&in_contents);
	
	in_file.read_to_string(&mut in_contents)?;
	
	//////// LEX, PARSE & COMPILE ////////
	
	let lexed_contents = lex(&in_contents);
	if debugging {
//		println!("{} LEX1: {:#?}\n", BrightYellow.paint("[DEBUG]"), lexed_contents);
	}
	
	let (lexed_contents, ops) = lex_ops(lexed_contents);
	
	let mut tokens = lex2(lexed_contents, line_offset, &ops);
	if debugging {
//		println!("{} LEX2: {:#?}\n", BrightYellow.paint("[DEBUG]"), tokens);
	}
	
	lex3(&mut tokens);
	
	if debugging {
//		println!("{} LEX3: {:#?}\n", BrightYellow.paint("[DEBUG]"), tokens);
	}
	
	let mut functions = def_functions();
	let mut macros;
	match parse(&mut tokens, functions) {
		(f, m) => {
			functions = f;
			macros = m;
		}
	}
	
	let mut all_children = Vec::new();
	let mut i = 0;
	while i < tokens.len() {
		parse2(&mut tokens, &functions, &macros, &mut all_children, &mut i, debugging);
		i += 1;
	}
	
	let tokens_len = tokens.len();
	let tok_offset = get_tok_offset(&tokens, line_offset);
//	let mut depth = 0;
//	let mut rows = vec![0];
	let mut i = 0;
	while i < tokens_len {
		parse3(&mut tokens, &mut macros, &functions, &mut i, tok_offset)?;
		i += 1;
	}
	
	if debugging {
//		println!("{} PARSE: {:#?}\n", BrightYellow.paint("[DEBUG]"), tokens);
	}
	
	let mut out_contents = String::new();
	let mut i = 0;
	while i < tokens_len {
		out_contents = compile(&tokens, &functions, &mut i, out_contents);
		i += 1;
	}
	
	//////// CREATE RUST OUTPUT ////////
	
	if debugging {
		println!("{} OUTPUT DIR: {:?}", BrightYellow.paint("[DEBUG]"), output_dir);
		println!("{} OUTPUT FILE: {:?}", BrightYellow.paint("[DEBUG]"), output);
	}
	
	fs::create_dir_all(&output_dir)?;
	
	let mut out_file = File::create(output)?;
	out_file.write_all(out_contents.as_bytes())?;
	
	Command::new("rustfmt").arg(output).output().expect("failed to format Rust code");
	
	//////// CREATE BINARY OUTPUT ////////
	
	let mut error = false;
	
	if !matches.is_present("rust") || matches.is_present("run") {
		if debugging {
			println!("{} FINAL OUTPUT DIR: {:?}", BrightYellow.paint("[DEBUG]"), final_output_dir);
			println!("{} FINAL OUTPUT FILE: {:?}", BrightYellow.paint("[DEBUG]"), final_output);
		}
		
		fs::create_dir_all(&final_output_dir)?;
		
		let out = if matches.is_present("optimise") {
			Command::new("rustc")
				.args(&["-O", "--color", "always", "-A", "unused_parens", "-A", "unused_must_use", "-A", "unused_unsafe", "-A", "unreachable_code", "-A", "unused_mut", "--out-dir", &final_output_dir, &output])
				.output()
				.expect("failed to compile Rust code")
		} else {
			Command::new("rustc")
				.args(&["-g", "--color", "always", "-A", "unused_parens", "-A", "unused_must_use", "-A", "unused_unsafe", "-A", "unreachable_code", "-A", "unused_mut", "--out-dir", &final_output_dir, &output])
				.output()
				.expect("failed to compile Rust code")
		};
		
		if out.stdout.len() > 0 {
			print!("{}", str::from_utf8(&out.stdout).unwrap());
		}
		
		if out.stderr.len() > 0 {
			print!("{}", str::from_utf8(&out.stderr).unwrap());
			
			if !out.stderr.starts_with(b"\x1b[0m\x1b[1m\x1b[38;5;11mwarning") {
				error = true;
			}
		}
	}
	
	//////// RUN COMPILED BINARY ////////
	
	if matches.is_present("run") && !error {
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
		fs::remove_file(&output)?;
//		fs::remove_dir(&output_dir)?; // Doesn't work (on Windows) for some reason?
	}
	
	Ok(())
}
