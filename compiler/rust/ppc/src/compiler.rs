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
	cell::RefCell
};

use lib::{Token, Kind, Type, FilePos, Function, FunctionSection, Macro};

macro_rules! get_val {
	($e:expr) => ({
		use lib::Kind::*;
		use lib::Type::*;
		match $e {
			Func(_,_) => String::from("func"),
			GroupOp(ref val, _, _) => val.to_string(),
			Literal(b) => if b {
				String::from("true")
			} else {
				String::from("false")
			},
			Number(int, fraction) => {
				int.to_string() + "." + &fraction.to_string()
			},
			Op(ref val, _, _) => val.to_string(),
			Reserved(ref val, _) => val.to_string(),
			Str1(ref val) => "\"".to_string() + val + "\"",
			Str2(ref val) => "'".to_string() + val + "'",
			Type(ref typ, _) => match typ {
				&Array => String::from("array"),
				&Chan => String::from("chan"),
				&Const => String::from("const"),
				&Fraction => String::from("fraction"),
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
			Var(ref name, _, _, _) => name.to_string()
		}
	});
}

macro_rules! def_builtin_op {
	($a:expr, $b:expr, $name:expr, $typ1:expr, $typ2:expr, $output:expr, $precedence:expr) => (Function {
		structure: vec![
			FunctionSection::Arg($a, vec![vec![$typ1]]),
			FunctionSection::OpID(String::from($name)),
			FunctionSection::Arg($b, vec![vec![$typ2]])
		],
		
		output: if $output == Type::Void {
			vec![vec![]]
		} else {
			vec![vec![$output]]
		},
		
		precedence: $precedence // NOTE: 0 is *lowest* precedence, not highest. Highest precedence is 255.
	})
}

const BUILTIN_FUNCS: usize = 28;

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
		
		// WIP; 'macro' types are not yet implemented [EDIT: aren't they now?]
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
			structure: vec![
				FunctionSection::ID(String::from("let")),
				FunctionSection::Arg(String::from("a"), vec![vec![Type::Int]]), // WIP; No support for any types yet
				FunctionSection::OpID(String::from("=")),
				FunctionSection::Arg(String::from("b"), vec![vec![Type::Int]]), // WIP; No support for any types yet
			],
			
			output: vec![],
			
			precedence: 0
		},
		
		Function {
			structure: vec![
				FunctionSection::ID(String::from("println")),
				FunctionSection::Arg(String::from("a"), vec![vec![Type::Int]]) // WIP; No support for strings yet
			],
			
			output: vec![],
			
			precedence: 1
		},
		
		Function {
			structure: vec![
				FunctionSection::ID(String::from("print")),
				FunctionSection::Arg(String::from("a"), vec![vec![Type::Int]]) // WIP; No support for strings yet
			],
			
			output: vec![],
			
			precedence: 1
		}
	])
}

pub fn def_functions() -> Vec<Function> {
	def_builtin_funcs!()
}

fn has_one_arg(structure: &Vec<FunctionSection>) -> bool {
	let mut found_arg = false;
	for section in structure {
		if let FunctionSection::Arg(_,_) = section {
			if found_arg {
				return false;
			} else {
				found_arg = true;
			}
		}
	}
	
	found_arg
}

pub fn parse<'a>(tokens: &'a mut Vec<Token>, mut functions: Vec<Function>) -> Vec<Function> {
	let mut fpos = Vec::new();
	let mut i = 0;
	while i < tokens.len() {
		match tokens[i].kind {
			Kind::Func(_, ref children) => {
				let mut func_struct = vec![];
				let mut precedence = 1;
				
				fpos.push(i);
				i += 1;
				
				// Parse function structure
				while i < tokens.len() {
					match tokens[i].kind {
						Kind::GroupOp(ref op, _, _) if op == "{" || op == ";" => break, // End of function structure
						
						Kind::Op(ref op, _, _) => {
							let mut name = op.to_string();
							
							i += 1;
							while i < tokens.len() {
								match tokens[i].kind {
									Kind::Op(ref op, _, _) => {
										name += op;
										i += 1;
									},
									
									_ => break
								}
							}
							
							if name == "->" {
								break; // End of function structure
							} else {
								// Function name
								
								if name == "**" {
									precedence = 247;
								}
								
								func_struct.push(FunctionSection::OpID(name));
							}
						},
						
						Kind::Var(ref name, ref typ, _, _) => if typ[0].len() > 0 {
							// Function arg
							func_struct.push(FunctionSection::Arg(name.to_string(), typ.clone()));
						} else {
							// Function name
							func_struct.push(FunctionSection::ID(name.to_string()));
						},
						
						_ => ()
					}
					
					i += 1;
				}
				
				// Get function output
				let output = if let Kind::Type(_, ref typ) = tokens[i].kind {
					if precedence != 247 {
						precedence = if has_one_arg(&func_struct) {
							255
						} else {
							2
						};
					}
					
					typ.clone()
				} else {
					Vec::new()
				};
				
				functions.push(Function {
					structure: func_struct,
					output,
					precedence
				});
				
				while i < tokens.len() {
					match tokens[i].kind {
						Kind::GroupOp(ref op, _, _) if op == "{" || op == ";" => break,
						_ => i += 1
					}
				}
				
				children.borrow_mut().push(i); // Save function body index
			},
			
			_ => i += 1
		}
	}
	
	let mut id = BUILTIN_FUNCS;
	for i in fpos {
		match tokens[i].kind {
			Kind::Func(ref mut f, _) => *f = id,
			_ => unreachable!()
		}
		
		id += 1;
	}
	
	functions
}

