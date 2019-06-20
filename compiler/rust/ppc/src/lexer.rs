use std::{usize, cell::RefCell};
use crate::library::{Token, Kind, FuncType, Type, FilePos};

fn is_var(c: char) -> bool {
	c != '{' && c != '}' && c != '[' && c != ']' && c != '(' && c != ')' && c != ';' && c != '"' && c != '\'' && c != '/' && c != '*' && c != '\\' && !c.is_whitespace()
}

pub fn lex<'a>(contents: &'a String) -> Vec<&'a str> {
	let mut result = Vec::new();
	let mut last = 0;
	for (index, matched) in contents.match_indices(|c: char| !is_var(c)) {
		if last != index {
			result.push(&contents[last..index]);
		}
		
		result.push(matched);
		
		last = index + matched.len();
	}
	
	if last < contents.len() {
		result.push(&contents[last..]);
	}
	
	result
}

pub fn lex_ops<'a>(tokens: Vec<&'a str>) -> (Vec<&'a str>, Vec<char>) {
	let mut ops = Vec::new();
	let mut new_tokens = Vec::new();
	
	let mut ignoring = false;
	let mut ignoring2 = 0;
	let mut in_str = false;
	let mut in_str2 = false;
	let mut escaping = false;
	
	let mut i = 0;
	while i < tokens.len() {
		let mut push = true;
		
		if ignoring {
			if tokens[i] == "\n" {
				ignoring = false;
			}
		} else if ignoring2 > 0 {
			if tokens[i] == "*" && tokens[i + 1] == "/" {
				ignoring2 -= 1;
				new_tokens.push(tokens[i]);
				i += 1;
			} else if tokens[i] == "/" && tokens[i + 1] == "*" {
				ignoring2 += 1;
				new_tokens.push(tokens[i]);
				i += 1;
			}
		} else if tokens[i] == "/" && tokens[i + 1] == "/" {
			ignoring = true;
			
			new_tokens.push(tokens[i]);
			i += 1;
		} else if tokens[i] == "/" && tokens[i + 1] == "*" {
			ignoring2 = 1;
			
			new_tokens.push(tokens[i]);
			i += 1;
		} else if escaping {
			escaping = false;
		} else if in_str {
			if tokens[i] == "\"" {
				in_str = false;
			} else if tokens[i] == "\\" {
				escaping = true;
			}
		} else if in_str2 {
			if tokens[i] == "'" {
				in_str2 = false;
			} else if tokens[i] == "\\" {
				escaping = true;
			}
		} else if tokens[i] == "\"" {
			in_str = true;
		} else if tokens[i] == "'" {
			in_str2 = true;
		} else if tokens[i] == "operator" {
			i += 2;
			ops.push(tokens[i].chars().next().unwrap());
			i += 1;
			
			push = false;
		} else {
			let mut indexes = Vec::new();
			for (i, c) in tokens[i].chars().enumerate() {
				if ops.contains(&c) {
					indexes.push(i);
				}
			}
			
			for (c, idx) in indexes.iter().enumerate() {
				if c == 0 {
					if *idx != 0 {
						new_tokens.push(&tokens[i][..*idx]);
					}
				} else if *idx != indexes[c - 1] + 1 {
					new_tokens.push(&tokens[i][indexes[c - 1] + 1..*idx]);
				}
				
				new_tokens.push(&tokens[i][*idx..*idx + 1]);
				
				if c == indexes.len() - 1 && *idx + 1 < tokens[i].len() {
					new_tokens.push(&tokens[i][*idx + 1..]);
				}
			}
			
			if indexes.len() == 0 {
				new_tokens.push(tokens[i]);
			}
			
			push = false;
		}
		
		if push {
			new_tokens.push(tokens[i]);
		}
		
		i += 1;
	}
	
	(new_tokens, ops)
}

