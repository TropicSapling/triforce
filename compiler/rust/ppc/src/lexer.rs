use std::cell::RefCell;
use lib::{Token, Kind, Type, FilePos, Macro};

fn is_var(c: char) -> bool {
	c == '_' || c == '$' || c.is_alphanumeric()
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

pub fn lex2(tokens: Vec<&str>, line_offset: usize) -> Vec<Token> {
	let mut res: Vec<Token> = Vec::new();
	let mut string = Token {
		kind: Kind::Str1(String::from("")),
		pos: FilePos {line: 1, col: 1},
		children: RefCell::new(vec![])
	};
	
	let mut in_str = false;
	let mut in_str2 = false;
	let mut escaping = false;
	let mut ignoring = false;
	let mut ignoring2 = false;
	let mut possible_comment = false;
	
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
		} else if ignoring2 {
			if possible_comment {
				if item == "/" {
					ignoring2 = false;
				}
				
				possible_comment = false;
			}
			
			if item == "*" {
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
					ignoring2 = true;
					possible_comment = false;
					
					continue;
				} else {
					possible_comment = false;
					
					string.kind = Kind::Op(String::from("/"));
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
							Kind::Number(n, _) => string.kind = Kind::Number(n, item.parse::<u64>().unwrap()),
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
				
				let int_res = item.parse::<u64>();
				
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
						"+" | "-" | "*" | "/" | "%" | "=" | "&" | "|" | "^" | "<" | ">" | "!" | "~" | "?" | ":" | "." | "," | "@" | ";" => Kind::Op(item.to_string()),
						"{" | "}" | "[" | "]" | "(" | ")" => Kind::GroupOp(item.to_string()),
						"array" => Kind::Type(Type::Array),
						"bool" => Kind::Type(Type::Bool),
						"chan" => Kind::Type(Type::Chan),
						"char" => Kind::Type(Type::Char),
						"const" => Kind::Type(Type::Const),
						"fraction" => Kind::Type(Type::Fraction),
						"func" => Kind::Type(Type::Func),
						"heap" => Kind::Type(Type::Heap),
						"int" => Kind::Type(Type::Int),
						"list" => Kind::Type(Type::List),
						"macro" => Kind::Type(Type::Macro),
						"only" => Kind::Type(Type::Only),
						"pointer" => Kind::Type(Type::Pointer),
						"register" => Kind::Type(Type::Register),
						"stack" => Kind::Type(Type::Stack),
						"unique" => Kind::Type(Type::Unique),
						"unsigned" => Kind::Type(Type::Unsigned),
						"volatile" => Kind::Type(Type::Volatile),
						"void" => Kind::Type(Type::Void),
						"as" | "async" | "break" | "continue" | "else" | "export" | "foreach" | "from" | "goto" | "if" | "import" | "in" | "let" | "match" | "receive" | "repeat" | "return" | "select" | "send" | "to" | "type" | "until" | "when" | "while" => Kind::Reserved(item.to_string()),
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
						_ => Kind::Var(item.to_string(), vec![Vec::new()])
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

fn del_outofscope_macros(macros: &mut Vec<Macro>, depth: usize) {
	let mut i = 0;
	while i < macros.len() {
		if depth < macros[i].depth {
			macros.remove(i);
		} else {
			i += 1;
		}
	}
}

pub fn lex3(tokens: &mut Vec<Token>) {
	let mut macros = Vec::new();
	let mut full_depth = 0;
	let mut i = 0;
	while i < tokens.len() {
		match tokens[i].kind.clone() {
			Kind::GroupOp(ref op) if op == "{" => full_depth += 1,
			Kind::GroupOp(ref op) if op == "}" => if full_depth > 0 {
				full_depth -= 1;
				del_outofscope_macros(&mut macros, full_depth);
			} else {
				panic!("{}:{} Excess ending bracket", tokens[i].pos.line, tokens[i].pos.col);
			},
			
			Kind::Type(ref typ) if typ == &Type::Macro => {
				tokens.remove(i);
				
				let name;
				match tokens[i].kind.clone() {
					Kind::Type(ref typ) if typ == &Type::Func => {
						tokens.remove(i);
						name = tokens[i].clone();
					},
					
					_ => name = tokens[i].clone()
				}
				
				let mut contents = Vec::new();
				let mut depth = 0;
				
				tokens.drain(i..i + 2);
				while i < tokens.len() {
					match tokens[i].kind.clone() {
						Kind::GroupOp(ref op) if op == "{" => depth += 1,
						Kind::GroupOp(ref op) if op == "}" => if depth > 0 {
							depth -= 1;
						} else {
							panic!("{}:{} Excess ending bracket", tokens[i].pos.line, tokens[i].pos.col);
						}
						
						Kind::Op(ref op) if op == ";" && depth == 0 => {
							tokens.remove(i);
							i -= 1;
							break;
						},
						
						_ => ()
					}
					
					contents.push(tokens[i].clone());
					tokens.remove(i);
				}
				
				macros.push(Macro {name, contents, depth: full_depth});
			},
			
			Kind::Type(ref typ) => {
				let mut types = vec![vec![typ.clone()]];
				
				i += 1;
				
				let mut t = 0;
				while i < tokens.len() {
					match tokens[i].kind {
						Kind::Type(ref typ) => types[t].push(typ.clone()),
						Kind::Op(ref op) if op == "|" => {
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
					Kind::Var(_, ref mut typ) => *typ = types, // This should probably be changed because it's not really good for performance to copy a vector like this...
					_ => i -= 1
				}
			},
			
			Kind::Var(ref name, _) => {
				let mut j = 0;
				while j < macros.len() {
					if let Kind::Var(ref name2, _) = macros[j].name.kind {
						if name == name2 {
							// Expand macro
							
							tokens[i] = macros[j].contents[0].clone();
							let mut pos = i + 1;
							let mut k = 1;
							while k < macros[j].contents.len() {
								tokens.insert(pos, macros[j].contents[k].clone());
								pos += 1;
								k += 1;
							}
							
							i -= 1;
							break;
						}
					}
					
					j += 1;
				}
			},
			
			_ => ()
		}
		
		i += 1;
	}
}