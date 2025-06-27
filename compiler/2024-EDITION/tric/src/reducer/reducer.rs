use std::collections::HashMap;
use crate::enums::{Expr::{self, *}, Token::{self, *}, Cmd::*};

pub struct Reducer {
	env: HashMap<Expr, Expr>
}

impl Reducer {
	pub fn new() -> Self {
		Self {env: HashMap::new()}
	}

	pub fn reduced(&mut self, expr: &Expr) -> Expr {
		// If variable, replace with its value
		if let Some(val) = self.env.get(expr) {
			return val.clone()
		}

		match expr {
			// Non-empty list => possible command or function application
			List(args) if !args.is_empty() => {
				let head = self.reduced(&args[0]);

				match &head {
					// Non-empty list as prefix => possible lambda application
					List(pars) if !pars.is_empty() => match &pars[0] {
						// MacroFun application
						Atom(tok) if is_mapp(tok, args) => self.reduced_mac(&pars, args),

						// Other prefix => return as-is
						_ => head
					}

					// Def-commands reduce to their bodies
					Atom(tok) if is_defcmd(tok) => self.reduced(&args[2]),

					// Unapplied MacroFun reduces to itself
					Atom(tok) if *tok == Special(MacroFun) => expr.clone(),

					// Anything else => return as-is
					_ => head
				}
			}

			// Empty list or atom => return as-is
			_ => expr.clone()
		}
	}

	fn reduced_mac(&mut self, pars: &[Expr], args: &[Expr]) -> Expr {
		// Insert new variable binding into environment
		self.env.insert(ident(&pars[1]), val(&args[1]));

		// Reduce the macro body
		let expanded_macro = self.reduced(&pars[2]);

		// Remove variable binding to restore environment
		self.env.remove(&pars[1]);

		expanded_macro
	}
}

fn is_mapp(tok: &Token, args: &[Expr]) -> bool {
	*tok == Special(MacroFun) && args.len() > 1 && args[1] != Atom(Newline)
}

fn is_defcmd(tok: &Token) -> bool {
	*tok == Special(Defgroup) || *tok == Special(Deftoken)
}

fn ident(expr: &Expr) -> Expr {
	match expr {
		List(list) if !list.is_empty() => {
			let _type = List(list[1..].to_vec()); // currently unused

			list[0].clone()
		}

		_ => error!("invalid Λ param `{expr:?}`")
	}
}

fn val(expr: &Expr) -> Expr {
	match expr {
		List(_) => expr.clone(),
		atom    => List(vec![atom.clone()])
	}
}
