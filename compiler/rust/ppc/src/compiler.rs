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
			GroupOp(ref val) => val,
			Op(ref val) => val,
			Reserved(ref val) => val,
			Str1(ref val) => val,
			Str2(ref val) => val,
			_ => panic!("")
		}
	});
}

macro_rules! is_val {
	($e:expr, $pattern:pat, $var:expr, $val:expr) => ({
		match $e {
			$pattern => $var == $val,
			_ => false
		}
	});
}

macro_rules! group_expr {
	($start:expr, $end:expr, $tokens:expr, $token:expr, $i:expr) => ({
		let mut j = 1;
		let mut nests = 0;
		while $i + j < $tokens.len() && (nests > 0 || !is_val!($tokens[$i + j].kind, Kind::GroupOp(ref val), val, $end)) {
			(*$token.children.borrow_mut()).push($i + j);
			
			match $tokens[$i + j].kind {
				Kind::GroupOp(ref val) => match val.as_ref() {
					$start => nests += 1,
					$end => nests -= 1,
					&_ => ()
				},
				_ => ()
			}
			
			j += 1;
		}
	})
}

macro_rules! def_builtin_funcs {
	($a:expr, $b:expr) => (vec![
		Function {
			name: "+",
			pos: 0, // Not a real pos, but it will be ignored anyway
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				}
			],
			precedence: 245, // NOTE: 0 is *lowest* precedence, not highest. Highest precedence is 255.
			output: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
		},
		
		Function {
			name: "-",
			pos: 0, // Not a real pos, but it will be ignored anyway
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				}
			],
			precedence: 245,
			output: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
		},
		
		Function {
			name: "*",
			pos: 0, // Not a real pos, but it will be ignored anyway
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				}
			],
			precedence: 246,
			output: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
		},
		
		Function {
			name: "/",
			pos: 0, // Not a real pos, but it will be ignored anyway
			args: vec![
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				},
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				}
			],
			precedence: 246,
			output: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
		},
		
		Function {
			name: "%",
			pos: 0, // Not a real pos, but it will be ignored anyway
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				}
			],
			precedence: 246,
			output: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
		},
		
		Function {
			name: "**",
			pos: 0, // Not a real pos, but it will be ignored anyway
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				}
			],
			precedence: 247,
			output: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
		}
	])
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

/* fn group(tokens: &mut Vec<Token>, i: &mut usize, op: &'static str, op_close: &'static str) {
	let mut tok_str = String::from(op);
	
	while !is_val!(tokens[*i].kind, Kind::GroupOp(ref val), val, op_close) {
		*i += 1;
		tok_str = compile(tokens, i, tok_str);
	}
	
	tokens[*i].kind = Kind::Var(tok_str, Type::Void);
	
	*i -= 1;
} */

fn is_defined<'a>(defs: &'a Vec<Function>, call: &str) -> Option<&'a Function<'a>> {
	for def in defs {
		if def.name == call {
			return Some(&def);
		}
	}
	
	None
}

