use std::{fs, io::Error};
use culpa::throws;

#[macro_use]
mod helpers;
mod lexer;
mod parser;

#[throws]
fn main() {
	let code = fs::read_to_string("../postcard.tri")?;
	debug!(&code);

	let tokens = lexer::lexer::tokenised(code);
	debug!(&tokens);

	let expr = parser::parser::parsed(tokens);
	debug!(expr);
}
