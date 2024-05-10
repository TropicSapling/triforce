use std::{fs, io::Error};
use culpa::throws;

#[macro_use]
mod helpers;
mod lexer;

#[throws]
fn main() {
	let code = fs::read_to_string("../postcard.tri")?;

	debug!(&code);

	let tokens = lexer::tokenised(code);

	debug!(tokens);
}