pub fn lex2(tokens: Vec<&str>, line_offset: usize, ops: &Vec<char>) -> Vec<Token> {
	let mut res: Vec<Token> = Vec::new();
	let mut string = Token {
		kind: Kind::Str1(String::from("")),
		pos: FilePos {line: 1, col: 1},
	};
	
	let mut in_str = false;
	let mut in_str2 = false;
	let mut escaping = false;
	let mut ignoring = false;
	let mut ignoring2 = 0;
	let mut possible_comment = false;
	let mut possible_comment_end = false;
	
	let mut num_pos = 0;
	let mut line = 1;
	let mut col = 1;
	
	for item in tokens {
		if ignoring {
			if item == "\n" {
				line += 1;
				col = 0;
				ignoring = false;
			}
		} else if ignoring2 > 0 {
			if possible_comment_end {
				if item == "/" {
					ignoring2 -= 1;
				}
				
				if item != "*" {
					possible_comment_end = false;
				}
			} else if possible_comment {
				if item == "*" {
					ignoring2 += 1;
				}
				
				if item != "/" {
					possible_comment = false;
				}
			} else if item == "*" {
				possible_comment_end = true;
			} else if item == "/" {
				possible_comment = true;
			}
			
			if item == "\n" {
				line += 1;
				col = 0;
			}
		} else {
			if possible_comment {
				if item == "/" {
					ignoring = true;
					possible_comment = false;
					
					continue;
				} else if item == "*" {
					ignoring2 = 1;
					possible_comment = false;
					
					continue;
				} else {
					possible_comment = false;
					
					string.kind = Kind::Op(String::from("/"), RefCell::new(Vec::new()), RefCell::new(Vec::new()), RefCell::new(Vec::new()), RefCell::new(None));
					string.pos = if line > line_offset {
						FilePos {line: line - line_offset, col}
					} else {
						FilePos {line, col}
					};
					
					res.push(string.clone());
				}
			}
			
			if escaping {
				match string.kind {
					Kind::Str1(ref mut s) | Kind::Str2(ref mut s) => {
						*s += "\\";
						
						*s += item;
						string.pos = if line > line_offset {
							FilePos {line: line - line_offset, col}
						} else {
							FilePos {line, col}
						};
						
						escaping = false;
					},
					
					_ => unreachable!()
				}
			} else if in_str {
				if item == "\"" {
					res.push(string.clone());
					in_str = false;
				} else if item == "\\" {
					escaping = true;
				} else {
					match string.kind {
						Kind::Str1(ref mut s) => *s += item,
						_ => unreachable!()
					};
				}
			} else if in_str2 {
				if item == "'" {
					res.push(string.clone());
					in_str2 = false;
				} else if item == "\\" {
					escaping = true;
				} else {
					match string.kind {
						Kind::Str2(ref mut s) => *s += item,
						_ => unreachable!()
					}
				}
			} else if item == "\"" {
				string.kind = Kind::Str1(String::from(""));
				string.pos = if line > line_offset {
					FilePos {line: line - line_offset, col}
				} else {
					FilePos {line, col}
				};
				in_str = true;
			} else if item == "'" {
				string.kind = Kind::Str2(String::from(""));
				string.pos = if line > line_offset {
					FilePos {line: line - line_offset, col}
				} else {
					FilePos {line, col}
				};
				in_str2 = true;
			} else {
				if num_pos > 0 && (item == "." || num_pos == 2) {
					if num_pos == 2 {
						match string.kind {
							Kind::Number(n, _) => string.kind = Kind::Number(n, item.parse::<usize>().unwrap()),
							_ => unreachable!()
						}
						
						res.push(string.clone());
						
						num_pos = 0;
					} else {
						num_pos = 2;
					}
					
					continue;
				} else if num_pos == 1 {
					res.push(string.clone());
					
					num_pos = 0;
				}
				
				let int_res = item.parse::<usize>();
				
				if item == "/" {
					possible_comment = true;
				} else if let Ok(int_val) = int_res {
					string.kind = Kind::Number(int_val, 0);
					string.pos = if line > line_offset {
						FilePos {line: line - line_offset, col}
					} else {
						FilePos {line, col}
					};
					
					num_pos = 1;
				} else {
					string.kind = match item {
						"func" => Kind::Func(FuncType::Func(0), RefCell::new(0)),
						"macro" => Kind::Func(FuncType::Macro, RefCell::new(0)),
						"{" | "}" | "[" | "]" | "(" | ")" | ";" => Kind::GroupOp(item.to_string(), RefCell::new(Vec::new())),
						"array" => Kind::Type(Type::Array, Vec::new()),
						"bool" => Kind::Type(Type::Bool, Vec::new()),
						"chan" => Kind::Type(Type::Chan, Vec::new()),
						"char" => Kind::Type(Type::Char, Vec::new()),
						"const" => Kind::Type(Type::Const, Vec::new()),
						"fraction" => Kind::Type(Type::Fraction, Vec::new()),
						"heap" => Kind::Type(Type::Heap, Vec::new()),
						"int" => Kind::Type(Type::Int, Vec::new()),
						"list" => Kind::Type(Type::List, Vec::new()),
						"only" => Kind::Type(Type::Only, Vec::new()),
						"pointer" => Kind::Type(Type::Pointer, Vec::new()),
						"register" => Kind::Type(Type::Register, Vec::new()),
						"stack" => Kind::Type(Type::Stack, Vec::new()),
						"unique" => Kind::Type(Type::Unique, Vec::new()),
						"unsigned" => Kind::Type(Type::Unsigned, Vec::new()),
						"volatile" => Kind::Type(Type::Volatile, Vec::new()),
						"void" => Kind::Type(Type::Void, Vec::new()),
						"as" | "async" | "break" | "continue" | "export" | "foreach" | "from" | "goto" | "import" | "in" | "match" | "receive" | "select" | "send" | "to" | "type" | "until" | "when" | "while" => Kind::Reserved(item.to_string(), RefCell::new(Vec::new())),
						"false" => Kind::Literal(false),
						"true" => Kind::Literal(true),
						
						"\n" => {
							line += 1;
							col = 1;
							continue;
						},
						
						"\r" | "\t" | " " => {
							col += 1;
							continue;
						},
						
						_ => if ops.contains(&item.chars().next().unwrap()) {
							Kind::Op(item.to_string(), RefCell::new(Vec::new()), RefCell::new(Vec::new()), RefCell::new(Vec::new()), RefCell::new(None))
						} else {
							Kind::Var(item.to_string(), vec![Vec::new()], RefCell::new(Vec::new()), RefCell::new(Vec::new()), RefCell::new(None))
						}
					};
					
					string.pos = if line > line_offset {
						FilePos {line: line - line_offset, col}
					} else {
						FilePos {line, col}
					};
					
					res.push(string.clone());
				}
			}
		}
		
		col += 1;
	}
	
	res
}

