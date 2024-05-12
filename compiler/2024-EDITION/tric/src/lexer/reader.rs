use std::str::Chars;
use crate::lexer::lexer::{Group, Token};

pub struct Reader<'a> {
	pub group: Group,
	pub posit: Chars<'a>,
	pub token: Token
}

impl Reader<'_> {
	pub fn new(posit: Chars) -> Reader {
		Reader {
			group: Group::Whitespace,
			posit,
			token: Token::Ignored
		}
	}

	/// Extends or completes a token with the given character.
	pub fn handle(&mut self, c: char, groups: &Vec<Group>) -> Token {
		let c = self.skip_if_comment(c);

		match self.group_of(c, groups) {
			g @ Group::ChrTok(_) => self.complete_token_and_init_next(c, g),
			g if self.group != g => self.complete_token_and_init_next(c, g),
			_                    => self.extend_token(c)
		}
	}

	fn group_of(&self, c: char, groups: &Vec<Group>) -> Group {
		use Group::*;

		// Assign a group the character qualifies into (if any)
		let group = groups.iter().find(|g| match g {
			StrTok(syms) => syms.contains(c),
			ChrTok(sym)  => c == *sym,
			NewlinesWs   => c.is_whitespace() && self.group == NewlinesWs || c == '\n',
			Whitespace   => c.is_whitespace(),
			_            => unreachable!()
		});

		// If a group was found, return it - otherwise return the default group
		match group {
			Some(g) => g.clone(),
			None    => Default
		}
	}

	fn extend_token(&mut self, c: char) -> Token {
		match &mut self.token {
			Token::Default(ref mut tokstr) |
			Token::UserDef(ref mut tokstr) => tokstr.push(c),

			_ => () // do nothing for non-extendable tokens
		}

		Token::Ignored
	}

	fn complete_token_and_init_next(&mut self, c: char, new_group: Group) -> Token {
		let finished_token = self.token.clone();

		// Switch group and begin formation of new token
		self.group = new_group;
		match self.group {
			Group::ChrTok('(') => self.token = self.beglist(),
			Group::ChrTok(')') => self.token = Token::EndList,
			Group::ChrTok(_)   |
			Group::StrTok(_)   => self.token = Token::UserDef(c.to_string()),
			Group::Default     => self.token = Token::Default(c.to_string()),
			Group::NewlinesWs  => self.token = Token::Newline,
			Group::Whitespace  => self.token = Token::Ignored
		}

		finished_token
	}

	fn skip_if_comment(&mut self, c: char) -> char {
		if c == '/' && self.posit.as_str().starts_with("/") {
			self.posit.find(|c| *c == '\n');
			'\n'
		} else {
			c
		}
	}

	/// Decides if list is of open form or not; returns corresponding token.
	fn beglist(&mut self) -> Token {
		if self.posit.as_str().starts_with(">>>") {
			self.posit.nth(2);
			Token::BegOpenList
		} else {
			Token::BegList
		}
	}
}
