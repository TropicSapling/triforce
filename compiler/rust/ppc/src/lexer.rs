use std::usize;
use std::cell::RefCell;
use lib::{Token, Kind, Type, FilePos, Macro, Function, FunctionArg};

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

pub fn lex2(tokens: Vec<&str>, line_offset: usize) -> (Vec<Token>, Vec<&str>) {
	let mut res: Vec<Token> = Vec::new();
	let mut ops = Vec::new();
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
	let mut op_up_nxt = false;
	
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
				} else if item == "operator" {
					op_up_nxt = true;
				} else if op_up_nxt {
					ops.push(item);
					op_up_nxt = false;
				} else {
					string.kind = match item {
						"{" | "}" | "[" | "]" | "(" | ")" | ";" => Kind::GroupOp(item.to_string()),
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
						
						_ => if ops.contains(&item) {
							Kind::Op(item.to_string())
						} else {
							Kind::Var(item.to_string(), vec![Vec::new()])
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
	
	(res, ops)
}

pub fn lex3(tokens: &mut Vec<Token>, mut functions: Vec<Function>) -> (Vec<Function>, Vec<Macro>) {
	let mut macros = Vec::new();
	let mut m_default_val = 0;
	let mut mdv_changes = vec![0];
	let mut full_depth = 0;
	let mut rows = vec![0];
	let mut i = 0;
	while i < tokens.len() {
		match tokens[i].kind.clone() {
			Kind::GroupOp(ref op) if op == "{" => {
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
			
			Kind::GroupOp(ref op) if op == "}" => if full_depth > 0 {
				m_default_val -= mdv_changes[full_depth];
				full_depth -= 1;
			} else {
				panic!("{}:{} Excess ending bracket", tokens[i].pos.line, tokens[i].pos.col);
			},
			
			Kind::Type(ref typ) if typ == &Type::Macro => {
				tokens.remove(i);
				
				match tokens[i + 1].kind.clone() {
					Kind::GroupOp(ref op) if op == ";" => {
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
									kind: Kind::Type(Type::Func),
									pos: FilePos {line: 0, col: 0}, // Obviously wrong but the pos is irrelevant anyway
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::Var(String::from("init"), vec![vec![Type::Func]]),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::GroupOp(String::from("{")),
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
									kind: Kind::GroupOp(String::from(";")),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::GroupOp(String::from("}")),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								}
							],
							
							returns: vec![vec![
								Token {
									kind: Kind::GroupOp(String::from(";")),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::Number(m_default_val, 0),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::GroupOp(String::from(";")),
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
							func: Function {
								name: String::from(""),
								pos: 0,
								args: vec![],
								precedence: 2,
								output: vec![]
							},
							
							code: vec![
								Token {
									kind: Kind::Type(Type::Func),
									pos: FilePos {line: 0, col: 0}, // Obviously wrong but the pos is irrelevant anyway
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::Var(String::from("init"), Vec::new()),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								},
								
								Token {
									kind: Kind::GroupOp(String::from("{")),
									pos: FilePos {line: 0, col: 0},
									children: RefCell::new(Vec::new())
								}
							],
							
							returns: vec![],
							depth: full_depth,
							row: rows[full_depth]
						});
						
						let mut last_item = macros.len();
						if last_item != 0 {
							last_item -= 1;
						}
						
						let mut par_type = vec![vec![]];
						
						let mut last_token_kind = tokens[i - 1].kind.clone();
						while i < tokens.len() {
							match tokens[i].kind.clone() {
								Kind::Type(_) => match tokens[i + 1].kind {
									Kind::GroupOp(ref op) if op == "{" => {
										let end = i;
										let mut t = 0;
										while i > 0 {
											match tokens[i].kind {
												Kind::Type(ref typ) => par_type[t].push(typ.clone()),
												Kind::Op(ref op) if op == "|" => {
													par_type.push(Vec::new());
													t += 1;
												},
												_ => break
											}
											
											i -= 1;
										}
										
										par_type.reverse();
										for section in par_type.iter_mut() {
											section.reverse();
										}
										
										i = end;
									},
									
									_ => ()
								},
								
								Kind::Var(ref name, _) => if let Kind::Type(_) = last_token_kind { // Function args
									macros[last_item].func.args.push(FunctionArg {name: name.clone(), typ: Vec::new()}); // Arg types for macro functions are WIP; TODO: replace 'Vec::new()' with actual type
								} else { // Function name
									macros[last_item].func.name += name;
									macros[last_item].func.pos = macros[last_item].func.args.len();
								},
								
								Kind::Op(ref op) => if op == "-" {
									match tokens[i + 1].kind {
										Kind::Op(ref op) if op == ">" => i += 1,
										_ => { // Operator (function) name
											macros[last_item].func.name += op;
											macros[last_item].func.pos = macros[last_item].func.args.len();
										}
									}
								} else if op != "|" { // Operator (function) name
									macros[last_item].func.name += op;
									macros[last_item].func.pos = macros[last_item].func.args.len();
								},
								
								Kind::GroupOp(ref op) => if op == "{" { // Function body
									macros[last_item].func.output = par_type.clone();
									if macros[last_item].func.name == "**" {
										macros[last_item].func.precedence = 247;
									} else if par_type[0].len() > 0 {
										if macros[last_item].func.args.len() == 1 {
											macros[last_item].func.precedence = 255;
										}
									}
									
									tokens.remove(i);
									break;
								} else if op == ";" { // End of function declaration
									panic!("{}:{} Macro functions must have a body", tokens[i].pos.line, tokens[i].pos.col);
								},
								
								_ => ()
							}
							
							last_token_kind = tokens[i].kind.clone();
							tokens.remove(i);
						}
						
						let mut point = 0;
						let mut depth = 0;
						while i < tokens.len() {
							match tokens[i].kind.clone() {
								Kind::GroupOp(ref op) if op == "{" => depth += 1,
								Kind::GroupOp(ref op) if op == "}" => if depth > 0 {
									depth -= 1;
								} else {
									break;
								},
								
								Kind::Reserved(ref keyword) if keyword == "return" => {
									macros[last_item].returns.push(vec![Token {
										kind: Kind::GroupOp(String::from(";")),
										pos: FilePos {line: 0, col: 0},
										children: RefCell::new(Vec::new())
									}]);
									
									macros[last_item].code.push(tokens[i].clone());
									macros[last_item].code.push(Token {
										kind: Kind::Number(point, 0),
										pos: FilePos {line: 0, col: 0},
										children: RefCell::new(Vec::new())
									});
									
									tokens.remove(i);
									
									let mut depth = 0;
									while i < tokens.len() {
										match tokens[i].kind.clone() {
											Kind::GroupOp(ref op) if op == "{" => depth += 1,
											Kind::GroupOp(ref op) if op == "}" => if depth > 0 {
												depth -= 1;
											} else {
												panic!("{}:{} Excess ending bracket", tokens[i].pos.line, tokens[i].pos.col);
											},
											
											Kind::GroupOp(ref op) if op == ";" && depth == 0 => break,
											_ => ()
										}
										
										macros[last_item].returns[point].push(tokens[i].clone());
										tokens.remove(i);
									}
									
									macros[last_item].returns[point].push(Token {
										kind: Kind::GroupOp(String::from(";")),
										pos: FilePos {line: 0, col: 0},
										children: RefCell::new(Vec::new())
									});
									
									point += 1;
								},
								
								_ => ()
							}
							
							macros[last_item].code.push(tokens[i].clone());
							tokens.remove(i);
						}
						
						macros[last_item].code.push(tokens[i].clone());
						tokens.remove(i);
						i -= 1;
						
						functions.push(macros[last_item].func.clone());
						
						match lex3(&mut macros[last_item].code, functions) {
							(f, _) => functions = f
						}
					}
				}
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
			
			_ => ()
		}
		
		i += 1;
	}
	
	(functions, macros)
}