use crate::enums::{Expr, Token, Expr::*, Token::*, Cmd::*};

fn parsed_list(posit: &mut impl Iterator<Item = Token>) -> Expr {
	let mut list = vec![];

	let mut first = true;
	while let Some(token) = posit.next() {
		match token {
			BegOpenList => {
				list.push(parsed_list(posit));
				break
			}

			BegList => list.push(parsed_list(posit)),
			EndList => break,

			Default(ref s) => match s.as_str() {
				"defgroup"  => list.push(Atom(Special(Defgroup))),
				"deftokens" => list.push(Atom(Special(Deftoken))),
				"λ"         => list.push(Atom(Special(MacroFun))),
				"Λ"         => list.push(Atom(Special(Function))),
				_           => list.push(Atom(token))
			}

			Newline if first => continue,

			_ => list.push(Atom(token))
		}

		first = false
	}

	List(list)
}

pub fn parsed(tokens: Vec<Token>) -> Expr {
	if tokens.len() > 1 {
		parsed_list(&mut tokens.into_iter())
	} else {
		List(vec![Atom(tokens[0].clone())])
	}
}
