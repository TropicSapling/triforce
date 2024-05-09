use std::{collections::HashMap, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Default(String),
	UserDef(String),
	BegOpenList,
	BegList,
	EndList,
	Newline,
	Ignored
}

#[derive(Clone, PartialEq)]
enum Group {
	Default,
	NewlineWs,
	Whitespace,
	Custom(HashMap<String, Token>),
	BegList,
	EndList
}

struct Cursor<'a> {
	group: Group,
	posit: Chars<'a>,
	token: Token
}

impl Cursor<'_> {
	fn handle(&mut self, groups: &Vec<Group>, c: char) -> Token {
		let c = self.skip_if_comment(c);

		let new_group = groups.iter().find(|g| match g {
			Group::BegList     => c == '(',
			Group::EndList     => c == ')',
			Group::Custom(map) => map.keys().any(|s| s.starts_with(c)),
			Group::NewlineWs   => {
				c == '\n' || (c.is_whitespace() && self.group == Group::NewlineWs)
			}
			Group::Whitespace  => c.is_whitespace(),
			Group::Default     => true
		}).unwrap();

		if self.group != Group::BegList
		&& self.group != Group::EndList
		&& self.group == *new_group {
			// Extend current token
			match &mut self.token {
				Token::Default(ref mut tokstr) |
				Token::UserDef(ref mut tokstr) => tokstr.push(c),
				_                              => ()
			}

			Token::Ignored
		} else {
			// Switch group, return finished token, create new token
			let finished_token = self.token.clone();

			self.group = new_group.clone();
			match new_group {
				Group::BegList    => self.token = self.beglist(),
				Group::EndList    => self.token = Token::EndList,
				Group::NewlineWs  => self.token = Token::Newline,
				Group::Whitespace => self.token = Token::Ignored,
				Group::Default    => self.token = Token::Default(c.to_string()),
				Group::Custom(_)  => self.token = Token::UserDef(c.to_string())
			}

			finished_token
		}
	}

	fn skip_if_comment(&mut self, c: char) -> char {
		if c == '/' && self.posit.clone().next().is_some_and(|c| c == '/') {
			self.posit.find(|c| *c == '\n'); '\n'
		} else {
			c
		}
	}

	fn beglist(&mut self) -> Token {
		if self.posit.as_str().starts_with(">>>") {
			self.posit.nth(2);
			Token::BegOpenList
		} else {
			Token::BegList
		}
	}
}

// Tokenises based on "maximal munch"
pub fn tokenised(code: String) -> Vec<Token> {
	let mut tokens: Vec<Token> = vec![];
	let mut groups: Vec<Group> = vec![
		Group::BegList,
		Group::EndList,
		Group::NewlineWs,
		Group::Whitespace,
		Group::Default
	];

	let mut cursor = Cursor {
		group: Group::Whitespace,
		posit: code.chars(),
		token: Token::Ignored
	};

	while let Some(c) = cursor.posit.next() {
		let token = cursor.handle(&groups, c);
		if token != Token::Ignored {
			tokens.push(token)
		}
	}

	tokens
}
