use std::str::Chars;

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
enum Group {
	StrTok(String),
	ChrTok(char),
	Whitespace,
	NewlineWs,
	Default
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
			Group::StrTok(s) => s.contains(c),
			Group::ChrTok(r) => c == *r,

			Group::NewlineWs => {
				c == '\n' || (c.is_whitespace() && self.group == Group::NewlineWs)
			}

			Group::Whitespace => c.is_whitespace(),

			_ => false
		});

		let new_group = match new_group {
			Some(g) => g,
			None    => &Group::Default
		};

		match new_group {
			Group::ChrTok(_)              => self.complete_token(c, new_group),
			_ if self.group != *new_group => self.complete_token(c, new_group),
			_                             => self.extend_token(c)
		}
	}

	fn extend_token(&mut self, c: char) -> Token {
		match &mut self.token {
			Token::Default(ref mut tokstr) |
			Token::UserDef(ref mut tokstr) => tokstr.push(c),
			_ => ()
		}

		Token::Ignored
	}

	fn complete_token(&mut self, c: char, new_group: &Group) -> Token {
		let finished_token = self.token.clone();

		self.group = new_group.clone();
		match new_group {
			Group::ChrTok('(') => self.token = self.beglist(),
			Group::ChrTok(')') => self.token = Token::EndList,
			Group::ChrTok(_)   |
			Group::StrTok(_)   => self.token = Token::UserDef(c.to_string()),
			Group::Default     => self.token = Token::Default(c.to_string()),
			Group::NewlineWs   => self.token = Token::Newline,
			Group::Whitespace  => self.token = Token::Ignored
		}

		finished_token
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

pub fn tokenised(code: String) -> Vec<Token> {
	let mut tokens: Vec<Token> = vec![];
	let mut groups: Vec<Group> = vec![
		Group::ChrTok('('),
		Group::ChrTok(')'),
		Group::NewlineWs,
		Group::Whitespace,
		Group::Default
	];

	let mut cursor = Cursor {
		group: Group::Whitespace,
		posit: code.chars(),
		token: Token::Ignored
	};

	let mut deftokens = false;
	let mut defgroup  = false;
	while let Some(c) = cursor.posit.next() {
		let token = cursor.handle(&groups, c);
		if token != Token::Ignored {
			tokens.push(token);
			match tokens.last().unwrap() {
				Token::Default(s) if s == "defgroup"  => {
					groups.push(Group::StrTok(String::new()));
					defgroup = true;
				}

				Token::Default(s) if s == "deftokens" => deftokens = true,

				Token::Default(s) if deftokens => {
					groups.push(Group::ChrTok(s.chars().next().unwrap()))
				}

				Token::Default(s) if defgroup => {
					if let Some(Group::StrTok(ref mut tok_grp)) = groups.last_mut() {
						tok_grp.push(s.chars().next().unwrap())
					}
				}

				Token::EndList if deftokens => deftokens = false,
				Token::EndList if defgroup  => defgroup  = false,

				_ => ()
			}
		}
	}

	debug!(groups);

	tokens
}
