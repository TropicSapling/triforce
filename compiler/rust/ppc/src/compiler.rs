use lib::{Token, Kind, Type, Function, FunctionArg};

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
			Literal(b) => if b {
				"true"
			} else {
				"false"
			},
			Op(ref val) => val,
			Reserved(ref val) => val,
			Str1(ref val) => val,
			Str2(ref val) => val,
			Type(ref typ) => match typ {
				&Array => "array",
				&Chan => "chan",
				&Const => "const",
				&Fraction => "fraction",
				&Func => "func",
				&Heap => "heap",
				&List => "list",
				&Only => "only",
				&Register => "register",
				&Stack => "stack",
				&Unique => "unique",
				&Volatile => "volatile",
				&Bool => "bool",
				&Char => "char",
				&Int => "int",
				&Pointer => "pointer",
				&Unsigned => "unsigned",
				&Void => "void",
			},
			Var(ref name, _) => name,
			_ => unreachable!()
		}
	});
}

macro_rules! get_val2 {
	($e:expr) => ({
		use lib::Kind::*;
		match $e {
			GroupOp(ref val) => val.to_string(),
			Literal(b) => if b {
				String::from("true")
			} else {
				String::from("false")
			},
			Number(int, fraction) => {
				int.to_string() + "." + &fraction.to_string()
			},
			Op(ref val) => val.to_string(),
			Reserved(ref val) => val.to_string(),
			Str1(ref val) => val.to_string(),
			Str2(ref val) => val.to_string(),
			Type(ref typ) => match typ {
				&Array => String::from("array"),
				&Chan => String::from("chan"),
				&Const => String::from("const"),
				&Fraction => String::from("fraction"),
				&Func => String::from("func"),
				&Heap => String::from("heap"),
				&List => String::from("list"),
				&Only => String::from("only"),
				&Register => String::from("register"),
				&Stack => String::from("stack"),
				&Unique => String::from("unique"),
				&Volatile => String::from("volatile"),
				&Bool => String::from("bool"),
				&Char => String::from("char"),
				&Int => String::from("int"),
				&Pointer => String::from("pointer"),
				&Unsigned => String::from("unsigned"),
				&Void => String::from("void"),
			},
			Var(ref name, _) => name.to_string()
		}
	});
}

