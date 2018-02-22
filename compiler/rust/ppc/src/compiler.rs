use lib::{Token, Kind, Type, Function, FunctionArg};

macro_rules! last {
	($e:expr) => ($e[$e.len() - 1]);
}

macro_rules! is_kind {
	($lhs_kind:expr, $rhs_kind:pat) => (match $lhs_kind {
		$rhs_kind => true,
		_ => false
	});
}

macro_rules! get_val {
	($e:expr) => ({
		use lib::Kind::*;
		match $e {
			GroupOp(val) => val,
			Op(val) => val,
			Reserved(val) => val,
			Str1(val) => val,
			Str2(val) => val,
			_ => String::new()
		}
	});
}

macro_rules! group_expr {
	($end:expr, $tokens:expr, $token:expr, $i:expr) => ({
		let mut j = nxt(&$tokens, $i);
		while $i + j < $tokens.len() && match $tokens[$i + j].kind {
			Kind::GroupOp(val) => val != $end,
			_ => true
		} {
			(*$token.children.borrow_mut()).push($i + j);
			
			j += nxt(&$tokens, $i + j);
		}
	})
}

fn nxt(tokens: &Vec<Token>, i: usize) -> usize {
	let mut j: usize = 0;
	while {
		j += 1;
		
		i + j < tokens.len() && is_kind!(tokens[i + j].kind, Kind::Whitespace(_))
	} {}
	
	if i + j < tokens.len() {
		j
	} else {
		0
	}
}

fn prev(tokens: &Vec<Token>, i: usize) -> usize {
	let mut j: usize = 0;
	while {
		j += 1;
		
		i - j > 0 && is_kind!(tokens[i - j].kind, Kind::Whitespace(_)) // MAY NEED CHANGING
	} {}
	
	if i - j > 0 {
		j
	} else {
		0
	}
}

fn group(tokens: &mut Vec<Token>, i: &mut usize, op: &'static str, op_close: &'static str) {
	let mut tok_str = String::from(op);
	
	while match tokens[*i].kind {
		Kind::GroupOp(ref mut val) => *val != op_close,
		_ => true
	} {
		*i += 1;
		tok_str = compile(tokens, i, tok_str);
	}
	
	tokens[*i].kind = Kind::Var(tok_str, Type::Void);
	
	*i -= 1;
}

fn is_defined<'a>(defs: &'a Vec<Function>, call: &str) -> Option<&'a Function<'a>> {
	for def in defs {
		if def.name == call {
			return Some(&def);
		}
	}
	
	None
}

pub fn parse(tokens: &mut Vec<Token>) {
	let mut functions: Vec<Function> = Vec::new();
	let mut func = false;
	let mut par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
	let mut type_i = 0;
	
	for token in (&tokens).iter() {
		if is_kind!(token.kind, Kind::Whitespace(_)) {
			continue; // Ignore whitespace
		}
		
		let mut last_item = functions.len();
		if last_item != 0 {
			last_item -= 1;
		}
		
		if match token.kind {
			Kind::Reserved(val) => val == "func",
			_ => false
		} {
			functions.push(Function {name: "", pos: 0, args: vec![], output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]});
			func = true;
		} else if func {
			if match token.kind {
				Kind::GroupOp(val) => val == "{", // Function body
				Kind::Op(val) => val == ";", // End of function declaration
				_ => false
			} {
				functions[last_item].output = par_type.clone();
				
				par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				type_i = 0;
				func = false;
			} else if is_kind!(token.kind, Kind::Type(_)) { // Parameter / return types
				let val = match token.kind {
					Kind::Type(val) => val,
					_ => Type::Void
				};
				par_type[type_i] = val.clone();
				type_i += 1;
			} else if par_type[0] != Type::Void {
				let name = match token.kind {
					Kind::Var(name, _) => name,
					_ => String::new()
				};
				functions[last_item].args.push(FunctionArg {name: &name, typ: par_type});
				
				par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				type_i = 0;
			} else if functions[last_item].name == "" && (is_kind!(token.kind, Kind::Var(_,_)) || is_kind!(token.kind, Kind::Op(_))) { // Function name
				let name = match token.kind {
					Kind::Var(name, _) => name,
					_ => String::new()
				};
				functions[last_item].name = &name;
				functions[last_item].pos = functions[last_item].args.len();
			}
		}
	}
	
	let mut i = 0;
	while i < tokens.len() {
		let token = &tokens[i];
		
		if is_kind!(token.kind, Kind::Var(_,_)) || is_kind!(token.kind, Kind::Op(_)) {
			let val = match token.kind {
				Kind::Var(val, _) => val,
				_ => String::new()
			}; // Probably needs fixing
			let def = is_defined(&functions, &val);
			
			if let Some(def) = def {
				if def.pos > 0 {
					let mut j = 0;
					while i - j > 0 && j < def.pos && match tokens[i - j].kind {
						Kind::Op(val) => val != ";",
						_ => true
					} { // NOTE: comparison may need to be changed
						j += prev(&tokens, i - j);
						
						(*token.children.borrow_mut()).push(i - j);
					}
				}
				
				let mut j = 0;
				while i + j < tokens.len() && j < def.args.len() - def.pos && match tokens[i + j].kind {
						Kind::Op(val) => val != ";",
						_ => true
					} {
					j += nxt(&tokens, i + j);
					
					(*token.children.borrow_mut()).push(i + j);
				}
				
				if (*token.children.borrow()).len() > 1 { // DEBUG
					println!("{:#?}", tokens[(*token.children.borrow())[0]]);
					println!("{:#?}", tokens[(*token.children.borrow())[1]]);
				}
			}
		} else if is_kind!(token.kind, Kind::GroupOp(_)) {
			let val = match token.kind {
				Kind::GroupOp(val) => val,
				_ => String::new()
			};
			match val.as_ref() {
				"(" => group_expr!(")", tokens, token, i),
				"{" => group_expr!("}", tokens, token, i),
				"[" => group_expr!("]", tokens, token, i),
				&_ => (),
			}
		}
		
		i += 1;
	}
}

