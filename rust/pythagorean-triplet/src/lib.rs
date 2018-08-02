/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Pythagorean Triplet

pub fn find() -> Option<u32> {
	//Brute force method
	for a in 1..1000 {
		for b in a..1000 {
			for c in b..1000 {
				if a*a + b*b == c*c && a + b + c == 1000 {
					return Some(a * b * c);
				}
			}
		}
	}
	None
  //unimplemented!();
}
