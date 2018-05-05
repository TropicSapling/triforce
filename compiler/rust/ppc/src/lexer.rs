use std::cell::RefCell;
use lib::{Token, Kind, Type, FilePos};

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

pub fn lex2(tokens: Vec<&str>) -> Vec<Token> {
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
					string.pos = FilePos {line, col};
					
					res.push(string.clone());
				}
			}
			
			if escaping {
				let mut val = match string.kind {
					Kind::Str1(ref mut value) => value,
					Kind::Str2(ref mut value) => value,
					_ => unreachable!()
				};
				if item == "0" || item == "n" { // Null and newlines
					*val += "\\";
				}
				
				*val += item;
				string.pos = FilePos {line, col};
				
				escaping = false;
			} else if in_str {
				if item == "\"" {
					res.push(string.clone());
					in_str = false;
				} else if item == "\\" {
					escaping = true;
				} else {
					let mut val = match string.kind {
						Kind::Str1(ref mut val) => val,
						_ => unreachable!()
					};
					*val += item;
				}
			} else if in_str2 {
				if item == "'" {
					res.push(string.clone());
					in_str2 = false;
				} else if item == "\\" {
					escaping = true;
				} else {
					let mut val = match string.kind {
						Kind::Str2(ref mut val) => val,
						_ => unreachable!()
					};
					*val += item;
				}
			} else if item == "\"" {
				string.kind = Kind::Str1(String::from(""));
				string.pos = FilePos {line, col};
				in_str = true;
			} else if item == "'" {
				string.kind = Kind::Str2(String::from(""));
				string.pos = FilePos {line, col};
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
					string.pos = FilePos {line, col};
					
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
						"only" => Kind::Type(Type::Only),
						"pointer" => Kind::Type(Type::Pointer),
						"register" => Kind::Type(Type::Register),
						"stack" => Kind::Type(Type::Stack),
						"unique" => Kind::Type(Type::Unique),
						"unsigned" => Kind::Type(Type::Unsigned),
						"volatile" => Kind::Type(Type::Volatile),
						"void" => Kind::Type(Type::Void),
						"as" | "async" | "break" | "continue" | "else" | "export" | "foreach" | "from" | "goto" | "if" | "import" | "in" | "match" | "receive" | "repeat" | "return" | "select" | "send" | "to" | "type" | "until" | "when" | "while" => Kind::Reserved(item.to_string()),
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
						_ => Kind::Var(item.to_string(), [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void])
					};
					string.pos = FilePos {line, col};
					
					res.push(string.clone());
				}
			}
		}
		
		col += 1;
	}
	
	res
}

pub fn lex3(tokens: &mut Vec<Token>) {
	let mut i = 0;
	while i < tokens.len() {
		match tokens[i].kind.clone() {
			Kind::Type(ref typ) => {
				let mut types = [typ.clone(), Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				let mut j = 1;
				
				i += 1;
				
				loop {
					match tokens[i].kind {
						Kind::Type(ref typ) => types[j] = typ.clone(),
						_ => break
					};
					
					i += 1;
					j += 1;
				}
				
				match tokens[i].kind {
					Kind::Var(_, ref mut typ) => *typ = types,
					_ => ()
				}
			},
			
			_ => ()
		}
		
		i += 1;
	}
}