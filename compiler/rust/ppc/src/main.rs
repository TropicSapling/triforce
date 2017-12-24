extern crate clap;

use clap::{Arg, App};

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
		.get_matches();
	
	let input = matches.value_of("input").unwrap();
	let default_out = &get_default_output(input);
	let output = matches.value_of("output").unwrap_or(default_out);
	
	println!("IN: {}", input);
	println!("OUT: {}", output);
}