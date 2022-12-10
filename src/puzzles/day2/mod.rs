use crate::input;
use crate::print;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Copy, Clone)]
enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

struct RoundInputs {
    player: char,
    opponent: char,
}

struct RoundActions {
    player: Action,
    opponent: Action,
}

type ActionResolver = dyn Fn(&RoundInputs) -> RoundActions;

// Round parsing

impl FromStr for RoundInputs {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s
            .split_ascii_whitespace()
            .map(|c| c.chars().next().unwrap())
            .into_iter();
        let opponent = chars.next().expect("Missing player move");
        let player = chars.next().expect("Missing opponent move");
        Ok(RoundInputs { player, opponent })
    }
    type Err = ParseError;
}

// Tournament simulation

fn simulate_round(actions: &RoundActions) -> isize {
    let points_shape = actions.player as isize;
    let points_outcome = match actions.opponent {
        Action::Rock => match actions.player {
            Action::Rock => Outcome::Draw,
            Action::Paper => Outcome::Win,
            Action::Scissors => Outcome::Loss,
        },
        Action::Paper => match actions.player {
            Action::Rock => Outcome::Loss,
            Action::Paper => Outcome::Draw,
            Action::Scissors => Outcome::Win,
        },
        Action::Scissors => match actions.player {
            Action::Rock => Outcome::Win,
            Action::Paper => Outcome::Loss,
            Action::Scissors => Outcome::Draw,
        },
    } as isize;
    points_shape + points_outcome
}

fn simulate_tournament(rounds: &Vec<RoundInputs>, action_resolver: &ActionResolver) -> isize {
    let mut score = 0;
    for round in rounds {
        let actions = action_resolver(&round);
        score += simulate_round(&actions);
    }
    score
}

// PART 1
// Simulate tournament using default move resolver

fn resolve_basic(inputs: &RoundInputs) -> RoundActions {
    fn map_char(c: char) -> Action {
        match c {
            'A' | 'X' => Action::Rock,
            'B' | 'Y' => Action::Paper,
            _ => Action::Scissors,
        }
    }
    RoundActions {
        player: map_char(inputs.player),
        opponent: map_char(inputs.opponent),
    }
}

pub fn part1() {
    print::intro(2, 1);
    let data = input::day_input::<RoundInputs>(2);

    let sum_test = simulate_tournament(&data.test, &resolve_basic);
    let sum = simulate_tournament(&data.real, &resolve_basic);

    print::answer_with_test(sum, sum_test);
}

// PART 2
// Second column dictates how round should end

fn resolve_for_outcome(inputs: &RoundInputs) -> RoundActions {
    let opponent: Action = match inputs.opponent {
        'A' => Action::Rock,
        'B' => Action::Paper,
        _ => Action::Scissors,
    };
    let player: Action = match inputs.player {
        'X' => {
            // Should lose
            match opponent {
                Action::Rock => Action::Scissors,
                Action::Paper => Action::Rock,
                Action::Scissors => Action::Paper,
            }
        }
        'Y' => {
            // Should draw
            opponent.clone()
        }
        _ => {
            // Should win
            match opponent {
                Action::Rock => Action::Paper,
                Action::Paper => Action::Scissors,
                Action::Scissors => Action::Rock,
            }
        }
    };
    RoundActions { player, opponent }
}

pub fn part2() {
    print::intro(2, 2);
    let data = input::day_input::<RoundInputs>(2);

    let sum_test = simulate_tournament(&data.test, &resolve_for_outcome);
    let sum = simulate_tournament(&data.real, &resolve_for_outcome);

    print::answer_with_test(sum, sum_test);
}
