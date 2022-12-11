use crate::input;
use crate::print;

// PART 1

pub fn part1() {
    print::intro(day_number, 1);

    let data = input::day_input::<>(day_number);

    print::answer_with_test(value, value_test);
}

// PART 2

pub fn part2() {
    print::intro(day_number, 2);

    let data = input::day_input::<>(day_number);

    print::answer_with_test(value, value_test);
}
