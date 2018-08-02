/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Collatz Conjecture

pub fn collatz(n: u64) -> Option<u64> {
	if n < 1 {return None};
	let mut res = n;
	let mut step = 0;
	
	while res != 1 {
		if res % 2 == 0 {
			res /= 2;
		}
		else {
			res = res * 3 + 1;
		}
		step = step + 1;
	}
	Some(step)
	
	//unimplemented!(
	//	"return Some(x) where x is the number of steps required to reach 1 starting with {}",
	//	n,
	//)
}