fn parse_func(tokens: &mut Vec<Token>, blueprint: &Vec<(&FunctionSection, usize)>, all_children: &mut Vec<usize>) {
	if blueprint.len() > 1 {
		let mut last_s = 0;
		let mut parents = match &tokens[0].kind {
			Kind::GroupOp(_, _, ref arr) | Kind::Op(_, _, ref arr) | Kind::Var(_, _, _, ref arr) => arr,
			_ => unreachable!()
		};
		
		for (s, section) in blueprint.iter().enumerate() {
			match section.0 {
				FunctionSection::ID(_) | FunctionSection::OpID(_) => {
					let parent = &tokens[section.1];
					match parent.kind {
						Kind::Op(_, ref children, ref sidekicks) | Kind::Var(_, _, ref children, ref sidekicks) => {
							if last_s == 0 {
								parents = sidekicks;
							}
							
							let mut i = section.1 - 1;
							let mut c = 0;
							let mut depth = 0;
							while i > 0 && c < s - last_s {
								match tokens[i].kind {
									Kind::GroupOp(ref op, _, _) if op == "}" => depth += 1,
									Kind::GroupOp(ref op, _, _) if op == "{" => {
										depth -= 1;
										if depth == 0 && !all_children.contains(&i) {
											children.borrow_mut().push(i);
											all_children.push(i);
											c += 1;
										}
									},
									
									Kind::GroupOp(_,_,_) => (),
									
									Kind::Op(_,_,_) if depth == 0 => {
										while i > 0 {
											match tokens[i].kind {
												Kind::Op(_,_,_) => i -= 1,
												_ => break
											}
										}
										
										i += 1;
										
										if !all_children.contains(&i) {
											children.borrow_mut().push(i);
											all_children.push(i);
											c += 1;
										}
									},
									
									_ => if depth == 0 && !all_children.contains(&i) {
										children.borrow_mut().push(i);
										all_children.push(i);
										c += 1;
									}
								}
								
								i -= 1;
							}
							
							children.borrow_mut().reverse();
							
							let mut s2 = s + 1;
							while s2 < blueprint.len() {
								match blueprint[s2].0 {
									FunctionSection::ID(_) | FunctionSection::OpID(_) => break,
									_ => s2 += 1
								}
							}
							
							if s2 >= blueprint.len() {
								let mut i = section.1 + 1;
								while i < tokens.len() {
									match tokens[i].kind {
										Kind::Op(_,_,_) => i += 1,
										_ => break
									}
								}
								
								let mut s = s + 1;
								let mut depth = 0;
								while i < tokens.len() && s < blueprint.len() {
									match tokens[i].kind {
										Kind::GroupOp(ref op, _, _) if op == "}" => depth -= 1,
										Kind::GroupOp(ref op, _, _) if op == "{" => {
											if depth == 0 && !all_children.contains(&i) {
												children.borrow_mut().push(i);
												all_children.push(i);
												s += 1;
											}
											
											depth += 1;
										},
										
										Kind::GroupOp(_,_,_) => (),
										
										Kind::Op(_,_,_) if depth == 0 => {
											if !all_children.contains(&i) {
												children.borrow_mut().push(i);
												all_children.push(i);
												s += 1;
											}
											
											while i < tokens.len() {
												match tokens[i].kind {
													Kind::Op(_,_,_) => i += 1,
													_ => break
												}
											}
											
											i -= 1;
										},
										
										_ => if depth == 0 && !all_children.contains(&i) {
											children.borrow_mut().push(i);
											all_children.push(i);
											s += 1;
										}
									}
									
									i += 1;
								}
							}
						},
						
						_ => unreachable!()
					}
					
					if last_s != 0 {
						parents.borrow_mut().push(section.1);
						all_children.push(section.1);
					}
					
					last_s = s + 1;
				},
				
				_ => ()
			}
		}
	} else {
		match &tokens[blueprint[0].1].kind {
			Kind::Op(_, ref children, _) | Kind::Var(_, _, ref children, _) => {
				children.borrow_mut().push(usize::MAX);
			},
			
			_ => unreachable!()
		}
	}
}

