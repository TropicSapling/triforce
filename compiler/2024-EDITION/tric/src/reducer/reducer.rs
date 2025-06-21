use crate::enums::{Expr, Token};

pub fn reduced(expr: &Expr) -> &Expr {
	let Expr::List(exprs) = expr else {return expr};
	
	match exprs.len() {
		0 => expr,
		_ => {
			let cmd = match exprs[0] {
				Expr::Atom(_) => &exprs[0],
				_             => reduced(&exprs[0])
			};

			match cmd {
				Expr::Atom(atom) => match atom {
					Token::Default(s) if s == "defgroup"  => reduced(&exprs[2]),
					Token::Default(s) if s == "deftokens" => reduced(&exprs[2]),
					Token::Default(s) if s == "Λ"         => reduced_lambda(exprs),

					_ => cmd
				},

				_ => cmd
			}
		}
	}
}

fn reduced_lambda(args: &[Expr]) -> &Expr {
	reduced(&args[2]) // placeholder
}