pub fn parse<'a>(tokens: &'a Vec<Token>, func_par_a: &'a str, func_par_b: &'a str) -> Vec<Function<'a>> {
	let mut functions: Vec<Function> = def_builtin_funcs!(func_par_a, func_par_b);
	let mut func = false;
	let mut func_pos = 0;
	let mut par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
	
	// STAGE 1: DEFINE FUNCTIONS (this is done in a separate loop to allow function definitions to be placed both before and after function calls)
	for (i, token) in tokens.iter().enumerate() {
		if is_kind!(token.kind, Kind::Whitespace(_)) {
			continue; // Ignore whitespace
		}
		
		let mut last_item = functions.len();
		if last_item != 0 {
			last_item -= 1;
		}
		
		match token.kind {
			Kind::Whitespace(ref typ) => if func {
				tokens[func_pos].children.borrow_mut().push(i);
			},
			
			Kind::Type(ref typ) if !func => match typ {
				&Type::Func => {
					functions.push(Function {name: "", pos: 0, args: vec![], precedence: 0, output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]});
					func_pos = i;
					func = true;
				},
				_ => ()
			},
			
			Kind::Type(ref typ) => match tokens[i + nxt(&tokens, i)].kind {
				Kind::GroupOp(ref op) if op == "{" => tokens[func_pos].children.borrow_mut().push(i),
				_ => ()
			},
			
			Kind::Var(ref name, ref typ) => if typ[0] == Type::Void { // Function name
				functions[last_item].name = name;
				functions[last_item].pos = functions[last_item].args.len();
				
				tokens[func_pos].children.borrow_mut().push(i);
			} else { // Function args
				functions[last_item].args.push(FunctionArg {name, typ: typ.clone()});
				par_type = typ.clone();
			},
			
			Kind::Op(ref op) => if functions[last_item].name == "" { // Operator (function) name
				functions[last_item].name = op;
				functions[last_item].pos = functions[last_item].args.len();
				
				tokens[func_pos].children.borrow_mut().push(i);
			} else if op == ";" { // End of function declaration
				functions[last_item].output = par_type.clone();
				if par_type[0] != Type::Void {
					functions[last_item].precedence = 1;
				}
				
				par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				func = false;
			},
			
			Kind::GroupOp(ref op) => if op == "{" { // Function body
				functions[last_item].output = par_type.clone();
				if par_type[0] != Type::Void {
					functions[last_item].precedence = 1;
				}
				
				par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				func = false;
				
				tokens[func_pos].children.borrow_mut().push(i);
			},
			
			_ => ()
		}
		