fn get_parse_limit(tokens: &Vec<Token>, i: &mut usize) -> usize {
	let mut depth = 0;
	let mut limit = tokens.len();
	let start = *i;
	while *i < limit {
		match tokens[*i].kind {
			Kind::GroupOp(ref op, _, _) if op == ";" => if depth == 0 {
				limit = *i;
				break;
			},
			
			Kind::GroupOp(ref op, _, _) if op == "{" => {
				depth += 1;
			},
			
			Kind::GroupOp(ref op, _, _) if op == "}" => if depth > 0 {
				depth -= 1;
			} else {
				limit = *i;
				break;
			},
			
			_ => ()
		}
		
		*i += 1;
	}
	
	*i = start;
	
	limit
}

fn update_matches<'a>(matches: &mut Vec<(usize, Vec<(&'a FunctionSection, usize)>, usize)>, functions: &'a Vec<Function>, name: String, depth: usize, pos: usize, has_children: bool) {
	for (i, f) in functions.iter().enumerate() {
		for (j, section) in f.structure.iter().enumerate() {
			match section {
				FunctionSection::ID(ref s) | FunctionSection::OpID(ref s) if s == &name && !has_children => {
					for m in matches.iter_mut().filter(|m| m.0 == i) {
						if m.1.len() == j && pos != m.1[m.1.len() - 1].1 {
							if let Some(_) = m.1.iter().find(|s| match s.0 {
								FunctionSection::Arg(_,_) => false,
								_ => true
							}) {
								if m.2 == depth {
									m.1.push((section, pos));
								}
							} else {
								m.1.push((section, pos));
								m.2 = depth;
							}
						}
					}
					
					if j == 0 {
						matches.push((i, vec![(section, pos)], depth));
					}
				},
				
				FunctionSection::Arg(_,_) => {
					for m in matches.iter_mut().filter(|m| m.0 == i) {
						if m.1.len() == j && m.2 <= depth && pos != m.1[m.1.len() - 1].1 {
							m.1.push((section, pos));
						}
					}
					
					if j == 0 {
						matches.push((i, vec![(section, pos)], depth));
					}
				},
				
				_ => ()
			}
		}
	}
}

fn cleanup_matches(matches: &mut Vec<(usize, Vec<(&FunctionSection, usize)>, usize)>, functions: &Vec<Function>) {
	matches.retain(|m| m.1.len() == functions[m.0].structure.len());
	
	let mut i = 0;
	while i < matches.len() {
		let mut found = false;
		for (j, m) in matches.iter().enumerate() {
			if j != i {
				let mut matching = true;
				for section in &matches[i].1 {
					match section.0 {
						FunctionSection::ID(_) | FunctionSection::OpID(_) => if !m.1.contains(&section) {
							matching = false;
							break;
						},
						
						_ => ()
					}
				}
				
				if matching && m.1.len() > matches[i].1.len() {
					found = true;
					break;
				}
			}
		}
		
		if found {
			matches.remove(i);
		} else {
			i += 1;
		}
	}
}

fn cleanup_matches2(matches: &mut Vec<(usize, Vec<(&FunctionSection, usize)>, usize)>, functions: &Vec<Function>, depth: usize) {
	matches.retain(|m| m.2 <= depth || m.1.len() == functions[m.0].structure.len() || match m.1.iter().find(|s| match s.0 {
		FunctionSection::Arg(_,_) => false,
		_ => true
	}) {
		Some(_) => false,
		None => true
	});
}

