extern crate clap;

use clap::{Arg, App};

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
	
	println!("IN: {}", matches.value_of("input").unwrap());
	println!("OUT: {}", matches.value_of("output").unwrap_or("default"));
}