use std::{path::PathBuf, cell::RefCell};

type NodeList = RefCell<Vec<usize>>;
type OpList = RefCell<Vec<usize>>;
type FuncBodyPos = RefCell<usize>;
type MacroID = RefCell<Option<usize>>;
type TypeList = Vec<Vec<Type>>;

#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
	Func(FuncType, FuncBodyPos),
	GroupOp(String, NodeList),
	Literal(bool),
	Number(usize, usize),
	Op(String, OpList, NodeList, NodeList, MacroID),
	Reserved(String, NodeList),
	Str1(String),
	Str2(String),
	Type(Type, TypeList),
	Var(String, TypeList, NodeList, NodeList, MacroID)
}

#[derive(Clone, PartialEq, Debug)]
pub enum FuncType {
	Func(usize),
	Macro
}

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
	Array,
	Bool,
	Chan,
	Char,
	Const,
	Fraction,
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

#[derive(Clone, PartialEq, Debug)]
pub enum FunctionSection {
	ID(String),
	OpID(String),
	Arg(String, TypeList)
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Kind,
    pub pos: FilePos
}

#[derive(Clone, Debug)]
pub struct FilePos {
    pub line: usize,
    pub col: usize
}

#[derive(Clone, Debug)]
pub struct Function {
	pub structure: Vec<FunctionSection>,
	pub output: TypeList,
	pub precedence: u8
}

#[derive(Debug)]
pub struct Macro {
	pub func: usize,
	pub ret_points: Vec<usize>
//	pub depth: usize,
//	pub row: usize
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