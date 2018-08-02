/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Isogram

use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
	let mut seen = HashSet::new();
	for c in candidate.to_lowercase().chars().filter(|c| c.is_alphanumeric()) {
		if seen.contains(&c) {
			return false;
		}
		seen.insert(c);
	}
	true
}