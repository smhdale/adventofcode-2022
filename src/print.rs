use std::fmt::Display;

pub fn intro(day: u8, part: u8) {
	println!("=== DAY {}, PART {} ===", day, part);
}

pub fn answer<T: Display>(value: T) {
	println!("Answer: {}\n", value);
}

pub fn answer_with_test<T: Display>(value: T, value_test: T) {
	println!("Test:   {}\nAnswer: {}\n", value_test, value);
}