/*		if is_val!(token.kind, Kind::Type(ref val), val, &Type::Func) {
			functions.push(Function {name: "", pos: 0, args: vec![], precedence: 0, output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]});
			func_pos = i;
			func = true;
		} else if func {
			if match token.kind {
				Kind::GroupOp(ref val) => val == "{", // Function body
				Kind::Op(ref val) => val == ";", // End of function declaration
				_ => false
			} {
				functions[last_item].output = par_type.clone();
				if par_type[0] != Type::Void {
					functions[last_item].precedence = 1;
				}
				
				par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				type_i = 0;
				func = false;
			} else if is_kind!(token.kind, Kind::Type(_)) { // Parameter / return types
				let val = match token.kind {
					Kind::Type(ref val) => val,
					_ => &Type::Void
				};
				par_type[type_i] = val.clone();
				type_i += 1;
			} else if par_type[0] != Type::Void {
				let name = match token.kind {
					Kind::Var(ref name, _) => name,
					_ => panic!("")
				};
				functions[last_item].args.push(FunctionArg {name: name, typ: par_type});
				
				par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				type_i = 0;
			} else if functions[last_item].name == "" && (is_kind!(token.kind, Kind::Var(_,_)) || is_kind!(token.kind, Kind::Op(_))) { // Function name
				let name = match token.kind {
					Kind::Var(ref name, _) => name,
					_ => panic!("")
				};
				functions[last_item].name = name;
				functions[last_item].pos = functions[last_item].args.len();
				
				tokens[func_pos].children.borrow_mut().push(i);
			} else if get_val!(token.kind) == "-" || get_val!(token.kind) == ">" {
				tokens[func_pos].children.borrow_mut().push(i);
			}
		} */
	}
	
	// STAGE 2: ORGANISE FUNCTION CALLS
	let mut i = 0;
	while i < tokens.len() {
		let token = &tokens[i];
		
		if is_kind!(token.kind, Kind::Var(_,_)) || is_kind!(token.kind, Kind::Op(_)) {
			let val = match token.kind {
				Kind::Var(ref val, _) => val,
				Kind::Op(ref val) => val,
				_ => panic!("")
			}; // Probably needs fixing
			
			if let Some(def) = is_defined(&functions, &val) {
				if def.pos > 0 {
					let mut j = 0;
					let mut k = 0;
					while i - j > 0 && j - k < def.pos && !is_val!(tokens[i - j].kind, Kind::Op(ref val), val, ";") { // NOTE: comparison may need to be changed
						j += prev(&tokens, i - j);
						
						match tokens[i - j].kind { // NEEDS FIXING; will not correctly parse args with parentheses
							Kind::GroupOp(ref op) => {
								let mut nests = 0;
								let start_op = match op.as_ref() {
									")" => "(",
									"}" => "{",
									"]" => "[",
									&_ => panic!("")
								};
								
								let prev_tok = prev(&tokens, i - j);
								j += prev_tok;
								k += prev_tok;
								while i - j > 0 && (nests > 0 || !is_val!(tokens[i - j].kind, Kind::GroupOp(ref val), val, start_op)) {
									match tokens[i - j].kind {
										Kind::GroupOp(ref val) => if val == op {
											nests += 1;
										} else if val == start_op {
											nests -= 1;
										},
										_ => ()
									}
									
									let prev_tok = prev(&tokens, i - j);
									j += prev_tok;
									k += prev_tok;
								}
								
								(*token.children.borrow_mut()).push(i - j);
							},
							_ => (*token.children.borrow_mut()).push(i - j) // Will this cause the vector to be backwards? If so fix later
						}
					}
				}
				
				let mut j = 0;
				while i + j < tokens.len() && j < def.args.len() - def.pos && !is_val!(tokens[i + j].kind, Kind::Op(ref val), val, ";") {
					j += nxt(&tokens, i + j);
					
					(*token.children.borrow_mut()).push(i + j);
					
					match tokens[i + j].kind { // NEEDS FIXING; will not correctly parse args with parentheses
						Kind::GroupOp(ref op) => {
							let mut nests = 0;
							let end_op = match op.as_ref() {
								"(" => ")",
								"{" => "}",
								"[" => "]",
								&_ => panic!("")
							};
							
							j += nxt(&tokens, i + j);
							while i + j < tokens.len() && (nests > 0 || !is_val!(tokens[i + j].kind, Kind::GroupOp(ref val), val, end_op)) {
								match tokens[i + j].kind {
									Kind::GroupOp(ref val) => if val == op {
										nests += 1;
									} else if val == end_op {
										nests -= 1;
									},
									_ => ()
								}
								
								j += nxt(&tokens, i + j);
							}
						},
						_ => ()
					}
				}
			}
		} else if is_kind!(token.kind, Kind::GroupOp(_)) {
			let val = match token.kind {
				Kind::GroupOp(ref val) => val,
				_ => panic!("")
			};
			
			match val.as_ref() {
				"(" => group_expr!("(", ")", tokens, token, i),
				"{" => group_expr!("{", "}", tokens, token, i),
				"[" => group_expr!("[", "]", tokens, token, i),
				&_ => (),
			}
		}
		
		i += 1;
	}
	
	// STAGE 3: FURTHER ORGANISATION BASED ON PRECEDENCE
	let mut i = 0;
	while i < tokens.len() {
		match tokens[i].kind {
			Kind::Var(ref name, _) => if let Some(def) = is_defined(&functions, name) {
				let mut children = tokens[i].children.borrow_mut();
				
				for child in children.iter_mut() {
					let mut j = 0;
					while j < tokens.len() {
						if j != i {
							match tokens[j].kind {
								Kind::Var(ref name, _) => if let Some(def2) = is_defined(&functions, name) {
									let mut children2 = tokens[j].children.borrow_mut();
									
									for child2 in children2.iter() {
										if *child == *child2 {
											if def.precedence < def2.precedence {
												*child = j;
											} else if j < i && def.precedence == def2.precedence {
												*child = j;
											}
										}
									}
								},
								_ => ()
							}
						}
						
						j += 1;
					}
				}
			},
			
			// Add support for GroupOp?
			
			_ => ()
		}
		
		i += 1;
	}
	
	functions
}