macro_rules! def_builtin_funcs {
	($a:expr, $b:expr) => (vec![
		Function {
			name: String::from("+"),
			pos: 1,
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
			name: String::from("-"),
			pos: 1,
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
			name: String::from("*"),
			pos: 1,
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
			name: String::from("/"),
			pos: 1,
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
			name: String::from("%"),
			pos: 1,
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
			name: String::from("println"),
			pos: 0,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; No support for strings yet
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types ('int|fraction' in this case)
				}
			],
			precedence: 0,
			output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		}
	])
}

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
	let mut func_args = Vec::new();
	let mut par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
	
	// STAGE 1: DEFINE FUNCTIONS (this is done in a separate loop to allow function definitions to be placed both before and after function calls)
	let mut i = 0;
	while i < tokens.len() {
		let token = &tokens[i];
		
		let mut last_item = functions.len();
		if last_item != 0 {
			last_item -= 1;
		}
		
		match token.kind {
			Kind::Type(ref typ) if !func => match typ {
				&Type::Func => {
					functions.push(Function {name: String::from(""), pos: 0, args: vec![], precedence: 0, output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]});
					func_pos = i;
					func = true;
				},
				_ => ()
			},
			
			Kind::Type(ref typ) => match tokens[i + 1].kind {
				Kind::GroupOp(ref op) if op == "{" => {
					tokens[func_pos].children.borrow_mut().push(i);
					par_type[0] = typ.clone(); // TODO: Add support for returning multiple types
				},
				_ => ()
			},
			
			Kind::Var(ref name, ref typ) if func => if typ[0] == Type::Void || typ[0] == Type::Func { // Function name
				functions[last_item].name = name.to_string();
				functions[last_item].pos = functions[last_item].args.len();
				
				tokens[func_pos].children.borrow_mut().push(i);
			} else { // Function args
				functions[last_item].args.push(FunctionArg {name, typ: typ.clone()});
				func_args.push(i);
			},
			
			Kind::Op(ref op) if func => if op == "-" {
				match tokens[i + 1].kind {
					Kind::Op(ref op) if op == ">" => i += 1,
					_ => { // Operator (function) name
						functions[last_item].name += op;
						functions[last_item].pos = functions[last_item].args.len();
						
						if tokens[func_pos].children.borrow().len() < 1 {
							tokens[func_pos].children.borrow_mut().push(i);
						}
					}
				}
			} else if op == ";" { // End of function declaration
				functions[last_item].output = par_type.clone();
				if par_type[0] != Type::Void {
					functions[last_item].precedence = 1;
				}
				
				let func_name_pos = tokens[func_pos].children.borrow()[0];
				for arg in func_args {
					tokens[func_name_pos].children.borrow_mut().push(arg);
				}
				
				par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				func_args = Vec::new();
				func = false;
			} else { // Operator (function) name
				functions[last_item].name += op;
				functions[last_item].pos = functions[last_item].args.len();
				
				if tokens[func_pos].children.borrow().len() < 1 {
					tokens[func_pos].children.borrow_mut().push(i);
				}
			},
			
			Kind::GroupOp(ref op) if func => if op == "{" { // Function body
				functions[last_item].output = par_type.clone();
				if par_type[0] != Type::Void {
					functions[last_item].precedence = 1;
				}
				
				let func_name_pos = tokens[func_pos].children.borrow()[0];
				for arg in func_args {
					tokens[func_name_pos].children.borrow_mut().push(arg);
				}
				
				par_type = [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void];
				func_args = Vec::new();
				func = false;
				
				tokens[func_pos].children.borrow_mut().push(i);
				
				// Until the code below has been fixed, the compiler won't allow passing functions as arguments
				
/*				i += 1;
				
				let mut nests = 0;
				while i < tokens.len() {
					match tokens[i].kind {
						Kind::Var(ref name, ref mut typ) => {
							for arg in &functions[last_item].args {
								if arg.name == name {
									*typ = arg.typ.clone(); // Fix this by converting typ from Array to Vec?
									break;
								}
							}
						},
						
						Kind::GroupOp(ref op) if op == "}" => if nests > 0 {
							nests -= 1;
						} else {
							break;
						},
						
						Kind::GroupOp(ref op) if op == "{" => nests += 1,
						
						_ => ()
					}
					
					i += 1;
				} */
			},
			
			_ => ()
		}
		
		i += 1;
	}
	
	functions
}

fn parse_func(tokens: &Vec<Token>, func: (usize, &Function)) {
	let (mut i, def) = func;
	let start = i;
	let mut j = 0;
	let mut offset = 0;
	
	i -= 1;
	while i - j > 0 && j - offset < def.pos {
		match tokens[i - j].kind {
			Kind::Op(_) => {
				j += 1;
				while i - j > 0 {
					match tokens[i - j].kind {
						Kind::Op(_) => {
							j += 1;
							offset += 1;
						},
						_ => break
					}
				}
				j -= 1;
			},
			
			Kind::GroupOp(_) | Kind::Type(_) => {
				j += 1;
				offset += 1;
				continue;
			},
			
			_ => ()
		};
		
		let mut k = 0;
		while k < tokens.len() {
			if let Ok(children) = tokens[k].children.try_borrow() {
				if children.contains(&(i - j)) {
					break;
				}
			}
			
			k += 1;
		}
		
		if k < tokens.len() {
			j += 1;
			offset += 1;
			continue;
		} else {
			tokens[start].children.borrow_mut().insert(0, i - j);
		}
		
		j += 1;
	}
	
	i += 2;
	while i < tokens.len() {
		match tokens[i].kind {
			Kind::Op(_) => i += 1,
			_ => break
		}
	}
	
	j = 0;
	offset = 0;
	
	while i + j < tokens.len() && j - offset < def.args.len() - def.pos {
		let mut k = 0;
		while k < tokens.len() {
			if let Ok(children) = tokens[k].children.try_borrow() {
				if children.contains(&(i + j)) {
					break;
				}
			}
			
			k += 1;
		}
		
		if k < tokens.len() {
			j += 1;
			offset += 1;
			continue;
		} else {
			tokens[start].children.borrow_mut().push(i + j);
		}
		
		match tokens[i + j].kind {
			Kind::Op(_) => {
				j += 1;
				while i + j < tokens.len() {
					match tokens[i + j].kind {
						Kind::Op(_) => {
							j += 1;
							offset += 1;
						},
						_ => break
					}
				}
				j -= 1;
			},
			
			Kind::GroupOp(_) | Kind::Type(_) => {
				j += 1;
				offset += 1;
				continue;
			},
			
			_ => ()
		};
		
		j += 1;
	}
}

