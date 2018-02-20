use lib::{Token, Val, Kind, Type, Function, FunctionArg};

macro_rules! last {
	($e:expr) => ($e[$e.len() - 1]);
}

macro_rules! is_kind {
	($lhs_kind:expr, $rhs_kind:pat) => (match $lhs_kind {
		$rhs_kind => true,
		_ => false
	});
}

macro_rules! group_expr {
	($end:expr, $tokens:expr, $token:expr, $i:expr) => ({
		let mut j = nxt(&$tokens, $i);
		while $i + j < $tokens.len() && $tokens[$i + j].kind.0 != $end {
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
	
	while tokens[*i].kind.0 != op_close {
		*i += 1;
		tok_str = compile(tokens, i, tok_str);
	}
	
	tokens[*i].kind = Kind::Var(Val::Str(tok_str), Type::Void);
	
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
		
		if token.kind.0 == "func" {
			functions.push(Function {name: "", pos: 0, args: vec![], output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]});
			func = true;
		} else if func {
			if token.kind.0 == "{" || token.kind.0 == ";" { // Function body / end of function declaration
				functions[last_item].output = par_type.clone();
				
				par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				type_i = 0;
				func = false;
			} else if is_kind!(token.kind, Kind::Type(_)) { // Parameter / return types
				par_type[type_i] = token.kind.0.clone();
				type_i += 1;
			} else if par_type[0] != Type::Void {
				functions[last_item].args.push(FunctionArg {name: &token.kind.0, typ: par_type});
				
				par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				type_i = 0;
			} else if functions[last_item].name == "" && (is_kind!(token.kind, Kind::Var(_,_)) || is_kind!(token.kind, Kind::Op(_))) { // Function name
				functions[last_item].name = &token.kind.0;
				functions[last_item].pos = functions[last_item].args.len();
			}
		}
	}
	
	let mut i = 0;
	while i < tokens.len() {
		let token = &tokens[i];
		
		if is_kind!(token.kind, Kind::Var(_,_)) || is_kind!(token.kind, Kind::Op(_)) {
			let def = is_defined(&functions, &token.kind.0);
			
			if let Some(def) = def {
				if def.pos > 0 {
					let mut j = 0;
					while i - j > 0 && j < def.pos && tokens[i - j].kind.0 != ";" { // NOTE: comparison may need to be changed
						j += prev(&tokens, i - j);
						
						(*token.children.borrow_mut()).push(i - j);
					}
				}
				
				let mut j = 0;
				while i + j < tokens.len() && j < def.args.len() - def.pos && tokens[i + j].kind.0 != ";" {
					j += nxt(&tokens, i + j);
					
					(*token.children.borrow_mut()).push(i + j);
				}
				
				if (*token.children.borrow()).len() > 1 { // DEBUG
					println!("{:#?}", tokens[(*token.children.borrow())[0]]);
					println!("{:#?}", tokens[(*token.children.borrow())[1]]);
				}
			}
		} else if is_kind!(token.kind, Kind::GroupOp(_)) {
			match token.kind.0.as_ref() {
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
	match tokens[*i].kind.0.as_ref() {
		"array" | "chan" | "fraction" | "heap" | "list" | "number" | "register" | "stack" | "async" | "from" | "receive" | "select" | "send" | "to" => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, tokens[*i].kind.0),
		"@" => output += "*",
		"-" if tokens[*i + 1].kind.0 == ">" && !is_kind!(tokens[*i + 1 + nxt(tokens, *i + 1)].kind, Kind::Type(_)) => {
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
						output += &tokens[*i + nxt_tok].kind.0;
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
					output += &tokens[*i].kind.0;
					output += "\"";
				},
				Kind::Str2(_) => {
					output += "'";
					output += &tokens[*i].kind.0;
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
						output += &tokens[*i + last!(nxt_tokens)].kind.0;
						output += ":";
						
						output += match tokens[*i].kind.0.as_ref() {
							"unsigned" => {
								if nxt_tokens[0] > 0 && is_kind!(tokens[*i + nxt_tokens[0]].kind, Kind::Type(_)) {
									match tokens[*i + nxt_tokens[0]].kind.0.as_ref() {
										"int" => "u64",
										_ => panic!("{}:{} Invalid type '{}' following 'unsigned'", tokens[*i + nxt_tokens[0]].pos.line, tokens[*i + nxt_tokens[0]].pos.col, tokens[*i + nxt_tokens[0]].kind.0)
									}
								} else {
									panic!("{}:{} Missing data type following 'unsigned'", tokens[*i].pos.line, tokens[*i].pos.col);
								}
							},
							"int" => "i64",
							_ => &tokens[*i].kind.0
						};
						
						*i += last!(nxt_tokens);
					} else {
						output += match tokens[*i].kind.0.as_ref() {
							"unsigned" => {
								let nxt_tok = nxt(tokens, *i);
								
								*i += nxt_tok;
								
								if nxt_tok > 0 && is_kind!(tokens[*i].kind, Kind::Type(_)) {
									match tokens[*i].kind.0.as_ref() {
										"int" => "u64",
										_ => panic!("{}:{} Invalid type '{}' following 'unsigned'", tokens[*i].pos.line, tokens[*i].pos.col, tokens[*i].kind.0)
									}
								} else {
									panic!("{}:{} Missing data type following 'unsigned'", tokens[*i].pos.line, tokens[*i].pos.col);
								}
							},
							"int" => "i64",
							_ => &tokens[*i].kind.0
						};
					}
				},
				_ => output += &tokens[*i].kind.0
			}
			
			if pos_change > 0 {
				*i += pos_change;
				*i += nxt(tokens, *i);
				
				output += ",";
				output = compile(tokens, i, output);
				*i += 1;
				output += &tokens[*i].kind.0;
				output += ")";
			}
		}
	};
	
	output
}