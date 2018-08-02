/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Proverb

pub fn build_proverb(list: Vec<&str>) -> String {
	if list.len() < 1 {String::new()}
	else {
		let proverb: String = list.windows(2)
			.map(|s| format!("For want of a {} the {} was lost.\n", s[0], s[1]))
			.collect();
		
		format!("{}And all for the want of a {}.", proverb, list[0])
	}
  //unimplemented!("build a proverb from this list of items: {:?}", list)
}

