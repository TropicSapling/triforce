use std::usize;
use std::cell::RefMut;
use lib::{Token, Kind, Type, Function, FunctionArg};

macro_rules! get_val {
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
				&Macro => String::from("macro"),
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

macro_rules! def_builtin_op {
	($a:expr, $b:expr, $name:expr, $typ1:expr, $typ2:expr, $output:expr, $precedence:expr) => (Function {
		name: String::from($name),
		pos: 1,
		args: vec![
			FunctionArg {
				name: $a,
				typ: vec![vec![$typ1]]
			},
			FunctionArg {
				name: $b,
				typ: vec![vec![$typ2]]
			}
		],
		precedence: $precedence, // NOTE: 0 is *lowest* precedence, not highest. Highest precedence is 255.
		output: if $output == Type::Void {
			vec![vec![]]
		} else {
			vec![vec![$output]]
		}
	})
}

macro_rules! def_builtin_funcs {
	($a:expr, $b:expr) => (vec![
		// WIP; 'typ' structure needs support for multiple types ('int|fraction' for these operators)
		def_builtin_op!($a, $b, "+", Type::Int, Type::Int, Type::Int, 245),
		def_builtin_op!($a, $b, "-", Type::Int, Type::Int, Type::Int, 245),
		def_builtin_op!($a, $b, "*", Type::Int, Type::Int, Type::Int, 246),
		def_builtin_op!($a, $b, "/", Type::Int, Type::Int, Type::Int, 246),
		def_builtin_op!($a, $b, "%", Type::Int, Type::Int, Type::Int, 246),
		
		// WIP; 'typ' structure needs support for multiple types (all types for these operators)
		def_builtin_op!($a, $b, "==", Type::Int, Type::Int, Type::Bool, 242),
		def_builtin_op!($a, $b, "!=", Type::Int, Type::Int, Type::Bool, 242),
		def_builtin_op!($a, $b, "<", Type::Int, Type::Int, Type::Bool, 243),
		def_builtin_op!($a, $b, "<=", Type::Int, Type::Int, Type::Bool, 243),
		def_builtin_op!($a, $b, ">", Type::Int, Type::Int, Type::Bool, 243),
		def_builtin_op!($a, $b, ">=", Type::Int, Type::Int, Type::Bool, 243),
		
		def_builtin_op!($a, $b, "&&", Type::Bool, Type::Bool, Type::Bool, 238),
		def_builtin_op!($a, $b, "||", Type::Bool, Type::Bool, Type::Bool, 237),
		
		def_builtin_op!($a, $b, "<<", Type::Int, Type::Int, Type::Int, 244),
		def_builtin_op!($a, $b, ">>", Type::Int, Type::Int, Type::Int, 244),
		def_builtin_op!($a, $b, "^", Type::Int, Type::Int, Type::Int, 240),
		
		// WIP; 'macro' types are not yet implemented
		def_builtin_op!($a, $b, "=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!($a, $b, "+=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!($a, $b, "-=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!($a, $b, "*=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!($a, $b, "/=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!($a, $b, "%=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!($a, $b, ">>=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!($a, $b, "<<=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!($a, $b, "^=", Type::Int, Type::Int, Type::Void, 0),
		
		Function {
			name: String::from("println"),
			pos: 0,
			args: vec![
				FunctionArg {
					name: $a,
					typ: vec![vec![Type::Int]] // WIP; No support for strings yet
				}
			],
			precedence: 1,
			output: vec![]
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
	let mut par_type = vec![vec![]];
	
	// DEFINE FUNCTIONS (this is done in a separate loop to allow function definitions to be placed both before and after function calls)
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
					functions.push(Function {name: String::from(""), pos: 0, args: vec![], precedence: 1, output: vec![]});
					func_pos = i;
					func = true;
				},
				_ => ()
			},
			
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
					
					i += 1;
					tokens[func_pos].children.borrow_mut().push(i);
					
					i = end;
				},
				_ => ()
			},
			
			Kind::Var(ref name, ref typ) if func => if typ[0].len() == 0 || typ[0][0] == Type::Func { // Function name
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
				if functions[last_item].name == "**" {
					functions[last_item].precedence = 247;
				} else if par_type[0].len() > 0 {
					if func_args.len() == 1 {
						functions[last_item].precedence = 255;
					} else {
						functions[last_item].precedence = 2;
					}
				}
				
				let func_name_pos = tokens[func_pos].children.borrow()[0];
				for arg in func_args {
					tokens[func_name_pos].children.borrow_mut().push(arg);
				}
				
				par_type = vec![vec![]];
				func_args = Vec::new();
				func = false;
			} else if op != "|" { // Operator (function) name
				functions[last_item].name += op;
				functions[last_item].pos = functions[last_item].args.len();
				
				if tokens[func_pos].children.borrow().len() < 1 {
					tokens[func_pos].children.borrow_mut().push(i);
				}
			},
			
			Kind::GroupOp(ref op) if func => if op == "{" { // Function body
				functions[last_item].output = par_type.clone();
				if functions[last_item].name == "**" {
					functions[last_item].precedence = 247;
				} else if par_type[0].len() > 0 {
					if func_args.len() == 1 {
						functions[last_item].precedence = 255;
					} else {
						functions[last_item].precedence = 2;
					}
				}
				
				let func_name_pos = tokens[func_pos].children.borrow()[0];
				for arg in func_args {
					tokens[func_name_pos].children.borrow_mut().push(arg);
				}
				
				par_type = vec![vec![]];
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

fn parse_func(tokens: &Vec<Token>, func: (usize, &Function), functions: &Vec<Function>) {
	let (mut i, def) = func;
	let start = i;
	let mut j = 0;
	let mut offset = 0;
	
	i -= 1;
	while i - j > 0 && j - offset < def.pos {
		match tokens[i - j].kind {
			Kind::Op(ref op) if op == ";" => {
				j += 1;
				offset += 1;
				continue;
			},
			
			Kind::Op(ref op) => {
				let mut name = op.to_string();
				
				j += 1;
				while i - j > 0 {
					match tokens[i - j].kind {
						Kind::Op(ref op) => {
							name.insert(0, op.chars().next().unwrap());
							
							if let Some(_) = is_defined(functions, &name) {
								j += 1;
								offset += 1;
							} else {
								break;
							}
						},
						_ => break
					}
				}
				j -= 1;
			},
			
			Kind::GroupOp(ref op) if op == "{" => (),
			
			Kind::GroupOp(_) | Kind::Type(_) => {
				j += 1;
				offset += 1;
				continue;
			},
			
			_ => ()
		}
		
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
	
	if i >= tokens.len() {
		panic!("Unexpected EOF");
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
		
		let mut skip = (false, "");
		
		match tokens[i + j].kind {
			Kind::Op(ref op) if op == ";" => {
				j += 1;
				offset += 1;
				continue;
			},
			Kind::Op(ref op) => skip = (true, op),
			
			Kind::GroupOp(ref op) if op == "{" => (),
			
			Kind::GroupOp(_) | Kind::Type(_) => {
				j += 1;
				offset += 1;
				continue;
			},
			
			_ => ()
		}
		
		if k < tokens.len() {
			match tokens[i + j + 1].kind {
				Kind::Op(_) if skip.0 => offset += 1,
				_ => {
					j += 1;
					offset += 1;
					continue;
				}
			}
		} else {
			tokens[start].children.borrow_mut().push(i + j);
		}
		
		if skip.0 {
			let mut name = skip.1.to_string();
			
			j += 1;
			while i + j < tokens.len() {
				match tokens[i + j].kind {
					Kind::Op(ref op) => {
						name += op;
						
						if let Some(_) = is_defined(functions, &name) {
							j += 1;
							offset += 1;
						} else {
							break;
						}
					},
					_ => break
				}
			}
			j -= 1;
		}
		
		j += 1;
	}
	
	if i + j >= tokens.len() {
		panic!("Unexpected EOF");
	}
	
	if tokens[start].children.borrow().len() < 1 {
		tokens[start].children.borrow_mut().push(usize::MAX);
	}
}

fn parse_group(tokens: &Vec<Token>, i: usize, functions: &Vec<Function>) {
	let mut j = 1;
	let mut depth = 0;
	while i + j < tokens.len() {
		let mut k = 0;
		while k < tokens.len() {
			if let Ok(children) = tokens[k].children.try_borrow() {
				if children.contains(&(i + j)) {
					break;
				}
			}
			
			k += 1;
		}
		
		let mut skip = (false, "");
		
		match tokens[i + j].kind {
			Kind::Op(ref op) if op == ";" => {
				j += 1;
				continue;
			},
			Kind::Op(ref op) => skip = (true, op),
			
			Kind::GroupOp(ref op) if op == "{" => depth += 1,
			Kind::GroupOp(ref op) if op == "}" => if depth > 0 {
				depth -= 1;
			} else {
				break;
			},
			
			Kind::GroupOp(_) | Kind::Type(_) => {
				j += 1;
				continue;
			},
			
			_ => ()
		}
		
		if k < tokens.len() {
			match tokens[i + j + 1].kind {
				Kind::Op(_) if skip.0 => (),
				_ => {
					j += 1;
					continue;
				}
			}
		} else {
			tokens[i].children.borrow_mut().push(i + j);
		}
		
		if skip.0 {
			let mut name = skip.1.to_string();
			
			j += 1;
			while i + j < tokens.len() {
				match tokens[i + j].kind {
					Kind::Op(ref op) => {
						name += op;
						
						if let Some(_) = is_defined(functions, &name) {
							j += 1;
						} else {
							break;
						}
					},
					_ => break
				}
			}
			j -= 1;
		}
		
		j += 1;
	}
	
	if i + j >= tokens.len() {
		panic!("Unexpected EOF");
	}
}

fn get_parse_limit(tokens: &Vec<Token>, i: &mut usize) -> usize {
	let mut depth = 0;
	let mut dived = false;
	let mut limit = tokens.len();
	while *i < limit {
		match tokens[*i].kind {
			Kind::Op(ref op) if op == ";" => if depth == 0 {
				limit = *i;
				break;
			},
			
			Kind::Reserved(_) if depth == 0 => {
				*i -= 1;
				
				let mut depth = 0;
				while *i > 0 {
					match tokens[*i].kind {
						Kind::GroupOp(ref op) if op == "}" => depth += 1,
						Kind::GroupOp(ref op) if op == "{" => if depth > 1 {
							depth -= 1;
						} else {
							break;
						},
						
						_ => ()
					}
					
					*i -= 1;
				}
				
				limit = *i;
				break;
			},
			
			Kind::GroupOp(ref op) if op == "{" => {
				depth += 1;
				dived = true;
			},
			
			Kind::GroupOp(ref op) if op == "}" => if depth > 0 {
				depth -= 1;
			} else {
				if dived {
					*i -= 1;
					
					let mut depth = 0;
					while *i > 0 {
						match tokens[*i].kind {
							Kind::GroupOp(ref op) if op == "}" => depth += 1,
							Kind::GroupOp(ref op) if op == "{" => if depth > 1 {
								depth -= 1;
							} else {
								break;
							},
							
							_ => ()
						}
						
						*i -= 1;
					}
				}
				
				limit = *i;
				break;
			},
			
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
				
				if *i + 1 >= tokens.len() {
					panic!("Unexpected EOF");
				}
				
				if name == "->" {
					limit = start;
					break;
				}
			},
			
			_ => ()
		}
		
		*i += 1;
	}
	
	limit
}

fn parse_statement(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize) -> Option<usize> {
	match tokens[*i + 1].kind {
		Kind::GroupOp(ref op) if op == "}" => {
			*i += 1;
			return Some(*i - 1);
		},
		_ => ()
	}
	
	let start = *i;
	let limit = get_parse_limit(tokens, i);
	let mut lowest = None;
	
	loop {
		let mut highest: Option<(usize, Option<&Function>, u8)> = None;
		let mut depth = 0;
		let mut depth2 = 0;
		*i = start;
		while *i < limit {
			if tokens[*i].children.borrow().len() < 1 {
				match tokens[*i].kind {
					Kind::Var(ref name, _) => if let Some(def) = is_defined(functions, name) {
						match highest {
							Some(func) => match func.1 {
								Some(def2) => if (def.precedence > def2.precedence && depth + depth2 == func.2) || depth + depth2 > func.2 {
									highest = Some((*i, Some(def), depth + depth2));
								},
								None => if depth + depth2 > func.2 {
									highest = Some((*i, Some(def), depth + depth2));
								}
							},
							None => highest = Some((*i, Some(def), depth + depth2))
						}
					},
					
					Kind::GroupOp(ref op) if op == "{" => {
						match highest {
							Some(func) => if depth + depth2 >= func.2 {
								highest = Some((*i, None, depth + depth2));
							},
							None => highest = Some((*i, None, depth + depth2))
						}
						
						depth2 += 1;
					},
					
					Kind::GroupOp(ref op) if op == "}" => if depth2 > 0 {
						depth2 -= 1;
					},
					
					Kind::Op(ref op) if op != ";" => {
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
						
						if *i + 1 >= tokens.len() {
							panic!("Unexpected EOF");
						}
						
						if let Some(def) = is_defined(functions, &name) {
							match highest {
								Some(func) => match func.1 {
									Some(def2) => if (def.precedence > def2.precedence && depth + depth2 == func.2) || depth + depth2 > func.2 {
										highest = Some((start, Some(def), depth + depth2));
									},
									None => if depth + depth2 > func.2 {
										highest = Some((start, Some(def), depth + depth2));
									}
								},
								None => highest = Some((start, Some(def), depth + depth2))
							}
						} else {
							let mut j = 1;
							while j < name.len() {
								if let Some(def) = is_defined(functions, &name[..name.len() - j]) {
									match highest {
										Some(func) => match func.1 {
											Some(def2) => if (def.precedence > def2.precedence && depth + depth2 == func.2) || depth + depth2 > func.2 {
												highest = Some((start, Some(def), depth + depth2));
											},
											None => if depth + depth2 > func.2 {
												highest = Some((start, Some(def), depth + depth2));
											}
										},
										None => highest = Some((start, Some(def), depth + depth2))
									}
									
									break;
								}
								
								j += 1;
							}
							
							if j >= name.len() {
								panic!("{}:{} Undefined operator '{}'", tokens[*i].pos.line, tokens[*i].pos.col, &name);
							}
						}
					},
					
					Kind::GroupOp(ref op) if op == "(" => depth += 1,
					Kind::GroupOp(ref op) if op == ")" => if depth > 0 {
						depth -= 1;
					} else {
						panic!("{}:{} Excess ending parenthesis", tokens[*i].pos.line, tokens[*i].pos.col);
					},
					
					_ => ()
				}
			} else if let Kind::Op(ref op) = tokens[*i].kind {
				let mut name = op.to_string();
				
				*i += 1;
				while *i < tokens.len() {
					match tokens[*i].kind {
						Kind::Op(ref op) => {
							name += op;
							if let Some(_) = is_defined(functions, &name) {
								*i += 1;
							} else {
								break;
							}
						},
						_ => break
					}
				}
				*i -= 1;
			} else if let Kind::GroupOp(ref op) = tokens[*i].kind {
				if op == "{" {
					depth2 += 1;
				}
			}
			
			*i += 1;
		}
		
		match highest {
			Some(func) => {
				lowest = Some(func.0);
				
				match func.1 {
					Some(def) => parse_func(tokens, (func.0, def), functions),
					None => parse_group(tokens, func.0, functions)
				}
			},
			None => break
		}
	}
	
	lowest
}

fn parse_if(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize) {
	let mut body = tokens[*i].children.borrow_mut();
	*i += 1;
	
	let next = *i;
	if let Some(token) = parse_statement(tokens, functions, i) {
		body.push(token);
	} else {
		body.push(next);
	}
	
	body.push(*i);
	
	parse2(tokens, functions, i);
	*i += 1;
	
	match tokens[*i].kind {
		Kind::Reserved(ref keyword) if keyword == "else" => {
			*i += 1;
			body.push(*i);
			
			match tokens[*i].kind {
				Kind::Reserved(ref keyword) if keyword == "if" => parse_if(tokens, functions, i),
				_ => parse2(tokens, functions, i)
			}
		},
		
		_ => ()
	}
}

fn parse_ret(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize) {
	let mut body = tokens[*i].children.borrow_mut();
	*i += 1;
	
	let next = *i;
	if let Some(token) = parse_statement(tokens, functions, i) {
		body.push(token);
	} else {
		body.push(next);
	}
}

fn parse_let(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize) {
	let mut body = tokens[*i].children.borrow_mut();
	*i += 1;
	
	let start = *i;
	while *i < tokens.len() {
		match tokens[*i].kind {
			Kind::Op(_) => break,
			_ => *i += 1
		}
	}
	
	if *i >= tokens.len() {
		panic!("Unexpected EOF");
	}
	
	body.push(*i);
	
	*i = start;
	parse_statement(tokens, functions, i);
}

fn parse_type_decl<'a>(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, mut upper_body: RefMut<'a, Vec<usize>>) -> RefMut<'a, Vec<usize>> {
	let mut body = tokens[*i].children.borrow_mut();
	*i += 1;
	
	let start = *i;
	while *i < tokens.len() {
		match tokens[*i].kind {
			Kind::Op(ref op) => if op == "=" {
				upper_body.push(start - 1);
				break;
			} else {
				*i = start - 1;
				return upper_body;
			},
			_ => *i += 1
		}
	}
	
	if *i >= tokens.len() {
		panic!("Unexpected EOF");
	}
	
	body.push(*i);
	
	*i = start;
	parse_statement(tokens, functions, i);
	
	upper_body
}

pub fn parse2(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize) {
	match tokens[*i].kind {
		Kind::GroupOp(ref op) if op == "{" => {
			let mut body = tokens[*i].children.borrow_mut();
			let mut nests = 0;
			*i += 1;
			
			while *i < tokens.len() {
				let start = *i;
				match tokens[*i].kind {
					Kind::GroupOp(ref op) if op == "{" => nests += 1,
					
					Kind::GroupOp(ref op) if op == "}" => if nests > 0 {
						nests -= 1;
					} else {
						break;
					},
					
					_ => match tokens[*i].kind {
						Kind::Reserved(ref keyword) if keyword == "if" => {
							body.push(*i);
							parse_if(tokens, functions, i);
						},
						
						Kind::Reserved(ref keyword) if keyword == "return" => {
							body.push(*i);
							parse_ret(tokens, functions, i);
						},
						
						Kind::Reserved(ref keyword) if keyword == "let" => {
							body.push(*i);
							parse_let(tokens, functions, i);
						},
						
						Kind::Type(_) => body = parse_type_decl(tokens, functions, i, body),
						
						Kind::Op(ref op) if op == ";" => *i += 1,
						
						_ => if let Some(token) = parse_statement(tokens, functions, i) {
							body.push(token);
						} else {
							body.push(start); // Should this really be pushing start instead of *i?
//							*i += 1;
						}
					}
				}
			}
		},
		
		_ => ()
	}
}

fn compile_func(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, mut output: String) -> String {
	match tokens[*i].kind {
		Kind::GroupOp(ref op) if op == "{" => {
			let statements = tokens[*i].children.borrow();
			
			output += "{";
			
			for statement in statements.iter() {
				*i = *statement;
				output = compile_func(tokens, functions, i, output);
				
				let mut nests = 0;
				while *i + 1 < tokens.len() {
					match tokens[*i + 1].kind {
						Kind::Op(ref op) if op == ";" => {
							output += ";";
							break;
						},
						
						Kind::GroupOp(ref op) if op == "{" => nests += 1,
						
						Kind::GroupOp(ref op) if op == "}" => if nests > 0 {
							nests -= 1;
						} else {
							break;
						},
						
						_ => *i += 1
					}
				}
			}
			
			output += "}";
		},
		
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
		
		Kind::Op(ref op) => {
			let mut name = op.to_string();
			let start = *i;
			
			if name == "-" {
				match tokens[*i + 1].kind {
					Kind::Op(ref op) if op == ">" => {
						*i += 1;
						return output;
					},
					_ => ()
				}
			}
			
			*i += 1;
			while *i < tokens.len() {
				match tokens[*i].kind {
					Kind::Op(ref op) => {
						name += op;
						if let Some(_) = is_defined(functions, &name) { // NEEDS FIXING FOR RETURN ARROWS [EDIT: Has this been fixed yet?]
							*i += 1;
						} else {
							if name.ends_with("->") {
								if let Kind::Type(_) = tokens[*i + 1].kind {
									name.pop();
									*i -= 1;
								}
							}
							
							name.pop();
							break;
						}
					},
					
					_ => break
				}
			}
			*i -= 1;
			
			let args = tokens[start].children.borrow();
			
			match name.as_ref() {
				"=" | "+=" | "-=" | "*=" | "%=" | "^=" | "<<=" | ">>=" => {
					*i = args[0];
					output = compile_func(tokens, functions, i, output);
					
					output += &name;
					
					*i = args[1];
					output = compile_func(tokens, functions, i, output);
				},
				
				"+" | "-" | "*" | "/" | "%" | "==" | "!=" | "&&" | "|" | "||" | "^" | "<" | ">" | "<<" | ">>" => {
					*i = args[0];
					output += "(";
					output = compile_func(tokens, functions, i, output);
					output += ")";
					
					output += &name;
					
					*i = args[1];
					output += "(";
					output = compile_func(tokens, functions, i, output);
					output += ")";
				},
				
				_ => {
					let mut new_name = String::new();
					for op in name.chars() {
						new_name += match op {
							'+' => "plus",
							'-' => "minus",
							'*' => "times",
							'/' => "div",
							'%' => "mod",
							'=' => "eq",
							'&' => "and",
							'|' => "or",
							'^' => "xor",
							'<' => "larrow",
							'>' => "rarrow",
							'!' => "not",
							'~' => "binnot",
							'?' => "quest",
							':' => "colon",
							'.' => "dot",
							',' => "comma",
							'@' => "at",
							';' => "semic", // Should this really be allowed? Overriding the functionality of the semicolon may cause major issues
							_ => unreachable!()
						};
					}
					
					output += &new_name;
					output += "(";
					
					if args.len() >= 1 && args[0] != usize::MAX { // In reality it would probably be better to use Option instead of usize::MAX for this but I was too lazy xD
						for (a, arg) in args.iter().enumerate() {
							*i = *arg;
							output = compile_func(tokens, functions, i, output);
							
							if a < args.len() - 1 {
								output += ","
							}
						}
					}
					
					output += ")";
				}
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
				
				let mut nests = 0;
				while *i + 1 < tokens.len() {
					match tokens[*i + 1].kind {
						Kind::Op(ref op) if op == ";" => {
							output += ";";
							break;
						},
						
						Kind::GroupOp(ref op) if op == "{" => nests += 1,
						
						Kind::GroupOp(ref op) if op == "}" => if nests > 0 {
							nests -= 1;
						} else {
							break;
						},
						
						_ => *i += 1
					}
				}
			}
			
			output += "}";
			
			if children.len() > 2 {
				output += "else ";
				*i = children[2];
				output = compile_func(tokens, functions, i, output);
			}
		},
		
		Kind::Reserved(ref keyword) if keyword == "return" => {
			output += "return ";
			
			*i = tokens[*i].children.borrow()[0];
			output = compile_func(tokens, functions, i, output);
		},
		
		Kind::Reserved(ref keyword) if keyword == "let" => {
			output += "let ";
			match tokens[*i + 1].kind {
				Kind::Type(ref typ) if typ == &Type::Const => (),
				_ => output += "mut "
			}
			
			*i = tokens[*i].children.borrow()[0];
			output = compile_func(tokens, functions, i, output);
		},
		
		Kind::Str1(ref s) => {
			output += "\"";
			output += s;
			output += "\"";
		},
		
		Kind::Str2(ref s) => {
			if s.len() == 1 || (s.len() == 2 && s.chars().next().unwrap() == '\\') { // Just a character, not an actual string
				output += "'";
				output += s;
				output += "'";
			} else {
				panic!("{}:{} P+ style strings are not supported yet", tokens[*i].pos.line, tokens[*i].pos.col);
			}
		},
		
		Kind::Type(ref typ) => {
			use lib::Type::*;
			
			if tokens[*i].children.borrow().len() > 0 {
				output += "let ";
				if typ != &Type::Const {
					output += "mut ";
				}
				
				*i = tokens[*i].children.borrow()[0];
				output = compile_func(tokens, functions, i, output);
				return output;
			}
			
			let mut types = vec![vec![typ]];
			let mut t = 0;
			*i += 1;
			while *i < tokens.len() {
				match tokens[*i].kind {
					Kind::Type(ref typ) => types[t].push(typ),
					Kind::Op(ref op) if op == "|" => {
						types.push(Vec::new());
						t += 1;
					},
					_ => break
				}
				
				*i += 1;
			}
			*i -= 1;
			
			if *i + 1 >= tokens.len() {
				panic!("Unexpected EOF");
			}
			
			let mut unsigned = false;
			for section in types { // WIP; TODO: handle this correctly with enums, unions or something
				for typ in section {
					match *typ {
						Array => (), // WIP
						Bool => output += "bool",
						Chan => (), // WIP
						Char => output += "char",
						Const => (), // Should this be ignored?
						Fraction => (), // WIP
						Func => output += "fn",
						Heap => (), // WIP
						Int => if unsigned {
							output += "usize";
						} else {
							output += "isize";
						},
						List => (), // WIP
						Macro => (), // WIP
						Only => (), // WIP
						Pointer => output += "&", // NOTE: Needs changing (for example pointer*2)
						Register => (), // WIP
						Stack => (), // WIP
						Unique => (), // WIP
						Unsigned => unsigned = true,
						Void => output += "()",
						Volatile => (), // WIP
					}
				}
			}
		},
		
		Kind::Var(ref name, ref typ) if typ[0].len() == 0 ||
										typ[0][0] == Type::Func ||
										typ[0][0] == Type::Const => {
			if let Some(_) = is_defined(functions, name) { // TMP until I've worked out passing functions as arguments
				output += if name == "init" {
					"main"
				} else if name == "println" {
					"println!"
				} else {
					name
				};
				output += "(";
				
				if name == "println" {
					output += "\"{}\",";
				}
				
				let args = tokens[*i].children.borrow();
				if args.len() >= 1 && args[0] != usize::MAX {
					for (a, arg) in args.iter().enumerate() {
						*i = *arg;
						output = compile_func(tokens, functions, i, output);
						
						if a < args.len() - 1 {
							output += ","
						}
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
			
			for t in &typ[0] { // TMP until I've worked out how to handle multiple types
				match t {
					Array => (), // WIP
					Bool => output += "bool",
					Chan => (), // WIP
					Char => output += "char",
					Const => (),
					Fraction => (), // WIP
					Func => output += "fn",
					Heap => (), // WIP
					Int => if unsigned {
						output += "usize";
					} else {
						output += "isize";
					},
					List => (), // WIP
					Macro => (), // WIP
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
		
		_ => () // WIP
	}
	
	output
}

pub fn compile(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, mut output: String) -> String {
	use lib::Type::*;
	use lib::Kind::*;
	
	let children = tokens[*i].children.borrow();
	
	match tokens[*i].kind {
		Kind::Reserved(ref keyword) if keyword == "import" => {
			// Using Rust-style importing for now
			output += "use ";
			*i += 1;
			
			let mut success = false;
			while *i < tokens.len() {
				match tokens[*i].kind {
					Kind::Reserved(ref keyword) if keyword == "as" => {
						output += " as ";
						*i += 1;
					},
					
					Kind::Op(ref op) if op == ";" => {
						output += ";";
						success = true;
						break;
					},
					
					_ => {
						output += &get_val!(tokens[*i].kind); // Will probably be changed
						*i += 1;
					}
				}
			}
			
			if !success {
				panic!("Unexpected EOF");
			}
		},
		
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
				for statement in statements.iter() {
					*i = *statement;
					output = compile_func(tokens, functions, i, output);
					
					let mut nests = 0;
					while *i + 1 < tokens.len() {
						match tokens[*i + 1].kind {
							Kind::Op(ref op) if op == ";" => {
								output += ";";
								break;
							},
							
							Kind::GroupOp(ref op) if op == "{" => nests += 1,
							
							Kind::GroupOp(ref op) if op == "}" => if nests > 0 {
								nests -= 1;
							} else {
								break;
							},
							
							_ => *i += 1
						}
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
						output += &get_val!(tokens[*i].kind); // Will probably be changed
						*i += 1;
					}
				}
			}
		},
		
		_ => ()
	}
	
	output
	
	// OUTDATED CODE BELOW
	
/*	match tokens[*i].kind {
		Kind::Op(ref op) => match op.as_ref() {
			"@" => output += "*",
			"-" if get_val!(tokens[*i + 1].kind) == ">" => if *func_def {
				output += "-> ";
				
				*func_def = false;
//				*i += 2;
//				*i += nxt(&tokens, *i);
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
		}
	}
	
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
	}; */
}