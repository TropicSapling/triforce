use crate::lexer::lexer::{Group, Token};

pub struct GroupHandler {
	pub groups: Vec<Group>,
	
	defgroup  : bool,
	deftokens : bool
}

impl GroupHandler {
	pub fn new() -> GroupHandler {
		GroupHandler {
			groups: vec![
				Group::ChrTok('('),
				Group::ChrTok(')'),
				Group::NewlinesWs,
				Group::Whitespace
			],

			defgroup  : false,
			deftokens : false
		}
	}

	pub fn handle(&mut self, token: &Token) {
		match token {
			// -------- DEFINITION START --------

			Token::Default(s) if s == "defgroup"  => {
				self.groups.push(Group::StrTok(String::new()));
				self.defgroup = true;
			}

			Token::Default(s) if s == "deftokens" => self.deftokens = true,

			// -------- DEFINITION BODY --------

			Token::Default(s) if self.deftokens => {
				self.groups.push(Group::ChrTok(s.chars().next().unwrap()))
			}

			Token::Default(s) if self.defgroup => {
				if let Some(Group::StrTok(ref mut tok_grp)) = self.groups.last_mut() {
					tok_grp.push(s.chars().next().unwrap())
				}
			}

			// -------- DEFINITION END --------

			Token::EndList if self.deftokens => self.deftokens = false,
			Token::EndList if self.defgroup  => self.defgroup  = false,

			_ => ()
		}
	}
}
