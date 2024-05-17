use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Token {
	Literal(String, String),
	Default(String),
	UserDef(String),
	BegOpenList,
	BegList,
	EndList,
	Newline,
	Ignored
}

#[derive(Clone, PartialEq)]
pub enum Group {
	StringLiteral,
	StrTok(String),
	ChrTok(char),
	NewlinesWs,
	Whitespace,
	Default
}

pub enum Expr {
	List(Vec<Expr>),
	Atom(Token)
}

////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////

impl fmt::Debug for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Token::Literal(a, b) => format!("Literal({a:?}, {b:?})"),
			Token::Default(x)    => format!("Default({x:?})"),
			Token::UserDef(x)    => format!("UserDef({x:?})"),
			Token::BegOpenList   => format!("BegOpenList"),
			Token::BegList       => format!("BegList"),
			Token::EndList       => format!("EndList"),
			Token::Newline       => format!("Newline"),
			Token::Ignored       => format!("Ignored")
		})
	}
}

impl fmt::Debug for Expr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Expr::List(x) => format!("List({x:#?})"),
			Expr::Atom(x) => format!("Atom({x:?})")
		})
	}
}
