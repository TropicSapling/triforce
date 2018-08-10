use std::usize;
use std::cell::RefCell;
use lib::{Token, Kind, Type, FilePos, Macro, MacroFunction, Function, FunctionArg};
use compiler::{parse, parse2, parse_statement};

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
	
/*	i = 0;
	while i < macro_funcs.len() {
		if depth < macro_funcs[i].depth {
			macro_funcs.remove(i);
		} else {
			i += 1;
		}
	} */
}

pub fn lex3(tokens: &mut Vec<Token>, mut functions: Vec<Function>) -> (Vec<Function>, Vec<MacroFunction>) {
	let mut macros = Vec::new();
	let mut macro_funcs = Vec::new();
	let mut full_depth = 0;
	let mut bpos = 0;
//	let mut start = 0;
	let mut i = 0;
	while i < tokens.len() {
		match tokens[i].kind.clone() {
			Kind::GroupOp(ref op) if op == "{" => {
				full_depth += 1;
				bpos += 1;
			},
			Kind::GroupOp(ref op) if op == "}" => if full_depth > 0 {
				full_depth -= 1;
				del_outofscope_macros(&mut macros, full_depth);
			} else {
				panic!("{}:{} Excess ending bracket", tokens[i].pos.line, tokens[i].pos.col);
			},
			
			Kind::Type(ref typ) if typ == &Type::Macro => {
				tokens.remove(i);
				
				match tokens[i].kind.clone() {
					Kind::Type(ref typ) if typ == &Type::Func => { // Macro function
						tokens.remove(i);
						
						macro_funcs.push(MacroFunction {
							func: Function {
								name: String::from(""),
								pos: 0,
								args: vec![],
								precedence: 1,
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
							bpos
						});
						
						let mut last_item = macro_funcs.len();
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
									macro_funcs[last_item].func.args.push(FunctionArg {name: name.clone(), typ: Vec::new()}); // Arg types for macro functions are WIP; TODO: replace 'Vec::new()' with actual type
								} else { // Function name
									macro_funcs[last_item].func.name += name;
									macro_funcs[last_item].func.pos = macro_funcs[last_item].func.args.len();
								},
								
								Kind::Op(ref op) => if op == "-" {
									match tokens[i + 1].kind {
										Kind::Op(ref op) if op == ">" => i += 1,
										_ => { // Operator (function) name
											macro_funcs[last_item].func.name += op;
											macro_funcs[last_item].func.pos = macro_funcs[last_item].func.args.len();
										}
									}
								} else if op == ";" { // End of function declaration
									panic!("{}:{} Macro functions must have a body", tokens[i].pos.line, tokens[i].pos.col);
								} else if op != "|" { // Operator (function) name
									macro_funcs[last_item].func.name += op;
									macro_funcs[last_item].func.pos = macro_funcs[last_item].func.args.len();
								},
								
								Kind::GroupOp(ref op) => if op == "{" { // Function body
									macro_funcs[last_item].func.output = par_type.clone();
									if macro_funcs[last_item].func.name == "**" {
										macro_funcs[last_item].func.precedence = 247;
									} else if par_type[0].len() > 0 {
										if macro_funcs[last_item].func.args.len() == 1 {
											macro_funcs[last_item].func.precedence = 255;
										} else {
											macro_funcs[last_item].func.precedence = 2;
										}
									}
									
									tokens.remove(i);
									break;
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
									macro_funcs[last_item].returns.push(vec![Token {
										kind: Kind::Op(String::from(";")),
										pos: FilePos {line: 0, col: 0},
										children: RefCell::new(Vec::new())
									}]);
									
									macro_funcs[last_item].code.push(tokens[i].clone());
									macro_funcs[last_item].code.push(Token {
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
											
											Kind::Op(ref op) if op == ";" && depth == 0 => break,
											_ => ()
										}
										
										macro_funcs[last_item].returns[point].push(tokens[i].clone());
										tokens.remove(i);
									}
									
									macro_funcs[last_item].returns[point].push(Token {
										kind: Kind::Op(String::from(";")),
										pos: FilePos {line: 0, col: 0},
										children: RefCell::new(Vec::new())
									});
									
									point += 1;
								},
								
								_ => ()
							}
							
							macro_funcs[last_item].code.push(tokens[i].clone());
							tokens.remove(i);
						}
						
						macro_funcs[last_item].code.push(Token {
							kind: Kind::GroupOp(String::from("}")),
							pos: FilePos {line: 0, col: 0},
							children: RefCell::new(Vec::new())
						});
						
						functions.push(macro_funcs[last_item].func.clone());
						
						match lex3(&mut macro_funcs[last_item].code, functions) {
							(f, _) => functions = f
						}
						
/*						functions = parse(&macro_funcs[last_item].code, functions);
						parse2(&mut macro_funcs[last_item].code, &functions, &mut 2);
						
						for point in macro_funcs[last_item].returns.iter() {
							parse_statement(point, &functions, &mut 0);
						} */
					},
					
					_ => {
						let name = tokens[i].clone();
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
	
	(functions, macro_funcs)
}