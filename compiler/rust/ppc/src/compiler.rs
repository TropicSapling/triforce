use std::{
	fs,
	fs::File,
	io,
	io::prelude::*,
	io::Error,
	io::ErrorKind,
	process::Command,
	str,
	usize,
	cell::RefMut,
	cell::RefCell
};

use lib::{Token, Kind, Type, FilePos, Function, FunctionArg, MacroFunction};

macro_rules! get_val {
	($e:expr) => ({
		use lib::Kind::*;
		use lib::Type::*;
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
	() => (vec![
		// WIP; 'typ' structure needs support for multiple types ('int|fraction' for these operators)
		def_builtin_op!(String::from("a"), String::from("b"), "+", Type::Int, Type::Int, Type::Int, 245),
		def_builtin_op!(String::from("a"), String::from("b"), "-", Type::Int, Type::Int, Type::Int, 245),
		def_builtin_op!(String::from("a"), String::from("b"), "*", Type::Int, Type::Int, Type::Int, 246),
		def_builtin_op!(String::from("a"), String::from("b"), "/", Type::Int, Type::Int, Type::Int, 246),
		def_builtin_op!(String::from("a"), String::from("b"), "%", Type::Int, Type::Int, Type::Int, 246),
		
		// WIP; 'typ' structure needs support for multiple types (all types for these operators)
		def_builtin_op!(String::from("a"), String::from("b"), "==", Type::Int, Type::Int, Type::Bool, 242),
		def_builtin_op!(String::from("a"), String::from("b"), "!=", Type::Int, Type::Int, Type::Bool, 242),
		def_builtin_op!(String::from("a"), String::from("b"), "<", Type::Int, Type::Int, Type::Bool, 243),
		def_builtin_op!(String::from("a"), String::from("b"), "<=", Type::Int, Type::Int, Type::Bool, 243),
		def_builtin_op!(String::from("a"), String::from("b"), ">", Type::Int, Type::Int, Type::Bool, 243),
		def_builtin_op!(String::from("a"), String::from("b"), ">=", Type::Int, Type::Int, Type::Bool, 243),
		
		def_builtin_op!(String::from("a"), String::from("b"), "&&", Type::Bool, Type::Bool, Type::Bool, 238),
		def_builtin_op!(String::from("a"), String::from("b"), "||", Type::Bool, Type::Bool, Type::Bool, 237),
		
		def_builtin_op!(String::from("a"), String::from("b"), "<<", Type::Int, Type::Int, Type::Int, 244),
		def_builtin_op!(String::from("a"), String::from("b"), ">>", Type::Int, Type::Int, Type::Int, 244),
		def_builtin_op!(String::from("a"), String::from("b"), "^", Type::Int, Type::Int, Type::Int, 240),
		
		// WIP; 'macro' types are not yet implemented
		def_builtin_op!(String::from("a"), String::from("b"), "=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!(String::from("a"), String::from("b"), "+=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!(String::from("a"), String::from("b"), "-=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!(String::from("a"), String::from("b"), "*=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!(String::from("a"), String::from("b"), "/=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!(String::from("a"), String::from("b"), "%=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!(String::from("a"), String::from("b"), ">>=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!(String::from("a"), String::from("b"), "<<=", Type::Int, Type::Int, Type::Void, 0),
		def_builtin_op!(String::from("a"), String::from("b"), "^=", Type::Int, Type::Int, Type::Void, 0),
		
		Function {
			name: String::from("println"),
			pos: 0,
			args: vec![
				FunctionArg {
					name: String::from("a"),
					typ: vec![vec![Type::Int]] // WIP; No support for strings yet
				}
			],
			precedence: 1,
			output: vec![]
		},
		
		Function {
			name: String::from("print"),
			pos: 0,
			args: vec![
				FunctionArg {
					name: String::from("a"),
					typ: vec![vec![Type::Int]] // WIP; No support for strings yet
				}
			],
			precedence: 1,
			output: vec![]
		}
	])
}

fn is_defined<'a>(defs: &'a Vec<Function>, call: &str) -> Option<&'a Function> {
	for def in defs {
		if def.name == call {
			return Some(&def);
		}
	}
	
	None
}

pub fn def_functions() -> Vec<Function> {
	def_builtin_funcs!()
}

pub fn parse<'a>(tokens: &'a Vec<Token>, mut functions: Vec<Function>) -> Vec<Function> {
	let mut func = false;
	let mut func_pos = 0;
	let mut func_args = Vec::new();
	let mut par_type = vec![vec![]];
	
	// DEFINE FUNCTIONS (this is done in a separate loop to allow function definitions to be placed both before and after function calls)
	let mut i = 0;
	while i < tokens.len() {
		let mut last_item = functions.len();
		if last_item != 0 {
			last_item -= 1;
		}
		
		match tokens[i].kind {
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
				functions[last_item].args.push(FunctionArg {name: name.clone(), typ: typ.clone()});
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

fn get_parse_limit(tokens: &Vec<Token>, i: &mut usize) -> usize {
	let mut depth = 0;
	let mut dived = false;
	let mut limit = tokens.len();
	while *i < limit {
		match tokens[*i].kind {
			Kind::Op(ref op) if op == ";" && *i > 0 => if depth == 0 {
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

pub fn parse_statement(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize) -> Option<usize> {
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
					Kind::Var(ref name, _) if depth2 == 0 => if let Some(def) = is_defined(functions, name) {
						match highest {
							Some(func) => match func.1 {
								Some(def2) => if (def.precedence > def2.precedence && depth == func.2) || depth > func.2 {
									highest = Some((*i, Some(def), depth));
								},
								None => if depth >= func.2 {
									highest = Some((*i, Some(def), depth));
								}
							},
							None => highest = Some((*i, Some(def), depth))
						}
					},
					
					Kind::GroupOp(ref op) if op == "{" => {
						if depth2 == 0 {
							match highest {
								Some(func) => if depth >= func.2 {
									highest = Some((*i, None, depth));
								},
								None => highest = Some((*i, None, depth))
							}
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
						
						if depth2 == 0 {
							if let Some(def) = is_defined(functions, &name) {
								match highest {
									Some(func) => match func.1 {
										Some(def2) => if (def.precedence > def2.precedence && depth == func.2) || depth > func.2 {
											highest = Some((start, Some(def), depth));
										},
										None => if depth > func.2 {
											highest = Some((start, Some(def), depth));
										}
									},
									None => highest = Some((start, Some(def), depth))
								}
							} else {
								let mut j = 1;
								while j < name.len() {
									if let Some(def) = is_defined(functions, &name[..name.len() - j]) {
										match highest {
											Some(func) => match func.1 {
												Some(def2) => if (def.precedence > def2.precedence && depth == func.2) || depth > func.2 {
													highest = Some((start, Some(def), depth));
												},
												None => if depth > func.2 {
													highest = Some((start, Some(def), depth));
												}
											},
											None => highest = Some((start, Some(def), depth))
										}
										
										break;
									}
									
									j += 1;
								}
								
								if j >= name.len() {
									panic!("{}:{} Undefined operator '{}'", tokens[*i].pos.line, tokens[*i].pos.col, &name);
								}
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
					None => parse2(tokens, functions, &mut func.0.clone())
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
				
				if let Kind::GroupOp(ref op) = tokens[*i].kind {
					if op == "{" {
						nests += 1;
						
						body.push(*i);
						parse2(tokens, functions, i);
						
						*i += 1;
						continue;
					}
				}
				
				match tokens[*i].kind {
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
						}
					}
				}
			}
		},
		
		_ => ()
	}
}

fn correct_indexes_after_del(tokens: &Vec<Token>, i: usize) {
	for token in tokens {
		let mut children = token.children.borrow_mut();
		let mut c = 0;
		while c < children.len() {
			if children[c] > i && children[c] != usize::MAX {
				children[c] -= 1;
			}
			
			c += 1;
		}
	}
}

fn correct_indexes_after_add(tokens: &Vec<Token>, i: usize, exceptions: &Vec<usize>) {
	for (t, token) in tokens.iter().enumerate() {
		if exceptions.contains(&t) {
			continue;
		}
		
		let mut children = token.children.borrow_mut();
		let mut c = 0;
		while c < children.len() {
			if children[c] > i && children[c] != usize::MAX {
				children[c] += 1;
			}
			
			c += 1;
		}
	}
}

fn del_all_children(tokens: &mut Vec<Token>, children: &Vec<usize>) -> Vec<usize> {
	let mut trash = Vec::new();
	for child in children.iter() {
		if *child != usize::MAX {
			let children = tokens[*child].children.borrow().clone();
			for i in del_all_children(tokens, &children) {
				trash.push(i);
			}
			
			trash.push(*child);
			
			if let Kind::GroupOp(ref op) = tokens[*child].kind {
				if op == "{" {
					let mut depth = 0;
					let mut i = *child + 1;
					while i < tokens.len() {
						match tokens[i].kind {
							Kind::GroupOp(ref op) if op == "{" => depth += 1,
							Kind::GroupOp(ref op) if op == "}" => if depth > 0 {
								depth -= 1;
							} else {
								break;
							},
							
							_ => ()
						}
						
						i += 1;
					}
					
					if i < tokens.len() {
						trash.push(i);
					}
				}
			}
		}
	}
	
	trash
}

fn add_to_code(tokens: &Vec<Token>, functions: &Vec<Function>, code: &mut Vec<Token>, parent: usize) {
	match tokens[parent].kind {
		Kind::Var(ref name, _) => if let Some(def) = is_defined(functions, &name) {
			let new_parent = tokens[parent].clone();
			let mut children = tokens[parent].children.borrow_mut();
			
			let mut i = 0;
			while i < def.pos {
				add_to_code(tokens, functions, code, children[i]);
				
				i += 1;
			}
			
			new_parent.children.borrow_mut().clear();
			code.push(new_parent);
			
			i += 1;
			while i < tokens.len() && i < def.args.len() + 1 {
				add_to_code(tokens, functions, code, children[i - 1]);
				
				i += 1;
			}
		},
		
		Kind::Op(ref op) => {
			let mut name = op.to_string();
			let mut i = parent + 1;
			
			while i < tokens.len() {
				match tokens[i].kind {
					Kind::Op(ref op) => {
						name += op;
						if let Some(_) = is_defined(functions, &name) {
							i += 1;
						} else {
							name.pop();
							break;
						}
					},
					
					_ => break
				}
			}
			
			if let Some(def) = is_defined(functions, &name) {
				let new_parent = tokens[parent].clone();
				let mut children = tokens[parent].children.borrow_mut();
				
				let mut i = 0;
				while i < def.pos {
					add_to_code(tokens, functions, code, children[i]);
					
					i += 1;
				}
				
				new_parent.children.borrow_mut().clear();
				code.push(new_parent);
				
				i += 1;
				while i < tokens.len() && i < def.args.len() + 1 {
					add_to_code(tokens, functions, code, children[i - 1]);
					
					i += 1;
				}
			} else {
				panic!("{}:{} Undefined operator '{}'", tokens[parent].pos.line, tokens[parent].pos.col, get_val!(tokens[parent].kind));
			}
		},
		
		Kind::GroupOp(ref op) if op == "{" => {
			let new_parent = tokens[parent].clone();
			let mut children = tokens[parent].children.borrow_mut();
			
			new_parent.children.borrow_mut().clear();
			code.push(new_parent);
			
			let mut i = 0;
			while i < tokens.len() && i < children.len() {
				add_to_code(tokens, functions, code, children[i]);
				
				i += 1;
			}
			
			code.push(Token {
				kind: Kind::GroupOp(String::from("}")),
				pos: FilePos {line: 0, col: 0},
				children: RefCell::new(Vec::new())
			});
		},
		
		_ => code.push(tokens[parent].clone())
	}
}

pub fn parse3(tokens: &mut Vec<Token>, macro_funcs: &mut Vec<MacroFunction>, functions: &mut Vec<Function>, i: &mut usize) -> Result<(), Error> {
	match tokens[*i].kind.clone() {
		Kind::Var(ref name, _) => {
			let mut j = 0;
			while j < macro_funcs.len() {
				if name == &macro_funcs[j].func.name {
					// Run macro function
					
					let args = tokens[*i].children.borrow().clone();
					let mut new_code = Vec::new();
					let mut new_points: Vec<Vec<Token>> = Vec::new();
					if args.len() >= 1 && args[0] != usize::MAX {
						for (a, arg) in args.iter().enumerate() {
							for token in macro_funcs[j].code.iter() {
								match token.kind {
									Kind::Var(ref name, _) if name == &macro_funcs[j].func.args[a].name => add_to_code(tokens, functions, &mut new_code, *arg),
									_ => new_code.push(token.clone())
								}
							}
							
							for (p, point) in macro_funcs[j].returns.iter().enumerate() {
								new_points.push(Vec::new());
								for token in point.iter() {
									match token.kind {
										Kind::Var(ref name, _) if name == &macro_funcs[j].func.args[a].name => add_to_code(tokens, functions, &mut new_points[p], *arg),
										_ => new_points[p].push(token.clone())
									}
								}
							}
						}
					} else {
						new_code = macro_funcs[j].code.clone();
					}
					
					// Remove macro call since it will be replaced later
					let mut trash = del_all_children(tokens, &args);
					let mut t = 0;
					while t < trash.len() {
						tokens.remove(trash[t]);
						correct_indexes_after_del(tokens, trash[t]);
						
						let mut i = t + 1;
						while i < trash.len() {
							if trash[i] > trash[t] && trash[i] != usize::MAX {
								trash[i] -= 1;
							}
							
							i += 1;
						}
						
						t += 1;
					}
					
					tokens.remove(*i);
					correct_indexes_after_del(tokens, *i);
					
					// Parse macro function
					*functions = parse(&new_code, functions.clone()); // Ik, it's not good to clone for performance but I was just too lazy to fix the issues...
					parse2(&mut new_code, &functions, &mut 2);
					
					let mut lowest = [1, 1];
					for (p, point) in new_points.iter().enumerate() {
						if let Some(token) = parse_statement(point, &functions, &mut 0) {
							lowest[p] = token;
						}
					}
					
					let mut out_contents = String::new();
					let mut k = 0;
					while k < new_code.len() {
						out_contents = compile(&new_code, &functions, &mut k, out_contents);
						k += 1;
					}
					
					out_contents.insert_str(9, "->Result<(),usize>");
					let mut k = 0;
					while k + 6 < out_contents.len() {
						if &out_contents[k..k + 6] == "return" {
							k += 7;
							out_contents.insert_str(k, "Err(");
							k += 5;
							out_contents.insert(k, ')');
						}
						
						k += 1;
					}
					
					if args.len() == 0 || args[0] == usize::MAX {
						let pos = out_contents.len() - 1;
						out_contents.insert_str(pos, "Ok(())");
					}
					
					//////// CREATE RUST OUTPUT ////////
					
					fs::create_dir_all("macros")?;
					
					let mut out_file = File::create("macros\\macro.rs")?;
					out_file.write_all(out_contents.as_bytes())?;
					
					Command::new("rustfmt").arg("macros\\macro.rs").output().expect("failed to format Rust code");
					
					//////// CREATE BINARY OUTPUT ////////
					
					let mut error = false;
					
					let out = Command::new("rustc")
							.args(&["--color", "always", "--out-dir", "macros", "macros\\macro.rs"])
							.output()
							.expect("failed to compile Rust code");
					
					if out.stdout.len() > 0 {
						println!("{}", str::from_utf8(&out.stdout).unwrap());
					}
					
					if out.stderr.len() > 0 {
						println!("{}", str::from_utf8(&out.stderr).unwrap());
						error = true;
					}
					
					//////// RUN COMPILED BINARY ////////
					
					if !error {
						let out = if cfg!(target_os = "windows") {
							Command::new("macros\\macro.exe")
								.output()
								.expect("failed to execute process")
						} else {
							Command::new("./macros/macro.exe")
								.output()
								.expect("failed to execute process")
						};
						
						if out.stdout.len() > 0 {
							println!("{}", str::from_utf8(&out.stdout).unwrap());
							io::stdout().flush()?;
						}
						
						if out.stderr.len() > 0 {
							if out.stderr.starts_with(b"Error: ") {
								let point = str::from_utf8(&out.stderr).unwrap()[7..out.stderr.len() - 1].parse::<usize>();
								
								if let Ok(point) = point {
									let mut exceptions = Vec::new();
									'outer: for (t, tok) in tokens.iter_mut().enumerate() {
										let mut children = tok.children.borrow_mut();
										for child in children.iter_mut() {
											if *child == *i {
												*child = *i + lowest[point] - 1; // -1 because 'point' starts with semicolon that is ignored later
												exceptions.push(t);
												break 'outer;
											}
										}
									}
									
									let length = &new_points[point].len();
									for (t, token) in new_points[point][1..length - 1].iter().enumerate() {
										tokens.insert(*i, token.clone());
										
										for e in exceptions.iter_mut() {
											if *e > *i {
												*e += 1;
											}
										}
										
										correct_indexes_after_add(tokens, *i, &exceptions);
										
										let mut children = tokens[*i].children.borrow_mut();
										for child in children.iter_mut() {
											*child = *i + *child - t - 1;
										}
										
										exceptions.push(*i);
										
										*i += 1;
									}
								}
							} else {
								println!("{}", str::from_utf8(&out.stderr).unwrap());
							}
						} else {
							tokens.insert(*i, Token {
								kind: Kind::GroupOp(String::from("(")),
								pos: FilePos {line: 0, col: 0},
								children: RefCell::new(Vec::new())
							});
							
							correct_indexes_after_add(tokens, *i, &Vec::new());
							*i += 1;
							
							tokens.insert(*i, Token {
								kind: Kind::GroupOp(String::from(")")),
								pos: FilePos {line: 0, col: 0},
								children: RefCell::new(Vec::new())
							});
							
							correct_indexes_after_add(tokens, *i, &Vec::new());
						}
					}
					
					//////// DELETE CREATED FILES ////////
					
					fs::remove_file("macros\\macro.rs")?;
					
					if !error {
						fs::remove_file("macros\\macro.exe")?;
						fs::remove_file("macros\\macro.pdb")?;
					} else {
						return Err(Error::new(ErrorKind::Other, "compilation of macro failed"));
					}
					
//					fs::remove_dir("macros")?; // Doesn't work (on Windows) for some reason?
					
					break;
				}
				
				j += 1;
			}
		},
		
		_ => ()
	}
	
	Ok(())
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
				
				"+" | "-" | "*" | "/" | "%" | "==" | "<=" | ">=" | "!=" | "&&" | "|" | "||" | "^" | "<" | ">" | "<<" | ">>" => {
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
				} else if name == "print" {
					"print!"
				} else {
					name
				};
				output += "(";
				
				if name == "println" || name == "print" {
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