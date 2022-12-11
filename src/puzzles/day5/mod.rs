use regex::Regex;
use std::cmp;
use std::str::FromStr;
use std::string::ParseError;

use crate::input;
use crate::print;

type Crate = char;
type Stack = Vec<Crate>;

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    repeat: usize,
}

impl FromStr for Instruction {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"\d+").unwrap();

        let mut parts = regex.find_iter(s).map(|m| m.as_str().parse::<usize>());
        let repeat = parts.next().unwrap().expect("Invalid instruction");
        let from = parts.next().unwrap().expect("Invalid instruction") - 1;
        let to = parts.next().unwrap().expect("Invalid instruction") - 1;

        Ok(Instruction { from, to, repeat })
    }
    type Err = ParseError;
}

/**
 * A ship with a crane that keeps track of stacks of cargo, and can move cargo
 * around based in a set of instructions.
 */
struct Ship {
    stacks: Vec<Stack>,
    is_crane_version_9001: bool,
}

impl Ship {
    pub fn new(initial_state: &[String], is_crane_version_9001: bool) -> Ship {
        let mut diagram = initial_state.iter().rev();

        // Get number of stacks on ship
        let num_stacks = diagram
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("Invalid input");

        // Create stacks
        let mut stacks = vec![];
        for _ in 0..num_stacks {
            stacks.push(Stack::new());
        }

        // Add crates to stacks
        for line in diagram {
            let chars = line.as_bytes();
            for i in 0..chars.len() {
                if let (Some(stack), Some(c)) = (stacks.get_mut(i), chars.get(1 + i * 4)) {
                    if *c != b' ' {
                        stack.push(*c as char);
                    }
                }
            }
        }

        Ship {
            stacks,
            is_crane_version_9001,
        }
    }

    fn move_crates(&mut self, from: usize, to: usize, items: usize) {
        // Ensure source and target stacks exist
        if self.stacks.len() - 1 < cmp::max(from, to) {
            return;
        }

        // Pop substack off source stack
        let mut substack = match self.stacks.get_mut(from) {
            Some(stack_from) => stack_from.split_off(stack_from.len() - items),
            None => vec![],
        };

        // Append substack to target stack
        if let Some(target) = self.stacks.get_mut(to) {
            target.append(&mut substack);
        }
    }

    pub fn apply_instruction_9000(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.repeat {
            self.move_crates(instruction.from, instruction.to, 1);
        }
    }

    pub fn apply_instruction_9001(&mut self, instruction: &Instruction) {
        self.move_crates(instruction.from, instruction.to, instruction.repeat);
    }

    pub fn apply_instructions(&mut self, instructions: &[Instruction]) {
        instructions.iter().for_each(|i| {
            if self.is_crane_version_9001 {
                self.apply_instruction_9001(i);
            } else {
                self.apply_instruction_9000(i);
            }
        });
    }

    pub fn read_top_layer(&self) -> String {
        String::from_iter(self.stacks.iter().filter_map(|s| s.last()))
    }
}

fn create_ship_and_instructions(
    input: &[Vec<String>],
    is_crane_version_9001: bool,
) -> (Ship, Vec<Instruction>) {
    let mut iter = input.iter();
    let raw_ship = iter.next().expect("Invalid input");
    let raw_instructions = iter.next().expect("Invalid input");

    let ship = Ship::new(raw_ship, is_crane_version_9001);
    let instructions = raw_instructions
        .iter()
        .map(|i| i.parse::<Instruction>().expect("Invalid input"))
        .collect();

    (ship, instructions)
}

fn simulate_ship(input: &[Vec<String>], is_crane_version_9001: bool) -> String {
    let (mut ship, instructions) = create_ship_and_instructions(input, is_crane_version_9001);
    ship.apply_instructions(&instructions);
    ship.read_top_layer()
}

// PART 1

pub fn part1() {
    print::intro(5, 1);

    let data = input::day_input_grouped::<String>(5);

    let result_test = simulate_ship(&data.test, false);
    let result_real = simulate_ship(&data.real, false);

    print::answer_with_test(result_real, result_test);
}

// PART 2

pub fn part2() {
    print::intro(5, 2);

    let data = input::day_input_grouped::<String>(5);

    let result_test = simulate_ship(&data.test, true);
    let result_real = simulate_ship(&data.real, true);

    print::answer_with_test(result_real, result_test);
}