fn get_highest<'a>(matches: &'a Vec<(usize, Vec<(&'a FunctionSection, usize)>, usize)>, functions: &Vec<Function>) -> Option<&'a (usize, Vec<(&'a FunctionSection, usize)>, usize)> {
	if matches.len() > 0 {
		let mut top = &matches[0];
		for m in matches {
			if m.2 > top.2 || (m.2 == top.2 && functions[m.0].precedence >= functions[top.0].precedence) {
				top = m;
			}
		}
		
		Some(top)
	} else {
		None
	}
}

pub fn parse_statement(tokens: &mut Vec<Token>, functions: &Vec<Function>, all_children: &mut Vec<usize>, i: &mut usize) -> Option<usize> {
	let start = *i;
	let limit = get_parse_limit(tokens, i);
	let mut parsed = false;
	let mut lowest = None;
	
	loop {
		let mut matches = Vec::new();
		let mut depth = 0;
		let mut depth2 = 0;
		*i = start;
		while *i < limit {
			match tokens[*i].kind.clone() {
				Kind::GroupOp(ref op, _, _) if op == "(" && depth2 == 0 => depth += 1,
				Kind::GroupOp(ref op, _, _) if op == ")" && depth2 == 0 => if depth > 0 {
					depth -= 1;
					cleanup_matches2(&mut matches, functions, depth + depth2);
				} else {
					panic!("{}:{} Excess ending parenthesis", tokens[*i].pos.line, tokens[*i].pos.col);
				},
				
				Kind::GroupOp(ref op, _, _) if op == "{" => {
					if !all_children.contains(i) {
						update_matches(&mut matches, functions, String::new(), depth + depth2, *i, true);
					}
					
					if !parsed {
						parse2(tokens, functions, all_children, i);
						parsed = true;
					} else {
						depth2 += 1;
					}
				},
				
				Kind::GroupOp(ref op, _, _) if op == "}" => {
					depth2 -= 1;
				},
				
				Kind::Op(ref op, ref children, ref sidekicks) if depth2 == 0 => {
					let mut name = op.to_string();
					let start = *i;
					
					*i += 1;
					while *i < limit {
						match tokens[*i].kind {
							Kind::Op(ref op, _, _) => name += op,
							_ => break
						}
						
						*i += 1;
					}
					*i -= 1;
					
					if !all_children.contains(&start) {
						update_matches(&mut matches, functions, name, depth + depth2, start, children.borrow().len() > 0 || sidekicks.borrow().len() > 0);
					}
				},
				
				Kind::Var(ref name, _, ref children, ref sidekicks) if depth2 == 0 && !all_children.contains(i) => update_matches(&mut matches, functions, name.to_string(), depth + depth2, *i, children.borrow().len() > 0 || sidekicks.borrow().len() > 0),
				
				_ => if depth2 == 0 && !all_children.contains(i) {
					update_matches(&mut matches, functions, String::new(), depth + depth2, *i, false);
				}
			}
			
			*i += 1;
		}
		
		cleanup_matches(&mut matches, functions);
		
		match get_highest(&matches, functions) {
			Some(m) => {
				for section in &m.1 {
					match section.0 {
						FunctionSection::ID(_) | FunctionSection::OpID(_) => {
							lowest = Some(section.1);
							break;
						},
						
						_ => ()
					}
				}
				
				parse_func(tokens, &m.1, all_children);
				
				// DEBUG BELOW
				match tokens[lowest.unwrap()].kind {
					Kind::Op(_, ref children, ref sidekicks) | Kind::Var(_, _, ref children, ref sidekicks) | Kind::GroupOp(_, ref children, ref sidekicks) => {
						print!("\x1b[0m\x1b[1m\x1b[38;5;11m");
						
						for section in &m.1 {
							match section.0 {
								FunctionSection::ID(ref name) | FunctionSection::OpID(ref name) => print!(" {}", name),
								FunctionSection::Arg(ref arg, _) => print!(" <{}>", arg)
							}
						}
						
						print!(":\x1b[0m (");
						for child in children.borrow().iter() {
							if *child != usize::MAX {
								print!("\x1b[0m\x1b[1m\x1b[38;5;10m{}\x1b[0m[{}]", get_val!(tokens[*child].kind), *child);
								match tokens[*child].kind {
									Kind::Op(_, ref children, _) | Kind::Var(_, _, ref children, _) | Kind::GroupOp(_, ref children, _) if children.borrow().len() > 0 => {
										print!(": (");
										for child in children.borrow().iter() {
											if *child != usize::MAX {
												print!("\x1b[0m\x1b[1m\x1b[38;5;10m{}\x1b[0m[{}], ", get_val!(tokens[*child].kind), *child);
											}
										}
										print!(")");
									},
									
									_ => ()
								}
								print!(", ");
							}
						}
						print!(")");
						
						if sidekicks.borrow().len() > 0 {
							print!(", {{");
							
							for s in sidekicks.borrow().iter() {
								match tokens[*s].kind {
									Kind::Op(ref name, ref children, _) | Kind::Var(ref name, _, ref children, _) | Kind::GroupOp(ref name, ref children, _) => {
										print!("\x1b[0m\x1b[1m\x1b[38;5;10m{}\x1b[0m[{}]: (", name, s);
										for child in children.borrow().iter() {
											if *child != usize::MAX {
												print!("\x1b[0m\x1b[1m\x1b[38;5;10m{}\x1b[0m[{}]", get_val!(tokens[*child].kind), *child);
												match tokens[*child].kind {
													Kind::Op(_, ref children, _) | Kind::Var(_, _, ref children, _) | Kind::GroupOp(_, ref children, _) if children.borrow().len() > 0 => {
														print!(": (");
														for child in children.borrow().iter() {
															if *child != usize::MAX {
																print!("\x1b[0m\x1b[1m\x1b[38;5;10m{}\x1b[0m[{}], ", get_val!(tokens[*child].kind), *child);
															}
														}
														print!(")");
													},
													
													_ => ()
												}
												print!(", ");
											}
										}
										print!("), ");
									},
									
									_ => unreachable!()
								}
							}
							
							println!("}}");
						} else {
							println!("");
						}
					},
					
					_ => unreachable!()
				}
			},
			
			None => break
		}
	}
	
	lowest
}

fn parse_ret(tokens: &mut Vec<Token>, functions: &Vec<Function>, all_children: &mut Vec<usize>, i: &mut usize, children: RefCell<Vec<usize>>) {
	*i += 1;
	
	let next = *i;
	if let Some(token) = parse_statement(tokens, functions, all_children, i) {
		children.borrow_mut().push(token);
	} else {
		children.borrow_mut().push(next);
	}
}

/* fn parse_type_decl<'a>(tokens: &mut Vec<Token>, functions: &Vec<Function>, i: &mut usize, parent: usize) {
	let start = *i + 1;
	
	{
//		let mut body = tokens[*i].children.borrow_mut();
		let mut body = match tokens[*i].kind {
			Kind::Var(_, _, ref mut children) => children,
			_ => unreachable!()
		};
		*i += 1;
		
		while *i < tokens.len() {
			match tokens[*i].kind {
				Kind::Op(ref op) => if op == "=" {
//					tokens[parent].children.borrow_mut().push(start - 1);
					children.push(start - 1);
					break;
				} else {
					*i = start - 1;
					return;
				},
				_ => *i += 1
			}
		}
		
		if *i >= tokens.len() {
			panic!("Unexpected EOF");
		}
		
		body.push(*i);
	}
	
	*i = start;
	parse_statement(tokens, functions, i);
} */

pub fn parse2(tokens: &mut Vec<Token>, functions: &Vec<Function>, all_children: &mut Vec<usize>, i: &mut usize) {
	match tokens[*i].kind.clone() {
		Kind::GroupOp(ref op, _, _) if op == "{" => {
			let parent = *i;
			let mut nests = 0;
			*i += 1;
			
			while *i < tokens.len() {
				let start = *i;
				
				if let Kind::GroupOp(ref op, _, _) = tokens[*i].kind.clone() {
					if op == "{" {
						nests += 1;
						
						if let Kind::GroupOp(_, ref children, _) = tokens[parent].kind {
							children.borrow_mut().push(*i);
						}
						
						parse2(tokens, functions, all_children, i);
						
						*i += 1;
						continue;
					}
				}
				
				match tokens[*i].kind.clone() {
					Kind::GroupOp(ref op, _, _) if op == "}" => if nests > 0 {
						nests -= 1;
					} else {
						break;
					},
					
					_ => match tokens[*i].kind.clone() {
						Kind::Reserved(ref keyword, ref grandchildren) if keyword == "return" => {
							if let Kind::GroupOp(_, ref children, _) = tokens[parent].kind {
								children.borrow_mut().push(*i);
							}
							
							parse_ret(tokens, functions, all_children, i, grandchildren.clone());
						},
						
//						Kind::Type(_) => parse_type_decl(tokens, functions, i, parent),
						
						Kind::GroupOp(ref op, _, _) if op == ";" => *i += 1,
						
						_ => if let Some(token) = parse_statement(tokens, functions, all_children, i) {
							if let Kind::GroupOp(_, ref children, _) = tokens[parent].kind {
								children.borrow_mut().push(token);
							}
						} else {
							if let Kind::GroupOp(_, ref children, _) = tokens[parent].kind {
								children.borrow_mut().push(start); // Should this really be pushing start instead of *i?
							}
						}
					}
				}
			}
		},
		
		_ => ()
	}
}

/* fn correct_indexes_after_del(tokens: &Vec<Token>, i: usize) {
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

fn correct_indexes_after_add(tokens: &Vec<Token>, i: usize, exceptions: &Vec<(usize, Vec<usize>)>) {
	for (t, token) in tokens.iter().enumerate() {
		let mut children = token.children.borrow_mut();
		let mut c = 0;
		'outer: while c < children.len() {
			for e in exceptions {
				if e.0 == t && e.1.contains(&children[c]) {
					c += 1;
					continue 'outer;
				}
			}
			
			if children[c] >= i && children[c] != usize::MAX {
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
		} else {
			let new_parent = tokens[parent].clone();
			new_parent.children.borrow_mut().clear();
			code.push(new_parent);
		},
		
		Kind::Op(ref op) => {
			let mut name = op.to_string();
			let mut i = parent;
			
			get_op_name(tokens, functions, &mut i, &mut name);
			
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
				
				if i < children.len() {
					code.push(Token {
						kind: Kind::GroupOp(String::from(";")),
						pos: FilePos {line: 0, col: 0},
						children: RefCell::new(Vec::new())
					});
				}
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

fn parse_macro_func(tokens: &mut Vec<Token>, macros: &mut Vec<Macro>, functions: &mut Vec<Function>, i: &mut usize, name: &str, name_tok_len: usize, depth: usize, rows: &Vec<usize>, found: &mut bool) -> Result<(), Error> {
	let mut j = 0;
	while j < macros.len() {
		if name == &macros[j].func.name && depth >= macros[j].depth && rows[macros[j].depth] == macros[j].row {
			*found = true;
			
			// Parse function args
			let args = tokens[*i].children.borrow().clone();
			let mut new_code = Vec::new();
			let mut new_points: Vec<Vec<Token>> = Vec::new();
			if args.len() >= 1 && args[0] != usize::MAX {
				'tok_search: for token in macros[j].code.iter() {
					match token.kind {
						Kind::Var(ref name, _) => {
							for (a, arg) in args.iter().enumerate() {
								if name == &macros[j].func.args[a].name {
									add_to_code(tokens, functions, &mut new_code, *arg);
									continue 'tok_search;
								}
							}
							
							new_code.push(token.clone())
						},
						
						_ => new_code.push(token.clone())
					}
				}
					
				for (p, point) in macros[j].returns.iter().enumerate() {
					new_points.push(Vec::new());
					'tok_search2: for token in point.iter() {
						match token.kind {
							Kind::Var(ref name, _) => {
								for (a, arg) in args.iter().enumerate() {
									if name == &macros[j].func.args[a].name {
										add_to_code(tokens, functions, &mut new_points[p], *arg);
										continue 'tok_search2;
									}
								}
								
								new_points[p].push(token.clone())
							},
							
							_ => new_points[p].push(token.clone())
						}
					}
				}
			} else {
				new_code = macros[j].code.clone();
				new_points = macros[j].returns.clone();
			}
			
			// Save parent of macro call
			let mut exceptions = Vec::new();
			let mut parent = (0, 0);
			'outer: for (t, tok) in tokens.iter_mut().enumerate() {
				let mut children = tok.children.borrow_mut();
				for (c, child) in children.iter().enumerate() {
					if *child == *i {
						exceptions.push((t, vec![*child]));
						parent = (t, c);
						
						break 'outer;
					}
				}
			}
			
			// Remove macro call since it will be replaced later
			let mut trash = del_all_children(tokens, &args);
			let mut t = 0;
			while t < trash.len() {
				tokens.remove(trash[t]);
				correct_indexes_after_del(tokens, trash[t]);
				if parent.0 > trash[t] {
					parent.0 -= 1;
					exceptions[0].0 -= 1;
				}
				
				if *i > trash[t] {
					*i -= 1;
				}
				
				let mut i = t + 1;
				while i < trash.len() {
					if trash[i] > trash[t] && trash[i] != usize::MAX {
						trash[i] -= 1;
					}
					
					i += 1;
				}
				
				t += 1;
			}
			
			for _ in 0..name_tok_len {
				tokens.remove(*i);
				correct_indexes_after_del(tokens, *i);
				if parent.0 > *i {
					parent.0 -= 1;
					exceptions[0].0 -= 1;
				}
			}
			
			// Parse macro function
			*functions = parse(&new_code, functions.clone()); // Ik, it's not good to clone for performance but I was just too lazy to fix the issues...
			parse2(&mut new_code, &functions, &mut 2);
			
			let mut lowest = [1, 1];
			for (p, mut point) in new_points.iter_mut().enumerate() {
				if let Some(token) = parse_statement(&mut point, &functions, &mut 0) {
					lowest[p] = token;
				}
			}
			
			// Compile macro function
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
					
					while k < out_contents.len() {
						if let Ok(_) = &out_contents[k..k + 1].parse::<usize>() {
							k += 1;
						} else {
							break;
						}
					}
					
					out_contents.insert(k, ')');
				}
				
				k += 1;
			}
			
			if new_points.len() == 0 {
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
				
				if !out.stderr.starts_with(b"\x1b[0m\x1b[1m\x1b[38;5;11mwarning") {
					error = true;
				}
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
						// Replace macro function call with results
						
						let point = str::from_utf8(&out.stderr).unwrap()[7..out.stderr.len() - 1].parse::<usize>();
						
						if let Ok(point) = point {
							tokens[parent.0].children.borrow_mut()[parent.1] = *i + lowest[point] - 1; // -1 because 'point' starts with semicolon that is ignored later
							exceptions[0].1[0] = *i + lowest[point] - 1;
							
							let length = &new_points[point].len();
							for (t, token) in new_points[point][1..length - 1].iter().enumerate() {
								tokens.insert(*i, token.clone());
								
								for e in exceptions.iter_mut() {
									if e.0 >= *i {
										e.0 += 1;
									}
								}
								
								exceptions.push((*i, Vec::new()));
								let e = exceptions.len() - 1;
								
								{
									let mut children = tokens[*i].children.borrow_mut();
									for child in children.iter_mut() {
										*child = *i + *child - t - 1;
										exceptions[e].1.push(*child);
									}
								}
								
								correct_indexes_after_add(tokens, *i, &exceptions);
								
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
			
//			fs::remove_dir("macros")?; // Doesn't work (on Windows) for some reason?
			
			break;
		}
		
		j += 1;
	}
	
	Ok(())
}

pub fn parse3(tokens: &mut Vec<Token>, macros: &mut Vec<Macro>, functions: &mut Vec<Function>, i: &mut usize, depth: &mut usize, rows: &mut Vec<usize>) -> Result<(), Error> {
	match tokens[*i].kind.clone() {
//		Kind::Var(ref name, _) => return parse_macro_func(tokens, macros, functions, i, name, 1, *depth, rows, &mut false),
		
		Kind::Op(ref op, _) if op != ":" => { // 'op != ":"' part is tmp, used to allow Rust-style importing
			let mut name = op.to_string();
			let start = *i;
			
			get_op_name(tokens, functions, i, &mut name);
			
			let end = *i;
			*i = start;
			
			let mut found = false;
//			let res = parse_macro_func(tokens, macros, functions, i, &name, name.len(), *depth, rows, &mut found);
			
			if !found {
				*i = end;
			}
			
//			return res;
		},
		
		Kind::GroupOp(ref op, _) if op == "{" => {
			*depth += 1;
			if *depth + 1 > rows.len() {
				rows.push(0);
			} else {
				rows[*depth] += 1;
			}
		},
		
		Kind::GroupOp(ref op, _) if op == "}" => if *depth > 0 {
			*depth -= 1;
		} else {
			panic!("{}:{} Excess ending bracket", tokens[*i].pos.line, tokens[*i].pos.col);
		},
		
		_ => ()
	}
	
	Ok(())
}

fn get_op_name(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, name: &mut String) {
	*i += 1;
	while *i < tokens.len() {
		if let Kind::Op(ref op, _) = tokens[*i].kind {
			*name += op;
			*i += 1;
		} else {
			break;
		}
	}
	*i -= 1;
	
	while *i > 0 {
		if let Some(_) = is_defined(functions, &name) { // NEEDS FIXING FOR RETURN ARROWS [EDIT: Has this been fixed yet?]
			break;
		} else {
			name.pop();
		}
		
		*i -= 1;
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
						Kind::GroupOp(ref op) if op == ";" => {
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
			
			get_op_name(tokens, functions, i, &mut name);
			
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
				
				*i = *statement;
				let mut nests = 0;
				while *i + 1 < tokens.len() {
					match tokens[*i + 1].kind {
						Kind::GroupOp(ref op) if op == ";" => {
							output += ";";
							break;
						},
						
						Kind::GroupOp(ref op) if op == "{" => nests += 1,
						
						Kind::GroupOp(ref op) if op == "}" => if nests > 0 {
							nests -= 1;
						} else {
							break;
						},
						
						_ => ()
					}
					
					*i += 1;
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
} */

fn compile_type(typ: &Vec<Vec<Type>>) -> String {
	use lib::Type::*;
	
	let mut output = String::new();
	let mut unsigned = false;
	
	for t in &typ[0] { // TMP until I've worked out how to handle multiple types
		match t {
			Array => (), // WIP
			Bool => output += "bool",
			Chan => (), // WIP
			Char => output += "char",
			Const => (),
			Fraction => (), // WIP
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
	
	output
}

fn compile_func(function: &Function, mut output: String) -> String {
	let mut is_init = false;
	for section in function.structure.iter() {
		match section {
			FunctionSection::ID(ref name) | FunctionSection::OpID(ref name) => {
				if name == "init" {
					is_init = true;
				} else {
					is_init = false;
					break;
				}
			},
			
			_ => ()
		}
	}
	
	if is_init {
		output += "main";
	} else {
		let mut s = String::new();
		for section in function.structure.iter() {
			match section {
				FunctionSection::ID(ref name) | FunctionSection::OpID(ref name) => {
					for c in name.chars() {
						let ch = c.to_string();
						s += match c {
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
							_ => &ch
						};
					}
					
					s += "_";
				},
				
				_ => ()
			}
		}
		
		output += &s[..s.len() - 1];
	}
		
	output += "(";
	
	let mut not_first_arg = false;
	for section in function.structure.iter() {
		match section {
			FunctionSection::Arg(ref name, ref typ) => {
				if not_first_arg {
					output += ",";
				}
				
				output += name;
				output += ":";
				output += &compile_type(typ);
				
				not_first_arg = true;
			},
			
			_ => ()
		}
	}
	
	output += ")";
	
	if function.output.len() > 0 {
		output += "->";
		output += &compile_type(&function.output);
	}
	
	output
}

fn type_full_name(tokens: &Vec<Token>, mut output: String, sidekicks: &RefCell<Vec<usize>>, name: &str) -> String {
	if sidekicks.borrow().len() > 0 {
		let mut s = name.to_string() + "_";
		
		for sidekick in sidekicks.borrow().iter() {
			match tokens[*sidekick].kind {
				Kind::Op(ref op, _, _) => {
					// WIP
				},
				
				Kind::Var(ref name, _, _, _) => {
					s += name;
					s += "_";
				},
				
				_ => unreachable!()
			}
		}
		
		output + &s[..s.len() - 1]
	} else if name == "println" {
		output + "println!"
	} else {
		output + name
	}
}

fn compile_tok(tokens: &Vec<Token>, i: &mut usize, mut output: String) -> String {
	match tokens[*i].kind {
		Kind::GroupOp(_,_,_) => output = compile_body(tokens, i, output),
		
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
		
		Kind::Str2(ref s) => {
			if s.len() == 1 || (s.len() == 2 && s.chars().next().unwrap() == '\\') { // Just a character, not an actual string
				output += "'";
				output += s;
				output += "'";
			} else {
				panic!("{}:{} P+ style strings are not supported yet", tokens[*i].pos.line, tokens[*i].pos.col);
			}
		},
		
		Kind::Var(ref name, _, ref children, ref sidekicks) | Kind::Op(ref name, ref children, ref sidekicks) => {
			output = type_full_name(tokens, output, sidekicks, name);
			
			if children.borrow().len() > 0 {
				output += "(";
				
				if sidekicks.borrow().len() == 0 && name == "println" {
					output += "\"{}\",";
				}
				
				if children.borrow()[0] != usize::MAX {
					for (c, child) in children.borrow().iter().enumerate() {
						*i = *child;
						output = compile_tok(tokens, i, output);
						
						if c + 1 < children.borrow().len() {
							output += ",";
						}
					}
				}
				
				output += ")";
			}
		},
		
		Kind::Op(ref name, ref children, ref sidekicks) => {
			// WIP
			
			output = type_full_name(tokens, output, sidekicks, name);
		},
		
		_ => ()
	}
	
	output
}

fn compile_body(tokens: &Vec<Token>, i: &mut usize, mut output: String) -> String {
	output += "{";
	
	if let Kind::GroupOp(_, ref statements, _) = tokens[*i].kind {
		for statement in statements.borrow().iter() {
			*i = *statement;
			output = compile_tok(tokens, i, output);
		}
	}
	
	output += "}";
	
	output
}

pub fn compile(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, mut output: String) -> String {
	match tokens[*i].kind {
		Kind::Func(f, ref children) => {
			output += "fn ";
			
			output = compile_func(&functions[f], output);
			
			*i = children.borrow()[0];
			output = compile_body(tokens, i, output);
		},
		
		Kind::Reserved(ref keyword, _) if keyword == "import" => {
			// Using Rust-style importing for now
			output += "use ";
			*i += 1;
			
			let mut success = false;
			while *i < tokens.len() {
				match tokens[*i].kind {
					Kind::Reserved(ref keyword, _) if keyword == "as" => {
						output += " as ";
						*i += 1;
					},
					
					Kind::GroupOp(ref op, _, _) if op == ";" => {
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
		
		Kind::Var(ref name, _, _, _) if name == "#" => {
			while *i < tokens.len() {
				match tokens[*i].kind {
					Kind::GroupOp(ref op, _, _) if op == "]" => {
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
}

/* pub fn compile(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, mut output: String) -> String {
	use lib::Type::*;
	use lib::Kind::*;
	
	let children = tokens[*i].children.borrow();
	
	match tokens[*i].kind {
		Reserved(ref keyword, _) if keyword == "import" => {
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
					
					Kind::GroupOp(ref op) if op == ";" => {
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
		
		Reserved(ref keyword, _) if keyword == "func" => {
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
					
					*i = *statement;
					let mut nests = 0;
					while *i + 1 < tokens.len() {
						match tokens[*i + 1].kind {
							Kind::GroupOp(ref op) if op == ";" => {
								output += ";";
								break;
							},
							
							Kind::GroupOp(ref op) if op == "{" => nests += 1,
							
							Kind::GroupOp(ref op) if op == "}" => if nests > 0 {
								nests -= 1;
							} else {
								break;
							},
							
							_ => ()
						}
						
						*i += 1;
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
} */