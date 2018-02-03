use std::path::PathBuf;
use std::fmt;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Type {
    GroupOp,
    Literal,
    Number,
    Op,
    Reserved,
    Str1,
    Str2,
    Type,
    Var,
    Whitespace
}

#[derive(Clone)]
pub struct Token {
    pub val: String,
    pub t: Type,
    pub pos: FilePos,
    pub parent: TokRef,
    pub children: TokRefs
}

#[derive(Clone)]
#[derive(Debug)]
pub struct FilePos {
    pub line: u32,
    pub col: u32
}

#[derive(Clone)]
pub struct TokRef(*const Token);

#[derive(Clone)]
pub struct TokRefs(*const Vec<TokRef>);

fn format<T: fmt::Debug>(f: &mut fmt::Formatter, node: *const T) -> fmt::Result {
    unsafe {
        if node.is_null() {
            write!(f, "NULL")
        } else {
            write!(f, "{:#?}", *node)
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Token")
            .field("val", &self.val)
            .field("t", &self.t)
            .field("pos", &self.pos)
            .field("children", &self.children)
            .finish()
    }
}

impl fmt::Debug for TokRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format(f, {let &TokRef(node) = self; node})
    }
}

impl fmt::Debug for TokRefs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format(f, {let &TokRefs(node) = self; node})
    }
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