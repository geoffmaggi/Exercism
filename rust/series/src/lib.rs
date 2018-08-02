/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Series

pub fn series(digits: &str, len: usize) -> Vec<String> {
  let mut ret = Vec::new();
	if len <= digits.len() {
		for i in 0..(digits.len() - len + 1) {
			ret.push(digits[i..i+len].to_string());
		}
	}
	ret
	//unimplemented!("What are the series of length {} in string {:?}", len, digits)
}
