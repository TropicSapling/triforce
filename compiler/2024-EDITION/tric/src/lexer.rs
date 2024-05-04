use std::collections::HashMap;
use crate::helpers::Substr;

#[derive(Debug, Clone)]
pub enum Token {
	Default(String),
	UserDef(String),
	BegOpenList,
	BegList,
	EndList,
	Newline
}

enum Group {
	Default,
	NewlineWs,
	Whitespace,
	Custom(HashMap<String, Token>)
}

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
			},

			Group::NewlineWs | Group::Whitespace => if let Some(c) = it.peek() {
				if !c.is_whitespace() {
					group = Group::Default
				}
			},

			_ => todo!()
		}
	}

	tokens
}
