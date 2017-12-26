#[derive(Clone)]
#[derive(Debug)]
pub struct Token {
	val: String,
	t: &'static str
}

pub fn lex<'a>(contents: &'a String) -> Vec<&'a str> {
	let mut result = Vec::new();
	let mut last = 0;
	for (index, matched) in contents.match_indices(|c: char| !(c.is_alphanumeric())) {
		if last != index {
			result.push(&contents[last..index]);
		}
		
		result.push(matched);
		
		last = index + matched.len();
	}
	
	if last < contents.len() {
		result.push(&contents[last..]);
	}
	
	result
}

pub fn lex2(tokens: Vec<&str>) -> Vec<Token> {
	let mut res: Vec<Token> = Vec::new();
	let mut string = Token {
		val: String::from(""),
		t: "str1"
	};
	let mut in_str = false;
	let mut in_str2 = false;
	
	for item in tokens {
		if in_str {
			if item == "\"" {
				res.push(string.clone());
				string.val = String::from("");
				in_str = false;
			} else {
				string.val += item;
			}
		} else if in_str2 {
			if item == "'" {
				res.push(string.clone());
				string.val = String::from("");
				in_str2 = false;
			} else {
				string.val += item;
			}
		} else if item == "\"" {
			string.t = "str1";
			in_str = true;
		} else if item == "'" {
			string.t = "str2";
			in_str2 = true;
		} else {
			string.val = item.to_string();
			string.t = if item.parse::<u64>().is_ok() {
				"num"
			} else if item == "+" || item == "-" || item == "*" || item == "/" || item == "=" || item == "&" || item == "|" || item == "^" || item == ">" || item == "<" || item == "[" || item == "]" || item == "(" || item == ")" || item == "<" || item == ">" {
				"operator"
			} else {
				"var"
			};
			
			res.push(string.clone());
		}
	}
	
	res
}