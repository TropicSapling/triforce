use lib::{Token, Type, Type2, Function, FunctionArg};

macro_rules! last {
	($e:expr) => ($e[$e.len() - 1]);
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

pub fn parse(tokens: &mut Vec<Token>) {
	let mut functions: Vec<Function> = Vec::new();
	let mut func = false;
	let mut par_type = [Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void];
	let mut par_i = 0;
	
	for token in tokens {
		if token.t == Type::Whitespace {
			continue; // Ignore whitespace
		}
		
		if token.val == "func" {
			functions.push(Function {name: "", args: vec![], output: [Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void]});
			func = true;
		} else if func {
			if token.val == "-" { // Return type, WIP (needs to check for '>' as well)
				// WIP
			} else if token.val == "{" { // Function body
				func = false;
			} else if token.t == Type::Type { // Parameters
				par_type[par_i] = token.t2.clone();
				par_i += 1;
			} else if par_type[0] != Type2::Void {
				let last_item = functions.len() - 1;
				functions[last_item].args.push(FunctionArg {name: &token.val, t: par_type});
				
				par_type = [Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void, Type2::Void];
				par_i = 0;
			} else if token.t == Type::Var || token.t == Type::Op { // Function name
				let last_item = functions.len() - 1; // This can be replaced by the 'last!' macro once Rust has implemented non-lexical borrows
				functions[last_item].name = &token.val;
			}
		}
		
		// WIP
	}
	
	println!("{:#?}", functions);
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