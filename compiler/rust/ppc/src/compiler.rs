use lib::Token;

fn nxt(tokens: &Vec<Token>, i: usize) -> usize {
	let mut j: usize = 0;
	while {
		j += 1;
		
		i + j < tokens.len() && tokens[i + j].t == "whitespace"
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
	tokens[*i].t = "variable";
	
	*i -= 1;
}

pub fn parse(tokens: Vec<Token>) -> Vec<Token> {
	tokens // WIP
}

pub fn compile(mut tokens: &mut Vec<Token>, i: &mut usize, mut output: String) -> String {
	match tokens[*i].val.as_ref() {
		"array" | "chan" | "fraction" | "heap" | "list" | "number" | "register" | "stack" | "async" | "from" | "receive" | "select" | "send" | "to" => panic!("Unimplemented token '{}'", tokens[*i].val),
		"@" => output += "*",
		"-" if tokens[*i + 1].val == ">" && tokens[*i + 1 + nxt(tokens, *i + 1)].t != "type" => {
			output += "&";
			*i += 1;
		},
		"(" => group(&mut tokens, i, "(", ")"),
		"[" => group(&mut tokens, i, "[", "]"),
		"{" => group(&mut tokens, i, "{", "}"),
		"init" => output += "main",
		"func" => output += "fn",
		"import" => output += "use",
		_ => {
			let pos_change = match tokens[*i].t {
				"str1" | "str2" | "number" | "literal" | "variable" => {
					let nxt_tok = nxt(tokens, *i);
					if nxt_tok > 0 && tokens[*i + nxt_tok].t == "variable" {
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
				"str1" => {
					output += "\"";
					output += &tokens[*i].val;
					output += "\"";
				},
				"str2" => {
					output += "'";
					output += &tokens[*i].val;
					output += "'";
				},
				"type" => {
					let nxt_tok = nxt(tokens, *i);
					if nxt_tok > 0 && tokens[*i + nxt_tok].t == "variable" {
						output += &tokens[*i + nxt_tok].val;
						output += ":";
						output += match tokens[*i].val.as_ref() {
							"int" => "i64",
							_ => &tokens[*i].val
						};
						
						*i += nxt_tok;
					} else {
						output += match tokens[*i].val.as_ref() {
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