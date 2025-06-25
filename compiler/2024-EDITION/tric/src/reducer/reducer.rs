use std::collections::HashMap;
use crate::enums::{Expr, Token, Cmd};

pub struct Reducer {
	env: HashMap<Expr, Expr>
}

impl Reducer {
	pub fn new() -> Self {
		Reducer {env: HashMap::new()}
	}

	pub fn reduced(&mut self, expr: Expr) -> Expr {
		let Expr::List(ref args) = expr else {return expr};
		
		if args.len() > 0 {
			let cmd = self.reduced(args[0].clone());

			match cmd {
				Expr::Atom(ref atom) => match atom {
					Token::Special(Cmd::Defgroup) => self.reduced(args[2].clone()),
					Token::Special(Cmd::Deftoken) => self.reduced(args[2].clone()),
					Token::Special(Cmd::MacroFun) => self.reduced_fun(args.clone()),

					_ => cmd
				}

				_ => cmd
			}
		} else {
			expr
		}
	}

	fn reduced_fun(&mut self, args: Vec<Expr>) -> Expr {
		self.env.insert(args[1].clone(), args[3].clone());

		let res = self.reduced(args[2].clone());

		self.env.remove(&args[1]);

		res
	}
}
