pub fn lex<'a>(contents: &'a String) -> Vec<&'a str> {
	let mut result = Vec::new();
	let mut last = 0;
	for (index, matched) in contents.match_indices(|c: char| !(c.is_alphanumeric())) {
		if last != index {
			result.push(&contents[last..index]);
		}
		
		if matched != " " {
			result.push(matched);
		}
		
		last = index + matched.len();
	}
	
	if last < contents.len() {
		result.push(&contents[last..]);
	}
	
	result
}