fn parse_statement(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize) -> Option<usize> {
	match tokens[*i + 1].kind {
		Kind::GroupOp(ref op) if op == "}" => return Some(*i),
		_ => ()
	};
	
	match tokens[*i].kind {
		Kind::Reserved(ref keyword) if keyword == "if" => {
			let start = *i;
			let mut body = tokens[*i].children.borrow_mut();
			*i += 1;
			
			let next = *i;
			if let Some(token) = parse_statement(tokens, functions, i) {
				body.push(token);
			} else {
				body.push(next);
			}
			
			body.push(*i);
			*i -= 1;
			
			return Some(start);
		},
		_ => ()
	}
	
	let start = *i;
	let mut lowest = None;
	while *i < tokens.len() {
		let mut highest: Option<(usize, &Function, u8)> = None;
		let mut depth = 0;
		*i = start;
		while *i < tokens.len() {
			if tokens[*i].children.borrow().len() < 1 {
				match tokens[*i].kind {
					Kind::Var(ref name, _) => if let Some(def) = is_defined(functions, name) {
						match highest {
							Some(func) => if (def.precedence > func.1.precedence && depth == func.2) || depth > func.2 {
								highest = Some((*i, def, depth));
							},
							None => highest = Some((*i, def, depth))
						};
					},
					
					Kind::Op(ref op) if op == ";" => break,
					Kind::GroupOp(ref op) if op == "}" => break,
					Kind::GroupOp(ref op) if op == "{" => break,
					
					Kind::Op(ref op) => {
						let mut name = op.to_string();
						let start = *i;
						
						*i += 1;
						while *i < tokens.len() {
							match tokens[*i].kind {
								Kind::Op(ref op) => name += op,
								_ => break
							}
							
							*i += 1;
						}
						*i -= 1;
						
						if let Some(def) = is_defined(functions, &name) {
							match highest {
								Some(func) => if (def.precedence > func.1.precedence && depth == func.2) || depth > func.2 {
									highest = Some((start, def, depth));
								},
								None => highest = Some((start, def, depth))
							};
						} else if name == "->" {
							break;
						}
					},
					
					Kind::GroupOp(ref op) if op == "(" => depth += 1,
					Kind::GroupOp(ref op) if op == ")" => depth -= 1,
					
					_ => ()
				};
			} else if let Kind::Op(_) = tokens[*i].kind {
				*i += 1;
				while *i < tokens.len() {
					match tokens[*i].kind {
						Kind::Op(_) => (),
						_ => break
					}
					
					*i += 1;
				}
				*i -= 1;
			}
			
			*i += 1;
		}
		
		match highest {
			Some(func) => {
				lowest = Some(func.0);
				parse_func(tokens, (func.0, func.1));
			},
			None => break
		};
	}
	
	lowest
}

pub fn parse2(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize) {
	match tokens[*i].kind {
		Kind::GroupOp(ref op) if op == "{" => {
			let mut body = tokens[*i].children.borrow_mut();
			*i += 1;
			
			let next = *i;
			match tokens[*i].kind {
				Kind::GroupOp(ref op) if op == "}" => (),
				
				_ => if let Some(token) = parse_statement(tokens, functions, i) {
					body.push(token);
				} else {
					body.push(next);
				}
			};
		},
		
		_ => ()
	}
}

