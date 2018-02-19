use std::path::PathBuf;
use std::cell::RefCell;

#[derive(Clone, PartialEq, Debug)]
pub enum Val {
	Int(u64),
	Bool(bool),
	Str(String),
	Newline,
	CarRet,
	Tab,
	Space
}

#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
    GroupOp(Val),
    Literal(Val),
    Number(Val, Val),
    Op(Val),
    Reserved(Val),
    Str1(Val),
    Str2(Val),
    Type(Type),
    Var(Val, Type),
    Whitespace(Val)
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
	pub output: [Type; 8]
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