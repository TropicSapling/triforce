use std::collections::HashMap;
use crate::enums::{Expr, Token::*, Cmd::*};

pub struct Reducer {
	env: HashMap<Expr, Expr>
}

impl Reducer {
	pub fn new() -> Self {
		Reducer {env: HashMap::new()}
	}

	pub fn reduced(&mut self, expr: &Expr) -> Expr {
		// If variable, replace with its value
		if let Some(val) = self.env.get(expr) {
			return val.clone()
		}

		// Otherwise, check if it's a command
		match expr {
			// Non-empty list => possible command
			Expr::List(args) if !args.is_empty() => {
				let head = self.reduced(&args[0]);

				match head {
					Expr::Atom(ref tok) => match tok {
						Special(Defgroup) |
						Special(Deftoken) => self.reduced(&args[2]),
						Special(MacroFun) => self.reduced_mac(args),
						_                 => error!("undefined token `{tok:?}`")
					}

					_ => head
				}
			}

			// Empty list or atom => return as-is
			_ => expr.clone()
		}
	}

	fn reduced_mac(&mut self, args: &[Expr]) -> Expr {
		// Insert new variable binding into environment
		self.env.insert(ident(&args[1]).clone(), args[3].clone());

		// Reduce the macro body
		let expanded_macro = self.reduced(&args[2]);

		// Remove variable binding to restore environment
		self.env.remove(&args[1]);

		expanded_macro
	}
}

fn ident(expr: &Expr) -> &Expr {
	match expr {
		Expr::List(list) if !list.is_empty() => {
			let _type = &list[1..]; // currently unused

			&list[0]
		}

		_ => error!("invalid Λ param `{expr:?}`")
	}
}
