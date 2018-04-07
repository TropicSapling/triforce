use std::{path::PathBuf, cell::RefCell};
// use std::fmt;

/* #[derive(Clone, PartialEq, Debug)]
pub enum Whitespace {
	Newline,
	CarRet,
	Tab,
	Space
} */

#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
    GroupOp(String),
    Literal(bool),
    Number(u64, u64),
    Op(String),
    Reserved(String),
    Str1(String),
    Str2(String),
    Type(Type),
    Var(String, [Type; 8])
//    Whitespace(Whitespace)
}

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
	Array,
	Bool,
	Chan,
	Char,
	Const,
	Fraction,
	Func,
	Heap,
	Int,
	List,
	Only,
	Pointer,
	Register,
	Stack,
	Unique,
	Unsigned,
	Void,
	Volatile
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Kind,
    pub pos: FilePos,
    pub children: RefCell<Vec<usize>>
}

/* impl fmt::Debug for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Token")
			.field("kind", &self.kind)
			.field("pos", &self.pos)
			.field("children", {
				let res = Vec::new();
				for child in self.children.borrow().iter() {
					res.push(match tokens[*child].kind { // How to access tokens???
						Kind::GroupOp(ref op) => *op,
						Kind::Literal(b) => b.to_string(),
						Kind::Number(int, fraction) => int.to_string() + "." + &fraction.to_string(),
						Kind::Op(ref op) => *op,
						Kind::Reserved(ref keyword) => *keyword,
						Kind::Str1(ref s) | Kind::Str2(ref s) => *s,
						Kind::Var(ref name, _) => *name,
						_ => (*child).to_string()
					});
				}
				
				&res
			})
			.finish()
	}
} */

#[derive(Clone, Debug)]
pub struct FilePos {
    pub line: usize,
    pub col: usize
}

#[derive(Debug)]
pub struct Function<'a> {
	pub name: &'a str,
	pub pos: usize,
	pub args: Vec<FunctionArg<'a>>,
	pub output: [Type; 8],
	pub precedence: u8
}

#[derive(Debug)]
pub struct FunctionArg<'a> {
	pub name: &'a str,
	pub typ: [Type; 8]
}

pub fn get_io(input: &PathBuf) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
	let mut default_out = (*input).parent().unwrap().to_path_buf();
	default_out.push("rust");
	default_out.push(input.file_name().unwrap());
	default_out.set_extension("rs");
	
	let mut default_out_dir = (*input).parent().unwrap().to_path_buf();
	default_out_dir.push("rust");
	
	let mut default_fin_out = (*input).parent().unwrap().to_path_buf();
	default_fin_out.push("bin");
	default_fin_out.push(input.file_name().unwrap());
	default_fin_out.set_extension("exe"); // TODO: Support for Linux
	
	let mut default_fin_out_dir = (*input).parent().unwrap().to_path_buf();
	default_fin_out_dir.push("bin");
	
	(default_out, default_out_dir, default_fin_out, default_fin_out_dir)
}