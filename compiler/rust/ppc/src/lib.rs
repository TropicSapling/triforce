use std::{path::PathBuf, cell::RefCell};

#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
    GroupOp(String),
    Literal(bool),
    Number(usize, usize),
    Op(String),
    Reserved(String),
    Str1(String),
    Str2(String),
    Type(Type),
    Var(String, Vec<Vec<Type>>)
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
	Macro,
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

#[derive(Clone, Debug)]
pub struct FilePos {
    pub line: usize,
    pub col: usize
}

#[derive(Clone, Debug)]
pub struct Function {
	pub name: String,
	pub pos: usize,
	pub args: Vec<FunctionArg>,
	pub output: Vec<Vec<Type>>,
	pub precedence: u8
}

#[derive(Clone, Debug)]
pub struct FunctionArg {
	pub name: String,
	pub typ: Vec<Vec<Type>>
}

#[derive(Debug)]
pub struct Macro {
	pub name: Token,
	pub contents: Vec<Token>,
	pub depth: usize
}

#[derive(Debug)]
pub struct MacroFunction {
	pub func: Function,
	pub code: Vec<Token>,
	pub returns: Vec<Vec<Token>>,
	pub depth: usize,
	pub bpos: usize
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