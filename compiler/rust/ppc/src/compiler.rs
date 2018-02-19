use lib::{Token, Type, Type2, Function, FunctionArg};

macro_rules! last {
	($e:expr) => ($e[$e.len() - 1]);
}

macro_rules! group_expr {
	($end:expr, $tokens:expr, $token:expr, $i:expr) => ({
		let mut j = nxt(&$tokens, $i);
		while $i + j < $tokens.len() && $tokens[$i + j].val != $end {
			(*$token.children.borrow_mut()).push($i + j);
			
			j += nxt(&$tokens, $i + j);
		}
	})
}

fn nxt(tokens: &Vec<Token>, i: usize) -> usize {
	let mut j: usize = 0;
	while {
		j += 1;
		
		i + j < tokens.len() && tokens[i + j].t == Type::Whitespace
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
		
		i - j > 0 && tokens[i - j].t == Type::Whitespace // MAY NEED CHANGING
	} {}
	
	if i - j > 0 {
		j
	} else {
		0
	}
}

fn group(tokens: &mut Vec<Token>, i: &mut usize, op: &'static str, op_close: &'static str) {
	let mut tok_str = String::from(op);
	
	while tokens[*i].val != op_close {
		*i += 1;
		tok_str = compile(tokens, i, tok_str);
	}
	
	tokens[*i].val = tok_str;
	tokens[*i].t = Type::Var;
	
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
	let mut par_type = [Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void];
	let mut type_i = 0;
	
	for token in (&tokens).iter() {
		if token.t == Type::Whitespace {
			continue; // Ignore whitespace
		}
		
		let mut last_item = functions.len();
		if last_item != 0 {
			last_item -= 1;
		}
		
		if token.val == "func" {
			functions.push(Function {name: "", pos: 0, args: vec![], output: [Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void]});
			func = true;
		} else if func {
			if token.val == "{" || token.val == ";" { // Function body / end of function declaration
				functions[last_item].output = par_type.clone();
				
				par_type = [Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void];
				type_i = 0;
				func = false;
			} else if token.t == Type::Type { // Parameter / return types
				par_type[type_i] = token.t2.clone();
				type_i += 1;
			} else if par_type[0] != Type2::Void {
				functions[last_item].args.push(FunctionArg {name: &token.val, t: par_type});
				
				par_type = [Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void];
				type_i = 0;
			} else if functions[last_item].name == "" && (token.t == Type::Var || token.t == Type::Op) { // Function name
				functions[last_item].name = &token.val;
				functions[last_item].pos = functions[last_item].args.len();
			}
		}
	}
	
	let mut i = 0;
	while i < tokens.len() {
		let token = &tokens[i];
		
		if token.t == Type::Var || token.t == Type::Op {
			let def = is_defined(&functions, &token.val);
			
			if let Some(def) = def {
				if def.pos > 0 {
					let mut j = 0;
					while i - j > 0 && j < def.pos && tokens[i - j].val != ";" { // NOTE: comparison may need to be changed
						j += prev(&tokens, i - j);
						
						(*token.children.borrow_mut()).push(i - j);
					}
				}
				
				let mut j = 0;
				while i + j < tokens.len() && j < def.args.len() - def.pos && tokens[i + j].val != ";" {
					j += nxt(&tokens, i + j);
					
					(*token.children.borrow_mut()).push(i + j);
				}
				
				if (*token.children.borrow()).len() > 1 { // DEBUG
					println!("{:#?}", tokens[(*token.children.borrow())[0]]);
					println!("{:#?}", tokens[(*token.children.borrow())[1]]);
				}
			}
		} else if token.t == Type::GroupOp {
			match token.val.as_ref() {
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
	match tokens[*i].val.as_ref() {
		"array" | "chan" | "fraction" | "heap" | "list" | "number" | "register" | "stack" | "async" | "from" | "receive" | "select" | "send" | "to" => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, tokens[*i].val),
		"@" => output += "*",
		"-" if tokens[*i + 1].val == ">" && tokens[*i + 1 + nxt(tokens, *i + 1)].t != Type::Type => {
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
			let pos_change = match tokens[*i].t {
				Type::Str1 | Type::Str2 | Type::Number | Type::Literal | Type::Var => {
					let nxt_tok = nxt(tokens, *i);
					if nxt_tok > 0 && tokens[*i + nxt_tok].t == Type::Var {
						output += &tokens[*i + nxt_tok].val;
						output += "(";
						nxt_tok
					} else {
						0
					}
				},
				_ => 0
			};
			
			match tokens[*i].t {
				Type::Str1 => {
					output += "\"";
					output += &tokens[*i].val;
					output += "\"";
				},
				Type::Str2 => {
					output += "'";
					output += &tokens[*i].val;
					output += "'";
				},
				Type::Type => {
					let mut nxt_tokens: Vec<usize> = vec!(nxt(tokens, *i));
					while last!(nxt_tokens) > 0 && tokens[*i + last!(nxt_tokens)].t == Type::Type {
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
					
					if last!(nxt_tokens) > 0 && tokens[*i + last!(nxt_tokens)].t == Type::Var {
						output += &tokens[*i + last!(nxt_tokens)].val;
						output += ":";
						
						output += match tokens[*i].val.as_ref() {
							"unsigned" => {
								if nxt_tokens[0] > 0 && tokens[*i + nxt_tokens[0]].t == Type::Type {
									match tokens[*i + nxt_tokens[0]].val.as_ref() {
										"int" => "u64",
										_ => panic!("{}:{} Invalid type '{}' following 'unsigned'", tokens[*i + nxt_tokens[0]].pos.line, tokens[*i + nxt_tokens[0]].pos.col, tokens[*i + nxt_tokens[0]].val)
									}
								} else {
									panic!("{}:{} Missing data type following 'unsigned'", tokens[*i].pos.line, tokens[*i].pos.col);
								}
							},
							"int" => "i64",
							_ => &tokens[*i].val
						};
						
						*i += last!(nxt_tokens);
					} else {
						output += match tokens[*i].val.as_ref() {
							"unsigned" => {
								let nxt_tok = nxt(tokens, *i);
								
								*i += nxt_tok;
								
								if nxt_tok > 0 && tokens[*i].t == Type::Type {
									match tokens[*i].val.as_ref() {
										"int" => "u64",
										_ => panic!("{}:{} Invalid type '{}' following 'unsigned'", tokens[*i].pos.line, tokens[*i].pos.col, tokens[*i].val)
									}
								} else {
									panic!("{}:{} Missing data type following 'unsigned'", tokens[*i].pos.line, tokens[*i].pos.col);
								}
							},
							"int" => "i64",
							_ => &tokens[*i].val
						};
					}
				},
				_ => output += &tokens[*i].val
			}
			
			if pos_change > 0 {
				*i += pos_change;
				*i += nxt(tokens, *i);
				
				output += ",";
				output = compile(tokens, i, output);
				*i += 1;
				output += &tokens[*i].val;
				output += ")";
			}
		}
	};
	
	output
}