pub fn compile(mut tokens: &mut Vec<Token>, i: &mut usize, mut output: String) -> String {
	let val = {
		use lib::Kind::*;
		match tokens[*i].kind {
			GroupOp(val) => val,
			Op(val) => val,
			Reserved(val) => val,
			Str1(val) => val,
			Str2(val) => val,
			_ => String::new()
		}
	};
	
	match val.as_ref() {
		"array" | "chan" | "fraction" | "heap" | "list" | "number" | "register" | "stack" | "async" | "from" | "receive" | "select" | "send" | "to" => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
		"@" => output += "*",
		"-" if get_val!(tokens[*i + 1].kind) == ">" && !is_kind!(tokens[*i + 1 + nxt(tokens, *i + 1)].kind, Kind::Type(_)) => {
			output += "&";
			*i += 1;
		},
		"(" => group(&mut tokens, i, "(", ")"),
		"[" => group(&mut tokens, i, "[", "]"),
		"{" => group(&mut tokens, i, "{", "}"),
		"init" => output += "main",
		"func" => output += "fn",
		"import" => output += "use",
		"foreach" => output += "for",
		"as" => output += "@",
		"astype" => output += "as", // TMP; will be replaced with (<type>) <variable>
		_ => {
			let pos_change = match tokens[*i].kind {
				Kind::Str1(_) | Kind::Str2(_) | Kind::Number(_,_) | Kind::Literal(_) | Kind::Var(_,_) => {
					let nxt_tok = nxt(tokens, *i);
					if nxt_tok > 0 && is_kind!(tokens[*i + nxt_tok].kind, Kind::Var(_,_)) {
						output += &get_val!(tokens[*i + nxt_tok].kind);
						output += "(";
						nxt_tok
					} else {
						0
					}
				},
				_ => 0
			};
			
			match tokens[*i].kind {
				Kind::Str1(_) => {
					output += "\"";
					output += &get_val!(tokens[*i].kind);
					output += "\"";
				},
				Kind::Str2(_) => {
					output += "'";
					output += &get_val!(tokens[*i].kind);
					output += "'";
				},
				Kind::Type(_) => {
					let mut nxt_tokens: Vec<usize> = vec!(nxt(tokens, *i));
					while last!(nxt_tokens) > 0 && is_kind!(tokens[*i + last!(nxt_tokens)].kind, Kind::Type(_)) {
						let last_tok = last!(nxt_tokens);
						nxt_tokens.push({
							let nxt_tok = nxt(tokens, *i + last_tok) + last_tok;
							if nxt_tok == last_tok {
								0
							} else {
								nxt_tok
							}
						});
					}
					
					if last!(nxt_tokens) > 0 && is_kind!(tokens[*i + last!(nxt_tokens)].kind, Kind::Var(_,_)) {
						output += &get_val!(tokens[*i + last!(nxt_tokens)].kind);
						output += ":";
						
						output += match get_val!(tokens[*i].kind).as_ref() {
							"unsigned" => {
								if nxt_tokens[0] > 0 && is_kind!(tokens[*i + nxt_tokens[0]].kind, Kind::Type(_)) {
									match get_val!(tokens[*i + nxt_tokens[0]].kind).as_ref() {
										"int" => "u64",
										_ => panic!("{}:{} Invalid type '{}' following 'unsigned'", tokens[*i + nxt_tokens[0]].pos.line, tokens[*i + nxt_tokens[0]].pos.col, get_val!(tokens[*i + nxt_tokens[0]].kind))
									}
								} else {
									panic!("{}:{} Missing data type following 'unsigned'", tokens[*i].pos.line, tokens[*i].pos.col);
								}
							},
							"int" => "i64",
							_ => &get_val!(tokens[*i].kind)
						};
						
						*i += last!(nxt_tokens);
					} else {
						output += match get_val!(tokens[*i].kind).as_ref() {
							"unsigned" => {
								let nxt_tok = nxt(tokens, *i);
								
								*i += nxt_tok;
								
								if nxt_tok > 0 && is_kind!(tokens[*i].kind, Kind::Type(_)) {
									match get_val!(tokens[*i].kind).as_ref() {
										"int" => "u64",
										_ => panic!("{}:{} Invalid type '{}' following 'unsigned'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind))
									}
								} else {
									panic!("{}:{} Missing data type following 'unsigned'", tokens[*i].pos.line, tokens[*i].pos.col);
								}
							},
							"int" => "i64",
							_ => &get_val!(tokens[*i].kind)
						};
					}
				},
				_ => output += &get_val!(tokens[*i].kind)
			}
			
			if pos_change > 0 {
				*i += pos_change;
				*i += nxt(tokens, *i);
				
				output += ",";
				output = compile(tokens, i, output);
				*i += 1;
				output += &get_val!(tokens[*i].kind);
				output += ")";
			}
		}
	};
	
	output
}