fn compile_token(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, func_def: &mut bool, mut output: String) -> String {
	use lib::Kind::*;
	use lib::Type::*;
	use lib::Whitespace::*;
	
	match tokens[*i].kind {
		GroupOp(ref op) => {
			if !*func_def {
				output += op;
			}
			
			let children = tokens[*i].children.borrow();
			for child in children.iter() {
				*i = *child;
				output = compile_token(tokens, functions, i, func_def, output);
			}
			
			if children.len() > 0 {
				*i += 1;
				output = compile_token(tokens, functions, i, func_def, output);
			}
		},
		
		Literal(boolean) => if boolean {
			output += "true";
		} else {
			output += "false";
		},
		
		Number(int, fraction) => {
			output += &int.to_string();
			if fraction != 0 {
				output += ".";
				output += &fraction.to_string();
			}
		},
		
		Op(ref op) => match op.as_ref() {
			"@" => output += "*",
			"-" if get_val!(tokens[*i + 1].kind) == ">" && !is_kind!(tokens[*i + 1 + nxt(&tokens, *i + 1)].kind, Kind::Type(_)) => {
				output += "&";
				*i += 1;
			},
			_ => output += &op
		},
		
		Kind::Reserved(ref keyword) => match keyword.as_ref() {
			"async" | "from" | "receive" | "select" | "send" | "to" => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
			"import" => output += "use",
			"foreach" => output += "for",
			"as" => output += "@",
			"astype" => output += "as", // TMP; will be replaced with (<type>) <variable>
			_ => output += &keyword
		},
		
		Str1(ref s) => { // TMP; will be replaced with C-style (null terminated) strings
			output += "\"";
			output += &s;
			output += "\"";
		},
		
		Str2(ref s) => { // TMP; will be replaced with P+ style strings
			output += "\"";
			output += &s;
			output += "\"";
		},
		
/*		Type(ref typ) => match typ {
			&Array | &Chan | &Const | &Fraction | &Heap | &List | &Only | &Register | &Stack | &Unique | &Volatile => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
			&Bool => output += "bool",
			&Char => output += "char",
			&Func => output += "fn",
			&Int => match tokens[*i - prev(&tokens, *i)].kind {
				Type(ref typ) if typ == &Unsigned => output += "u64", // TMP
				_ => output += "i64" // TMP
			},
			&Pointer => output += "*", // TMP
			&Unsigned => (),
			&Void => output += "()"
		}, */
		
		Type(_) => (), // TMP
		
		Var(ref name, ref types) => {
			output += name;
			if let Some(def) = is_defined(&functions, &name) {
				// ???
				
				let children = tokens[*i].children.borrow();
				if children.len() > 0 { // Function call or definition
					output += "(";
					
					for (i, child) in children.iter().enumerate() {
						let mut c = *child;
						output = compile_token(tokens, functions, &mut c, func_def, output);
						if i + 1 < children.len() {
							output += ",";
						}
					}
					
					output += ")";
				}
			} else {
				output += ":";
				
				for typ in types {
					match typ {
						&Array | &Chan | &Const | &Fraction | &Func | &Heap | &List | &Only | &Register | &Stack | &Unique | &Volatile => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
						&Bool => output += "bool",
						&Char => output += "char",
						&Int => match tokens[*i - prev(&tokens, *i)].kind {
							Kind::Type(ref typ) if typ == &Unsigned => output += "u64", // TMP
							_ => output += "i64" // TMP
						},
						&Pointer => output += "*", // TMP
						&Unsigned => (),
						&Void => (), // May be changed
					}
					
					output += "";
				}
			}
		},
		
		Kind::Whitespace(ref typ) => match typ {
			&Newline => output += "\n",
			&CarRet => output += "\r",
			&Tab => output += "\t",
			&Space => output += " "
		}
	}
	
	output
}