// OUTDATED FUNCTION
fn compile_token(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, func_def: &mut bool, mut output: String, taken: &mut Vec<usize>) -> String {
	use lib::Kind::*;
	use lib::Type::*;
	
	if taken.contains(i) {
		return output;
	}
	
	match tokens[*i].kind {
		GroupOp(ref op) => {
			if !*func_def {
				output += op;
			}
			
			let children = tokens[*i].children.borrow();
			for child in children.iter() {
				*i = *child;
				output = compile_token(tokens, functions, i, func_def, output, taken);
			}
			
			if children.len() > 0 {
				*i += 1;
				output = compile_token(tokens, functions, i, func_def, output, taken);
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
			"-" => match tokens[*i + 1].kind {
				Kind::Op(ref s) if s == ">" && !is_kind!(tokens[*i + 2].kind, Kind::Type(_)) => {
					output += "&";
					taken.push(*i);
					*i += 1;
				},
				_ => output += &op
			},
			_ => output += &op
		},
		
		Reserved(ref keyword) => match keyword.as_ref() {
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
		
		Type(ref typ) => match typ {
			&Func => {
				output += "fn ";
				
				let children = tokens[*i].children.borrow();
				
				// Function name & args
				*i = children[0];
				output = compile_token(tokens, functions, i, func_def, output, taken);
				
				if children.len() > 2 {
					// Return type
					output += "->";
					*i = children[1];
					
					match tokens[*i].kind {
						Type(ref typ) => match typ {
							&Array | &Chan | &Const | &Fraction | &Func | &Heap | &List | &Only | &Register | &Stack | &Unique | &Volatile => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
							&Bool => output += "bool",
							&Char => output += "char",
							&Int => match tokens[*i - 1].kind {
								Kind::Type(ref typ) if typ == &Unsigned => output += "u64", // TMP
								_ => output += "i64" // TMP
							},
							&Pointer => output += "*", // TMP
							&Unsigned => (),
							&Void => (), // May be changed
						},
						_ => {
							let val = get_val2!(tokens[*i].kind);
							panic!("{}:{} Invalid return type '{}'", tokens[*i].pos.line, tokens[*i].pos.col, val);
						}
					}
					
					// Function body
					*i = children[2];
					output = compile_token(tokens, functions, i, func_def, output, taken);
				} else {
					// Function body
					*i = children[1];
					output = compile_token(tokens, functions, i, func_def, output, taken);
				}
			},
			_ => () // TMP
		},
		
		Var(ref name, ref types) => {
			if name == "init" {
				output += "main";
			} else {
				output += name;
			}
			
			if is_defined(&functions, &name).is_some() { // Function call or definition
				// ???
				
				let children = tokens[*i].children.borrow();
				if children.len() > 0 {
					output += "(";
					
					*func_def = true;
					for (i, child) in children.iter().enumerate() {
						let mut c = *child;
						output = compile_token(tokens, functions, &mut c, func_def, output, taken); // rename 'func_def' to 'ignore_parentheses'? Or at least 'func' to clarify it's not only definitions but also calls
						if i + 1 < children.len() {
							output += ",";
						}
					}
					*func_def = false;
					
					output += ")";
				} else {
					output += "()";
				}
			} else {
				output += ":";
				
				for (t, typ) in types.iter().enumerate() {
					match typ {
						&Array | &Chan | &Const | &Fraction | &Func | &Heap | &List | &Only | &Register | &Stack | &Unique | &Volatile => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
						&Bool => output += "bool",
						&Char => output += "char",
						&Int => if t > 0 {
							match &types[t - 1] {
								&Unsigned => output += "u64", // TMP
								_ => output += "i64" // TMP
							}
						} else {
							output += "i64";
						},
						&Pointer => output += "*", // TMP
						&Unsigned => (),
						&Void => (), // May be changed
					}
				}
			}
		}
		
/*		Kind::Whitespace(ref typ) => match typ {
			&Newline => output += "\n",
			&CarRet => output += "\r",
			&Tab => output += "\t",
			&Space => output += " "
		} */
	}
	
	taken.push(*i);
	
	output
}

fn compile_func(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, mut output: String) -> String {
	match tokens[*i].kind {
		Kind::Literal(b) => if b {
			output += "true";
		} else {
			output += "false";
		},
		
		Kind::Number(int, fraction) => {
			output += &int.to_string();
			if fraction != 0 {
				output += ".";
				output += &fraction.to_string();
			}
		},
		
		Kind::Str1(ref s) => {
			output += "\"";
			output += s;
			output += "\"";
		},
		
		Kind::Type(ref typ) => {
			use lib::Type::*;
			
			// TODO: Support unsigned int and other multiple-types
			
			match *typ {
				Array => (), // WIP
				Bool => output += "bool",
				Chan => (), // WIP
				Char => output += "char",
				Const => output += "const",
				Fraction => (), // WIP
				Func => output += "fn",
				Heap => (), // WIP
				Int => output += "isize",
				List => (), // WIP
				Only => (), // WIP
				Pointer => output += "&", // NOTE: Needs changing (for example pointer*2)
				Register => (), // WIP
				Stack => (), // WIP
				Unique => (), // WIP
				Unsigned => (), // WIP
				Void => (), // NOTE: Needs changing to 'output += "()"' once Void is not used for none-existing parameters (use None instead)
				Volatile => (), // WIP
			}
		}
		
		Kind::Var(ref name, ref typ) if typ[..] == [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] || typ[..] == [Type::Func, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] => {
			if let Some(_) = is_defined(functions, name) { // TMP until I've worked out passing functions as arguments
				output += if name == "init" {
					"main"
				} else if name == "println" {
					"println!"
				} else {
					name
				};
				output += "(";
				
				let args = tokens[*i].children.borrow();
				for (a, arg) in args.iter().enumerate() {
					*i = *arg;
					output = compile_func(tokens, functions, i, output);
					
					if a < args.len() - 1 {
						output += ","
					}
				}
				
				output += ")";
			} else {
				output += name;
			}
		},
		
		Kind::Var(ref name, ref typ) => {
			use lib::Type::*;
			
			output += name;
			output += ":";
			
			let mut unsigned = false;
			
			for t in typ {
				match *t {
					Array => (), // WIP
					Bool => output += "bool",
					Chan => (), // WIP
					Char => output += "char",
					Const => output += "const",
					Fraction => (), // WIP
					Func => output += "fn",
					Heap => (), // WIP
					Int => if unsigned {
						output += "usize";
					} else {
						output += "isize";
					},
					List => (), // WIP
					Only => (), // WIP
					Pointer => output += "&", // NOTE: Needs changing (for example pointer*2)
					Register => (), // WIP
					Stack => (), // WIP
					Unique => (), // WIP
					Unsigned => unsigned = true,
					Void => (), // NOTE: Needs changing to 'output += "()"' once Void is not used for none-existing parameters (use None instead)
					Volatile => (), // WIP
				}
			}
		},
		
		Kind::Op(ref op) => {
			let mut name = match op.as_ref() {
				"+" => "plus",
				"-" => "minus",
				"*" => "times",
				"/" => "div",
				"%" => "mod",
				"=" => "eq",
				"&" => "and",
				"|" => "or",
				"^" => "mod",
				"<" => "larrow",
				">" => "rarrow",
				"!" => "not",
				"~" => "binnot",
				"?" => "quest",
				":" => "colon",
				"." => "dot",
				"," => "comma",
				"@" => "at",
				";" => "semic",
				&_ => unreachable!()
			}.to_string();
			let start = *i;
					
			*i += 1;
			while *i < tokens.len() {
				match tokens[*i].kind {
					Kind::Op(ref op) => name += match op.as_ref() {
						"+" => "plus",
						"-" => match tokens[*i + 1].kind {
							Kind::Op(ref op) if op == ">" => break,
							_ => "minus"
						},
						"*" => "times",
						"/" => "div",
						"%" => "mod",
						"=" => "eq",
						"&" => "and",
						"|" => "or",
						"^" => "xor",
						"<" => "larrow",
						">" => "rarrow",
						"!" => "not",
						"~" => "binnot",
						"?" => "quest",
						":" => "colon",
						"." => "dot",
						"," => "comma",
						"@" => "at",
						";" => "semic",
						&_ => unreachable!()
					},
					_ => break
				}
				
				*i += 1;
			}
			*i -= 1;
			
			let args = tokens[start].children.borrow();
			
			if name == "plus" || name == "minus" || name == "times" || name == "div" || name == "mod" || name == "eq" || name == "eqeq" || name == "andand" || name == "or" || name == "oror" || name == "larrow" || name == "rarrow" {
				*i = args[0];
				output = compile_func(tokens, functions, i, output);
				
				output += match name.as_ref() {
					"plus" => "+",
					"minus" => "-",
					"times" => "*",
					"div" => "/",
					"mod" => "%",
					"eq" => "=",
					"eqeq" => "==",
					"andand" => "&&",
					"or" => "|",
					"oror" => "||",
					"larrow" => "<",
					"rarrow" => ">",
					&_ => unreachable!()
				};
				
				*i = args[1];
				output = compile_func(tokens, functions, i, output);
			} else {
				output += &name;
				output += "(";
				
				for (a, arg) in args.iter().enumerate() {
					*i = *arg;
					output = compile_func(tokens, functions, i, output);
					
					if a < args.len() - 1 {
						output += ","
					}
				}
				
				output += ")";
			}
		},
		
		Kind::Reserved(ref keyword) if keyword == "if" => {
			output += "if ";
			
			let children = tokens[*i].children.borrow();
			
			*i = children[0];
			output = compile_func(tokens, functions, i, output);
			output += " {";
			
			let statements = tokens[children[1]].children.borrow();
			for statement in statements.iter() {
				*i = *statement;
				output = compile_func(tokens, functions, i, output);
			}
			
			output += "}";
		},
		
		_ => () // WIP
	};
	
	output
}

pub fn compile(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, func_def: &mut bool, mut output: String, taken: &mut Vec<usize>) -> String {
	use lib::Type::*;
	use lib::Kind::*;
	
	let children = tokens[*i].children.borrow();
	
	match tokens[*i].kind {
		Type(ref typ) if typ == &Func => {
			output += "fn ";
			
			*i = children[0];
			output = compile_func(tokens, functions, i, output);
			
			if children.len() > 1 {
				let body = if children.len() > 2 {
					*i = children[1];
					output += "->";
					output = compile_func(tokens, functions, i, output);
					2
				} else {
					1
				};
				
				output += "{";
				
				let statements = tokens[children[body]].children.borrow();
				let statements_len = statements.len();
				for statement in statements.iter() {
					*i = *statement;
					output = compile_func(tokens, functions, i, output);
					if statements_len > 1 || body == 1 {
						output += ";"
					}
				}
				
				output += "}";
			} else {
				output += ";";
			}
		},
		
		Kind::Var(ref name, _) if name == "#" => {
			while *i < tokens.len() {
				match tokens[*i].kind {
					Kind::GroupOp(ref op) if op == "]" => {
						output += "]";
						break;
					},
					
					_ => {
						output += get_val!(tokens[*i].kind); // Will probably be changed
						*i += 1;
					}
				}
			}
		}
		
		_ => ()
	}
	
	return output;
	
	// OUTDATED CODE BELOW (intentionally unreachable)
	
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
					output = compile_token(tokens, functions, &mut c, func_def, output, taken);
					if i + 1 < children.len() {
						output += ",";
					}
				}
				
				output += ")";
			} else {
				output += ":";
				
				for typ in types {
					match typ {
						&Func => (), // TMP
						&Array | &Chan | &Const | &Fraction | &Func | &Heap | &List | &Only | &Register | &Stack | &Unique | &Volatile => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
						&Bool => output += "bool",
						&Char => output += "char",
						&Int => match tokens[*i - 1].kind {
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
				output = compile(tokens, functions, i, func_def, output, taken);
			}
		},
		
		Kind::Op(ref op) => match op.as_ref() {
			"@" => output += "*",
			"-" if get_val!(tokens[*i + 1].kind) == ">" => if *func_def {
				output += "-> ";
				
				*func_def = false;
/*				*i += 2;
				*i += nxt(&tokens, *i); */
				*i += 3;
				
				match tokens[*i].kind {
					Kind::Type(ref typ) => match typ {
						&Array | &Chan | &Const | &Fraction | &Func | &Heap | &List | &Only | &Register | &Stack | &Unique | &Volatile => panic!("{}:{} Unimplemented token '{}'", tokens[*i].pos.line, tokens[*i].pos.col, get_val!(tokens[*i].kind)),
						&Bool => output += "bool",
						&Char => output += "char",
						&Int => match tokens[*i - 1].kind {
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
		
/*		Kind::Whitespace(ref typ) => match typ {
			&Newline => output += "\n",
			&CarRet => output += "\r",
			&Tab => output += "\t",
			&Space => output += " "
		}, */
		
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