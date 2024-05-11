use crate::lexer::{cursor::Cursor, group_handler::GroupHandler};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Default(String),
	UserDef(String),
	BegOpenList,
	BegList,
	EndList,
	Newline,
	Ignored
}

#[derive(Clone, Debug, PartialEq)]
pub enum Group {
	StrTok(String),
	ChrTok(char),
	Whitespace,
	NewlineWs,
	Default
}

pub fn tokenised(code: String) -> Vec<Token> {
	let mut tokens = vec![];

	let mut grp_handler = GroupHandler::new();
	let mut cursor      = Cursor::new(code.chars());
	while let Some(c) = cursor.posit.next() {
		let token = cursor.handle(&grp_handler.groups, c);
		if token != Token::Ignored {
			grp_handler.handle(&token);
			tokens.push(token)
		}
	}

	tokens
}