pub fn compile(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, func_def: &mut bool, mut output: String) -> String {
	use lib::Type::*;
	use lib::Whitespace::*;
	
	let children = tokens[*i].children.borrow();
	
	match tokens[*i].kind {
		Kind::Var(ref name, ref types) => {
			/* match tokens[*child].kind {
				Kind::Var(_,_) => {
					let mut i2 = *child;
					output = compile(tokens, functions, i2, j, output);
				},
				
				Kind::GroupOp(ref op) => {
					output += op;
					
					for child in children.iter() {
						*i = *child;
						output = compile(tokens, functions, i, j, output);
					}
					
					if children.len() > 0 {
						*i += 1;
						output = compile(tokens, functions, i, j, output);
					}
				},
				
				_ => output = compile_token(tokens, functions, i, j, output)
			} */
			
			output += name;
			if children.len() > 0 { // Function call or definition
				output += "(";
				
				for (i, child) in children.iter().enumerate() {
					let mut c = *child;
					output = compile_token(tokens, functions, &mut c, func_def, output);
					if i + 1 < children.len() {
						output += ",";
					}
				}
				
				output += ")";
			} else {
				output += ":";
				
				for typ in types {
					match typ {
						&Array | &Chan | &Const | &Fraction | &Func | &Heap | &List | &Only | &Register | &Stack | &Unique | &Volatile => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
						&Bool => output += "bool",
						&Char => output += "char",
						&Int => match tokens[*i - prev(&tokens, *i)].kind {
							Kind::Type(ref typ) if typ == &Unsigned => output += "u64", // TMP
							_ => output += "i64" // TMP
						},
						&Pointer => output += "*", // TMP
						&Unsigned => (),
						&Void => () // May be changed
					}
					
					output += "";
				}
			}
		},
		
		Kind::GroupOp(ref op) => {
//			output += op;
			
			for child in children.iter() {
				*i = *child;
//				output = compile(tokens, functions, i, j, output);
			}
			
			if children.len() > 0 {
				*i += 1;
				output = compile(tokens, functions, i, func_def, output);
			}
		},
		
		Kind::Op(ref op) => match op.as_ref() {
			"@" => output += "*",
			"-" if get_val!(tokens[*i + 1].kind) == ">" => if *func_def {
				output += "-> ";
				
				*func_def = false;
				*i += 2;
				*i += nxt(&tokens, *i);
				
				match tokens[*i].kind {
					Kind::Type(ref typ) => match typ {
						&Array | &Chan | &Const | &Fraction | &Func | &Heap | &List | &Only | &Register | &Stack | &Unique | &Volatile => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
						&Bool => output += "bool",
						&Char => output += "char",
						&Int => match tokens[*i - prev(&tokens, *i)].kind {
							Kind::Type(ref typ) if typ == &Unsigned => output += "u64", // TMP
							_ => output += "i64" // TMP
						},
						&Pointer => output += "*", // TMP
						&Unsigned => (),
						&Void => output += "()"
					},
					_ => panic!("{}:{} Invalid placement of token.", tokens[*i].pos.line, tokens[*i].pos.col) // WIP; error msg will be improved
				}
			} else {
				output += "&";
				*i += 1;
			},
			_ => output += &op
		},
		
		Kind::Reserved(ref keyword) => match keyword.as_ref() {
			"async" | "from" | "receive" | "select" | "send" | "to" => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
			"import" => output += "use",
			"foreach" => output += "for",
			"as" => output += "@",
			"astype" => output += "as", // TMP; will be replaced with (<type>) <variable>
			_ => output += &keyword
		},
		
		Kind::Type(ref typ) => match typ {
/*			&Array | &Chan | &Const | &Fraction | &Heap | &List | &Only | &Register | &Stack | &Unique | &Volatile => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
			&Bool => output += "bool",
			&Char => output += "char", */
			&Func => {
				output += "fn";
				*func_def = true;
			},
/*			&Int => match tokens[*i - prev(&tokens, *i)].kind {
				Kind::Type(ref typ) if typ == &Unsigned => output += "u64", // TMP
				_ => output += "i64" // TMP
			},
			&Pointer => output += "*", // TMP
			&Unsigned => (),
			&Void => output += "()" */
			&_ => () // TMP
		},
		
		Kind::Whitespace(ref typ) => match typ {
			&Newline => output += "\n",
			&CarRet => output += "\r",
			&Tab => output += "\t",
			&Space => output += " "
		},
		
		_ => ()
	}
	
/*	let val = {
		use lib::Kind::*;
		match tokens[*i].kind {
			GroupOp(ref val) => val.clone(),
			Literal(ref val) => if *val { String::from("true") } else { String::from("false") }, // TMP
			Number(ref val, ref val2) => ,
			Op(ref val) => val.clone(),
			Reserved(ref val) => val.clone(),
			Str1(ref val) => val.clone(),
			Str2(ref val) => val.clone(),
			Type(ref val) => ,
			Var(ref val, ref val2) => ,
			Whitespace(ref val) => ,
			_ => panic!("")
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
	}; */
	
	output
}