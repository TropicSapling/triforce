use lib::{Token, Kind, Type, Function, FunctionArg};

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
			name: String::from("=="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				}
			],
			precedence: 242,
			output: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("!="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				}
			],
			precedence: 242,
			output: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("<"),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				}
			],
			precedence: 243,
			output: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("<="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				}
			],
			precedence: 243,
			output: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from(">"),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				}
			],
			precedence: 243,
			output: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from(">="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'typ' structure needs support for multiple types (all types in this case)
				}
			],
			precedence: 243,
			output: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("&&"),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
				},
				FunctionArg {
					name: $b,
					typ: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
				}
			],
			precedence: 238,
			output: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("||"),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
				},
				FunctionArg {
					name: $b,
					typ: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
				}
			],
			precedence: 237,
			output: [Type::Bool, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("<<"),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
				}
			],
			precedence: 244,
			output: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from(">>"),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
				}
			],
			precedence: 244,
			output: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("^"),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
				}
			],
			precedence: 240,
			output: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				}
			],
			precedence: 0,
			output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("+="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				}
			],
			precedence: 0,
			output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("-="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				}
			],
			precedence: 0,
			output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("*="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				}
			],
			precedence: 0,
			output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("/="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				}
			],
			precedence: 0,
			output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("%="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				}
			],
			precedence: 0,
			output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from(">>="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				}
			],
			precedence: 0,
			output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("<<="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				}
			],
			precedence: 0,
			output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("^="),
			pos: 1,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				},
				FunctionArg {
					name: $b,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; 'macro' types are not yet implemented
				}
			],
			precedence: 0,
			output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]
		},
		
		Function {
			name: String::from("println"),
			pos: 0,
			args: vec![
				FunctionArg {
					name: $a,
					typ: [Type::Int, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void] // WIP; No support for strings yet
				}
			],
			precedence: 1,
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
					functions.push(Function {name: String::from(""), pos: 0, args: vec![], precedence: 1, output: [Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void, Type::Void]});
					func_pos = i;
					func = true;
				},
				_ => ()
			},
			
			Kind::Type(_) => match tokens[i + 1].kind {
				Kind::GroupOp(ref op) if op == "{" => {
					let end = i;
					while i > 0 {
						match tokens[i].kind {
							Kind::Type(ref typ) => par_type[end - i] = typ.clone(),
							_ => break
						}
						
						i -= 1;
					}
					
					let mut j = 0;
					while j + 1 < 8 && par_type[j + 1] != Type::Void {
						j += 1;
					}
					
					let mut k = 0;
					while j != k && j + 1 != k {
						let tmp = par_type[j].clone();
						par_type[j] = par_type[k].clone();
						par_type[k] = tmp;
						
						j -= 1;
						k += 1;
					}
					
					i += 1;
					tokens[func_pos].children.borrow_mut().push(i);
					
					i = end;
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
				if functions[last_item].name == "**" {
					functions[last_item].precedence = 247;
				} else if par_type[0] != Type::Void {
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
				if functions[last_item].name == "**" {
					functions[last_item].precedence = 247;
				} else if par_type[0] != Type::Void {
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
		
		let mut skip = false;
		
		match tokens[i + j].kind {
			Kind::Op(_) => skip = true,
			
			Kind::GroupOp(_) | Kind::Type(_) => {
				j += 1;
				offset += 1;
				continue;
			},
			
			_ => ()
		};
		
		if k < tokens.len() {
			match tokens[i + j + 1].kind {
				Kind::Op(_) if skip => offset += 1,
				_ => {
					j += 1;
					offset += 1;
					continue;
				}
			}
		} else {
			tokens[start].children.borrow_mut().push(i + j);
		}
		
		if skip {
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
		}
		
		j += 1;
	}
}

fn parse_statement(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize) -> Option<usize> {
	match tokens[*i + 1].kind {
		Kind::GroupOp(ref op) if op == "}" => {
			*i += 1;
			return Some(*i - 1);
		},
		_ => ()
	};
	
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
					
					Kind::Op(ref op) if op == ";" => {
						*i += 1;
						break;
					},
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
						} else {
							let mut j = 1;
							while j < name.len() {
								if let Some(def) = is_defined(functions, &name[..name.len() - j]) {
									match highest {
										Some(func) => if (def.precedence > func.1.precedence && depth == func.2) || depth > func.2 {
											highest = Some((start, def, depth));
										},
										None => highest = Some((start, def, depth))
									};
									
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
					Kind::GroupOp(ref op) if op == ")" => depth -= 1,
					
					_ => ()
				};
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
			parse_if(tokens, functions, i);
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
	
	body.push(*i);
	
	*i = start;
	parse_statement(tokens, functions, i);
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
						
						_ => if let Some(token) = parse_statement(tokens, functions, i) {
							body.push(token);
						} else {
							body.push(start); // Should this really be pushing start instead of *i?
							*i += 1;
						}
					}
				};
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
						if let Some(_) = is_defined(functions, &name) { // NEEDS FIXING FOR RETURN ARROWS
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
					';' => "semic",
					_ => unreachable!()
				};
			}
			
			let name = new_name;
			
/*			let mut name = match op.as_ref() {
				"+" => "plus",
				"-" => "minus",
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
			*i -= 1; */
			
			let args = tokens[start].children.borrow();
			
			if name == "plus" || name == "pluseq" || name == "minus" || name == "minuseq" || name == "times" || name == "timeseq" || name == "div" || name == "diveq" ||
			   name == "mod" || name == "modeq" || name == "eq" || name == "eqeq" || name == "noteq" || name == "andand" || name == "or" || name == "oror" || name == "xor" ||
			   name == "xoreq" || name == "larrow" || name == "rarrow" || name == "larrowlarrow" || name == "larrowlarroweq" || name == "rarrowrarrow" || name == "rarrowrarroweq" {
				*i = args[0];
				output = compile_func(tokens, functions, i, output);
				
				output += match name.as_ref() {
					"plus" => "+",
					"pluseq" => "+=",
					"minus" => "-",
					"minuseq" => "-=",
					"times" => "*",
					"timeseq" => "*=",
					"div" => "/",
					"diveq" => "/=",
					"mod" => "%",
					"modeq" => "%=",
					"eq" => "=",
					"eqeq" => "==",
					"noteq" => "!=",
					"andand" => "&&",
					"or" => "|",
					"oror" => "||",
					"xor" => "^",
					"xoreq" => "^=",
					"larrow" => "<",
					"rarrow" => ">",
					"larrowlarrow" => "<<",
					"larrowlarroweq" => "<<=",
					"rarrowrarrow" => ">>",
					"rarrowrarroweq" => ">>=",
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
			output += "let mut "; // Mutable by default, constants coming soon
			
			*i = tokens[*i].children.borrow()[0];
			output = compile_func(tokens, functions, i, output);
		},
		
		Kind::Str1(ref s) => {
			output += "\"";
			output += s;
			output += "\"";
		},
		
		Kind::Str2(_) => {
			panic!("P+ style strings are not supported yet");
		},
		
		Kind::Type(ref typ) => {
			use lib::Type::*;
			
			let mut types = vec![typ];
			*i += 1;
			while *i < tokens.len() {
				match tokens[*i].kind {
					Kind::Type(ref typ) => types.push(typ),
					_ => break
				}
				
				*i += 1;
			}
			*i -= 1;
			
			let mut unsigned = false;
			for typ in types {
				match *typ {
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
					Void => output += "()",
					Volatile => (), // WIP
				}
			}
		},
		
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
				
				if name == "println" {
					output += "\"{}\",";
				}
				
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
		
		_ => () // WIP
	};
	
	output
}

pub fn compile(tokens: &Vec<Token>, functions: &Vec<Function>, i: &mut usize, mut output: String) -> String {
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