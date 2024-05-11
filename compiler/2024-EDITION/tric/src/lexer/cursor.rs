use std::str::Chars;
use crate::lexer::lexer::{Group, Token};

pub struct Cursor<'a> {
	pub group: Group,
	pub posit: Chars<'a>,
	pub token: Token
}

impl Cursor<'_> {
	pub fn new(posit: Chars) -> Cursor {
		Cursor {
			group: Group::Whitespace,
			posit,
			token: Token::Ignored
		}
	}

	pub fn handle(&mut self, groups: &Vec<Group>, c: char) -> Token {
		let c = self.skip_if_comment(c);

		match self.group_of(c, groups) {
			g @ Group::ChrTok(_)  => self.complete_token(c, g),
			g if self.group != *g => self.complete_token(c, g),
			_                     => self.extend_token(c)
		}
	}

	fn group_of<'a>(&self, c: char, groups: &'a Vec<Group>) -> &'a Group {
		use Group::*;

		let group = groups.iter().find(|g| match g {
			StrTok(syms) => syms.contains(c),
			ChrTok(sym)  => c == *sym,
			Whitespace   => c.is_whitespace(),
			NewlineWs    => c.is_whitespace() && self.group == NewlineWs || c == '\n',
			_            => false
		});

		match group {
			Some(g) => g,
			None    => &Default
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
