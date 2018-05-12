#![feature(test)]

extern crate test;

struct MyString(String);

fn get_mystr(s: MyString) -> String {
	let mut i = 0;
	let mut new_str = String::new();
	while i < s.0.len() { // IGNORES UTF-8; NEEDS FIXING
		new_str += &s.index(i).to_string();
		i += 1;
	}
	
	new_str
}

fn get_str(s: String) -> String {
	let mut new_str = String::new();
	for c in s.chars() {
		new_str += &c.to_string();
	}
	
	new_str
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;
	
	#[bench]
	fn bench_mystring(b: &mut Bencher) {
		b.iter(|| get_mystr(MyString(String::from("This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string."))));
	}
	
	#[bench]
	fn bench_string(b: &mut Bencher) {
		b.iter(|| get_str(String::from("This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string. This is a somewhat long string compared to the previous a bit shorter (or well very short in fact) string.")));
	}
}

impl MyString {
	fn index(&self, index: usize) -> char {
		self.0[index..].chars().next().unwrap()
	}
}