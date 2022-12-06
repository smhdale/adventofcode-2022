use crate::input;
use crate::print;

fn find_max_calories(elves: Vec<Vec<usize>>) -> usize {
	let mut max: usize = 0;
	for elf in elves {
		let sum = elf.iter().sum();
		if sum > max {
			max = sum;
		}
	}
	max
}

fn sum_calories(elves: Vec<Vec<usize>>) -> Vec<usize> {
	elves.iter().map(|elf| elf.iter().sum()).collect()
}

// PART 1
// Find the elf holding the most calories worth of food, and sum their total
pub fn part1() {
	print::intro(1, 1);

	let elves_test = input::day_input_grouped::<usize>(1, "input_test.txt");
	let elves = input::day_input_grouped::<usize>(1, "input.txt");

	let max_test = find_max_calories(elves_test);
	let max = find_max_calories(elves);

	print::answer_with_test(max_test, max);
}

// PART 2
// Find the top three elves with the most calories, and sum the total
pub fn part2() {
	print::intro(1, 2);

	let elves = input::day_input_grouped::<usize>(1, "input.txt");
	let mut totals = sum_calories(elves);
	totals.sort_unstable();
	totals.reverse();

	let mut sum = 0;
	for i in 0..3 {
		if let Some(total) = totals.get(i) {
			sum += total;
		}
	}

	print::answer(sum);
}
