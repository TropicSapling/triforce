use crate::lexer::lexer::Token;

#[derive(Debug)]
pub enum Expr {
	List(Vec<Expr>),
	Atom(Token)
}

pub fn parsed(tokens: Vec<Token>) -> Expr {
	use Expr::*;

	// TODO ...

	List(vec![Atom(tokens[0].clone())]) // tmp placeholder
}
