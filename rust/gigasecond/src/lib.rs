/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Gigasecond

extern crate chrono;
use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
  start + Duration::seconds(10i64.pow(9))
	//unimplemented!("What time is a gigasecond later than {}", start);
}
