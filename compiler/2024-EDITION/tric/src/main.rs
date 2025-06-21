use std::{fs, io::Error};
use culpa::throws;

#[macro_use]
mod helpers;
mod enums;
mod lexer;
mod parser;
mod reducer;

#[throws]
fn main() {
	let code   = debug!(fs::read_to_string("../simple.tri")?);
	let tokens = debug!(lexer::lexer::tokenised(code));
	let expr   = debug!(parser::parser::parsed(tokens));

	debug!(reducer::reducer::reduced(expr));
}
