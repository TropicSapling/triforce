use std::{collections::HashMap, str::Chars};
//use crate::helpers::Substr;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Default(String),
	UserDef(String),
	BegOpenList,
	BegList,
	EndList,
	Newline
}

#[derive(Clone, PartialEq)]
enum Group {
	Default,
	NewlineWs,
	Whitespace,
	Custom(HashMap<String, Token>)
}

struct Cursor<'a> {
	group: Group,
	posit: Chars<'a>,
	token: Token
}

impl Cursor<'_> {
	fn handle(&mut self, groups: &Vec<Group>, c: char) -> Option<Token> {
		let new_group = groups.iter().find(|g| match g {
			Group::Custom(map) => map.keys().any(|s| s.starts_with(c)),
			Group::NewlineWs   => {
				c == '\n' || (c.is_whitespace() && self.group == Group::NewlineWs)
			}
			Group::Whitespace  => c.is_whitespace(),
			Group::Default     => true
		}).unwrap();

		if self.group == *new_group {
			// Extend current token
			match &mut self.token {
				Token::Default(ref mut tokstr) |
				Token::UserDef(ref mut tokstr) => tokstr.push(c),
				_                              => ()
			}

			None
		} else {
			// Switch group, return finished token, create new token
			let finished_token = self.token.clone();

			self.group = new_group.clone();
			match new_group {
				Group::NewlineWs  => self.token = Token::Newline,
				Group::Whitespace => self.token = Token::Default(String::new()),
				Group::Default    => self.token = Token::Default(c.to_string()),
				Group::Custom(_)  => self.token = Token::UserDef(c.to_string())
			}

			if finished_token != Token::Default(String::new()) {
				Some(finished_token)
			} else {
				None
			}
		}
	}
}

// Tokenises based on "maximal munch"
pub fn tokenised(code: String) -> Vec<Token> {
	let mut tokens: Vec<Token> = vec![];
	let     groups: Vec<Group> = vec![
		Group::Custom(HashMap::from([
			(String::from("(>>>"), Token::BegOpenList),
			(String::from("("   ), Token::BegList    ),
			(String::from(")"   ), Token::EndList    )
		])),
		Group::NewlineWs,
		Group::Whitespace,
		Group::Default
	];

	let mut cursor = Cursor {
		group: Group::Default,
		posit: code.chars(),
		token: Token::Default(String::new())
	};

	while let Some(c) = cursor.posit.next() {
		if let Some(tok) = cursor.handle(&groups, c) {
			tokens.push(tok)
		}
	}

	tokens
}
/*
// Tokenises based on "maximal munch"
pub fn tokenised(code: String) -> Vec<Token> {
	let mut tokens: Vec<Token> = vec![];

	let standalones = HashMap::from([
		("(>>>" , Token::BegOpenList),
		("("    , Token::BegList    ),
		(")"    , Token::EndList    )
	]);

	/*let groups = vec![
		Group::Default,
		Group::NewlineWs,
		Group::Whitespace
	];*/

	let mut superpos: Vec<String> = vec![];
	let mut superend = (usize::MAX, usize::MAX);

	let mut comment  = false;
	let mut group    = Group::Default;
	let mut it       = code.chars().peekable();
	let mut nxt_it   = it.clone();
	while let Some(c) = it.next() {
		if comment && c != '\n' {continue}

		comment = false;
		match group {
			Group::Default => if c == '\n' || c.is_whitespace() {
				if let Some(c) = it.peek() {
					if c.is_whitespace() {
						group = if *c == '\n' {Group::NewlineWs} else {Group::Whitespace}
					}
				}

				if superpos.len() > 0 {
					if superend.0 == usize::MAX {
						tokens.push(Token::Default(superpos[0].clone()))
					} else {
						if superend.0 > 0 {
							tokens.push(Token::Default(superpos[0].substr(..superend.0)))
						}

						tokens.push(standalones.get(superpos[superend.0].substr(..superend.1).as_str()).unwrap().clone());
						it = nxt_it.clone();
						group = Group::Default
					}
				}

				if c == '\n' {tokens.push(Token::Newline)}

				superpos = vec![];
				superend = (usize::MAX, usize::MAX)
			} else {
				if superend.0 == usize::MAX {
					superpos.push(String::new())
				} else {
					superpos.truncate(superend.0 + 1)
				}

				for (i, alt) in superpos.iter_mut().enumerate() {
					alt.push(c);
					if standalones.get(alt.as_str()).is_some() {
						superend = (i, alt.len());
						nxt_it   = it.clone()
					} else if alt == "//" {
						comment = true
					}
				}

				if comment {
					superpos = vec![];
					superend = (usize::MAX, usize::MAX)
				}
			}

			Group::NewlineWs => {
				if tokens[tokens.len() - 1] != Token::Newline {
					tokens.push(Token::Newline)
				}

				if let Some(c) = it.peek() {
					if !c.is_whitespace() {
						group = Group::Default
					}
				}
			}

			Group::Whitespace => {
				if c == '\n' {
					group = Group::NewlineWs;
					if tokens[tokens.len() - 1] != Token::Newline {
						tokens.push(Token::Newline)
					}
				}

				if let Some(c) = it.peek() {
					if !c.is_whitespace() {
						group = Group::Default
					}
				}
			}

			_ => todo!()
		}
	}

	tokens
}*/
