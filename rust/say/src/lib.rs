/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Say

pub fn encode(n: u64) -> String {
	if n == 0 {
		return "zero".to_string();
	}
	
	//Step 2
	let hundreds = n%10_u64.pow(3);
	let thousands = (n%10_u64.pow(6))/10_u64.pow(3);
	let millions = (n%10_u64.pow(9))/10_u64.pow(6);
	let billions = (n%10_u64.pow(12))/10_u64.pow(9);
	let trillions = (n%10_u64.pow(15))/10_u64.pow(12);
	let quadrillions = (n%10_u64.pow(18))/10_u64.pow(15);
	let quintillions = n/10_u64.pow(18);
	
	let mut results = String::new();
	
	//Step 3 and 4
	if quintillions >= 1 {
		results = results + &say_chunk(quintillions) + " quintillion "
	}
	if quadrillions >= 1 {
		results = results + &say_chunk(quadrillions) + " quadrillion "
	}
	if trillions >= 1 {
		results = results + &say_chunk(trillions) + " trillion "
	}
	if billions >= 1 {
		results = results + &say_chunk(billions) + " billion "
	}
	if millions >= 1 {
		results = results + &say_chunk(millions) + " million "
	}
	if thousands >= 1 {
		results = results + &say_chunk(thousands) + " thousand "
	}
	results = results + &say_chunk(hundreds);
	
	results.trim().to_string()
}

//Part 1
fn say_chunk(n: u64) -> String {
	let first19 = vec!["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eightteen", "nineteen"];
	let prefixes = vec!["", "", "twen", "thir", "for", "fif", "six", "seven", "eigh" ,"nin"];
	
	let mut results = String::new();
	
	//Step 1
	let num = n%100;
	if num < 20 {
		results = first19[num as usize].to_string();
	}
	else if num < 100 {
		if num%10 == 0 {
			results = prefixes[(num/10) as usize].to_string() + "ty";
		}
		else {
			results = prefixes[(num/10) as usize].to_string() + "ty-" + first19[(num%10) as usize];
		}
	}
	
	if n < 1000 && n/100 >= 1 {
		results = first19[(n/100) as usize].to_string() + " hundred " + &results;
	}
	
	results.to_string()
}