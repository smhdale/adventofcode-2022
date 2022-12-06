use std::fmt::Display;

pub fn intro(day: u8, part: u8) {
	println!("=== DAY {}, PART {} ===", day, part);
}

pub fn answer<T: Display>(value: T) {
	println!("Answer: {}", value);
}

pub fn answer_with_test<T: Display>(value_test: T, value: T) {
	println!("Test:   {}\nAnswer: {}\n", value_test, value);
}