pub fn lex3(tokens: &mut Vec<Token>) {
	let mut pending = Vec::new();
	let mut m_default_val = 0;
	let mut mdv_changes = vec![0];
	let mut full_depth = 0;
	let mut rows = vec![0];
	let mut i = 0;
	while i < tokens.len() {
		match tokens[i].kind.clone() {
			Kind::GroupOp(ref op, _) if op == "{" => {
				full_depth += 1;
				
				if full_depth + 1 > rows.len() {
					rows.push(0);
				} else {
					rows[full_depth] += 1;
				}
				
				if full_depth + 1 > mdv_changes.len() {
					mdv_changes.push(0);
				} else {
					mdv_changes[full_depth] = 0;
				}
			},
			
			Kind::GroupOp(ref op, _) if op == "}" => if full_depth > 0 {
				m_default_val -= mdv_changes[full_depth];
				full_depth -= 1;
			} else {
				panic!("{}:{} Excess ending bracket", tokens[i].pos.line, tokens[i].pos.col);
			},
			
/*			Kind::Type(ref typ, _) if typ == &Type::Macro => {
				tokens.remove(i);
				
				match tokens[i + 1].kind.clone() {
					Kind::GroupOp(ref op, _) if op == ";" => {
						// Auto-initialised macros
						
						macros.push(Macro {
							func: Function {
								name: match tokens[i].kind.clone() {
									Kind::Var(name, _) => name,
									_ => panic!("{}:{} Invalid macro name", tokens[i].pos.line, tokens[i].pos.col) // Allow operators in the future?
								},
								pos: 0,
								args: vec![],
								precedence: 2,
								output: vec![vec![]]
							},
							
							code: vec![
								Token {
									kind: Kind::Reserved(String::from("func"), Vec::new()),
									pos: FilePos {line: 0, col: 0}, // Obviously wrong but the pos is irrelevant anyway
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::Var(String::from("init"), vec![vec![]]),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::GroupOp(String::from("{"), Vec::new()),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::Reserved(String::from("return")),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::Number(0, 0),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::GroupOp(String::from(";"), Vec::new()),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::GroupOp(String::from("}"), Vec::new()),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								}
							],
							
							returns: vec![vec![
								Token {
									kind: Kind::GroupOp(String::from(";"), Vec::new()),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::Number(m_default_val, 0),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::GroupOp(String::from(";"), Vec::new()),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								}
							]],
							
							depth: full_depth,
							row: rows[full_depth]
						});
						
						tokens.remove(i);
						i -= 1;
						
						m_default_val += 1;
						mdv_changes[full_depth] += 1;
					},
					
					_ => {
						// Macro functions
						
						macros.push(Macro {
							func: ...,
							code: ...,
							returns: vec![],
							depth: full_depth,
							row: rows[full_depth]
						});
						
						...
					}
				}
			}, */
			
			Kind::Type(ref typ, _) => {
				let mut types = vec![vec![typ.clone()]];
				
				let mut start = i;
				i += 1;
				
				let mut t = 0;
				while i < tokens.len() {
					match tokens[i].kind {
						Kind::Type(ref typ, _) => types[t].push(typ.clone()),
						Kind::Op(ref op, _, _, _, _) if op == "|" => {
							types.push(Vec::new());
							t += 1;
						},
						_ => break
					};
					
					i += 1;
				}
				
				if i >= tokens.len() {
					panic!("Unexpected EOF");
				}
				
				match tokens[i].kind {
					Kind::Var(_, ref mut typ, _, _, _) => *typ = types, // This should probably be changed because it's not really good for performance to copy a vector like this... [EDIT: does this actually copy the whole vector?]
					
					_ => {
						i -= 1;
						pending.push((start, types));
					}
				}
			},
			
			_ => ()
		}
		
		i += 1;
	}
	
	if full_depth > 0 {
		panic!("Unclosed curly bracket");
	}
	
	for (i, types) in pending {
		if let Kind::Type(_, ref mut typs) = tokens[i].kind {
			*typs = types;
		}
	}
}