use std::{fs, io::Error};
use culpa::throws;

mod helpers;
mod lexer;

macro_rules! debug {
	($e:expr) => (println!("");dbg!($e))
}

#[throws]
fn main() {
	let code = fs::read_to_string("../postcard.tri")?;

	debug!(&code);

	let tokens = lexer::tokenised(code);

	debug!(tokens);
}
