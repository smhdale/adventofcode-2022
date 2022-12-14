use std::fmt;
use std::str::FromStr;
use std::string::ParseError;

use crate::input;
use crate::print;

enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let instruction = parts.next().unwrap();
        match instruction {
            // Addx
            "addx" => {
                let value: i32 = parts.next().unwrap().parse().unwrap();
                Ok(Instruction::Addx(value))
            }
            // Catchall (noop)
            _ => Ok(Instruction::Noop),
        }
    }
    type Err = ParseError;
}

/**
 * CRT screen, 6 lines of 40 chars
 */
struct Screen {
    lines: Vec<String>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            lines: (0..6)
                .map(|_| str::repeat(" ", 40))
                .collect::<Vec<String>>(),
        }
    }

    pub fn draw(&mut self, cycle: i32, register: i32) {
        if let Ok(c) = usize::try_from(cycle) {
            // Get screen position
            let col = c % 40;
            let row = (c / 40) % 6;

            if let Some(line) = self.lines.get_mut(row) {
                let cursor = cycle % 40;
                if (cursor - register).abs() < 2 {
                    line.replace_range(col..=col, "X");
                }
            }
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lines.join("\n"))
    }
}

/**
 * Handheld device; can run CPU cycles
 */

struct Device {
    cycle: i32,
    register: i32,
    measurements: Vec<i32>,
    screen: Screen,
}

impl Device {
    pub fn new() -> Device {
        Device {
            cycle: 0,
            register: 1,
            measurements: vec![],
            screen: Screen::new(),
        }
    }

    fn measure(&mut self) {
        let measurement = self.cycle * self.register;
        self.measurements.push(measurement);
    }

    fn tick(&mut self) {
        // Update screen every cycle
        self.screen.draw(self.cycle, self.register);

        // Tick cycle
        self.cycle += 1;

        // Should we measure during this cycle?
        if self.cycle % 40 - 20 == 0 {
            self.measure();
        }
    }

    fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                self.tick();
            }
            Instruction::Addx(value) => {
                self.tick();
                self.tick();
                self.register += value;
            }
        }
    }

    pub fn process_instructions(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.process_instruction(instruction);
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.screen)
    }
}

// PART 1

fn measure_and_sum_signals(instructions: &[Instruction]) -> i32 {
    let mut device = Device::new();
    device.process_instructions(instructions);
    device.measurements.iter().sum()
}

pub fn part1() {
    print::intro(10, 1);

    let data = input::day_input(10);
    let sum_test = measure_and_sum_signals(&data.test);
    let sum_real = measure_and_sum_signals(&data.real);

    print::answer_with_test(sum_test, sum_real);
}

// PART 2

fn simulate_and_return_device(instructions: &[Instruction]) -> Device {
    let mut device = Device::new();
    device.process_instructions(instructions);
    device
}

pub fn part2() {
    print::intro(10, 2);

    let data = input::day_input(10);
    let dev_test = simulate_and_return_device(&data.test);
    let dev_real = simulate_and_return_device(&data.real);

    print::answer_with_test_newline(dev_test, dev_real);
}
