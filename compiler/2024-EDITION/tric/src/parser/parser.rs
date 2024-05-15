use crate::lexer::lexer::Token;

#[derive(Debug)]
pub enum Expr {
	List(Vec<Expr>),
	Atom(Token)
}

fn parsed_list(posit: &mut impl Iterator<Item = Token>) -> Expr {
	let mut list = vec![];

	let mut param = false;
	while let Some(token) = posit.next() {
		match token {
			Token::Default(tokstr) |
			Token::Default(tokstr) if tokstr == "Λ" || tokstr == "λ" => param = true,

			Token::BegOpenList => {
				list.push(parsed_list(posit));
				if param {
					param = false
				} else {
					break
				}
			}

			Token::BegList => {
				list.push(parsed_list(posit));
				param = false
			}

			Token::EndList => {
				param = false;
				break
			}

			_ => {
				list.push(Expr::Atom(token));
				param = false
			}
		}
	}

	Expr::List(list)
}

pub fn parsed(tokens: Vec<Token>) -> Expr {
	if tokens.len() > 1 {
		parsed_list(&mut tokens.into_iter())
	} else {
		Expr::List(vec![Expr::Atom(tokens[0].clone())])
	}
}
