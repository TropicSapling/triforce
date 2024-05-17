use crate::enums::{Expr, Token};

fn parsed_list(posit: &mut impl Iterator<Item = Token>) -> Expr {
	let mut list = vec![];

	while let Some(token) = posit.next() {
		match token {
			Token::BegOpenList => {
				list.push(parsed_list(posit));
				break
			}

			Token::BegList => list.push(parsed_list(posit)),
			Token::EndList => break,

			_ => list.push(Expr::Atom(token))
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
