/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Bob

pub fn reply(message: &str) -> &str {
	let message = message.trim(); //fix for tests
	let mut chars = message.chars().filter(|c| c.is_alphabetic()).peekable(); //contains just the Ascii chars of the string
	
	if message.is_empty() {
		"Fine. Be that way!"
	}
	else if chars.peek().is_some() && chars.all(|c| c.is_ascii_uppercase()) {
		if message.chars().last() == Some('?') {
			"Calm down, I know what I'm doing!"
		}
		else {
			"Whoa, chill out!"
		}
	}
	else if message.chars().last() == Some('?') {
		"Sure."
	}
	else {
		"Whatever."
	}
	//unimplemented!("have Bob reply to the incoming message: {}", message)
}

