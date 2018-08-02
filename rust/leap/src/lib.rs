/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Leap

pub fn is_leap_year(year: i32) -> bool {
	year%4 == 0 && (year%100 != 0 || year%400 == 0)
	//unimplemented!("true if {} is a leap year", year)
}
