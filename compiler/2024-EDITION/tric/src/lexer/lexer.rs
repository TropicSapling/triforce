use crate::lexer::{reader::Reader, group_handler::GroupHandler};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Literal(String, String),
	Default(String),
	UserDef(String),
	BegOpenList,
	BegList,
	EndList,
	Newline,
	Ignored
}

#[derive(Clone, PartialEq)]
pub enum Group {
	StringLiteral,
	StrTok(String),
	ChrTok(char),
	NewlinesWs,
	Whitespace,
	Default
}

pub fn tokenised(code: String) -> Vec<Token> {
	let mut tokens = vec![];

	// Init
	let mut grp_handler = GroupHandler::new();
	let mut reader      = Reader::new(code.chars());

	// Loop through code by character and form tokens
	while let Some(c) = reader.posit.next() {
		let token = reader.handle(c, &grp_handler.groups);

		if token != Token::Ignored {
			grp_handler.handle(&token); // handle `defgroup`, `deftokens`
			tokens.push(token)
		}
	}

	tokens
}
