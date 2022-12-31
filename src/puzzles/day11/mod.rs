use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use std::string::ParseError;

use queues::*;

use crate::input;
use crate::print;

// construct_uint! {
// 	pub struct BigUint(16);
// }

#[derive(Debug)]
enum OpValue {
    Old,
    New(u64),
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
}

/**
 * Operation
 */

#[derive(Debug)]
struct Operation {
    a: OpValue,
    b: OpValue,
    op: Op,
}

impl Operation {
    fn interpolate(&self, op_value: &OpValue, number: u64) -> u64 {
        match op_value {
            OpValue::Old => number,
            OpValue::New(n) => *n,
        }
    }

    pub fn apply(&self, number: u64) -> u64 {
        let a = self.interpolate(&self.a, number);
        let b = self.interpolate(&self.b, number);
        match self.op {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
        }
    }
}

impl FromStr for Operation {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let a = parts.nth(2).unwrap();
        let op = parts.next().unwrap();
        let b = parts.next().unwrap();

        fn parse_value(value: &str) -> OpValue {
            match value {
                "old" => OpValue::Old,
                n => OpValue::New(n.parse().unwrap()),
            }
        }

        fn parse_op(op: &str) -> Op {
            match op {
                "*" => Op::Mul,
                "-" => Op::Sub,
                _ => Op::Add,
            }
        }

        Ok(Operation {
            a: parse_value(a),
            b: parse_value(b),
            op: parse_op(op),
        })
    }
    type Err = ParseError;
}

/**
 * Monkey
 */

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Queue<u64>,
    operation: Operation,
    divisor: u64,
    target_pass: usize,
    target_fail: usize,
    inspections: u128,
}

impl Monkey {
    pub fn new(details: &[String]) -> Monkey {
        let mut lines = details.iter();

        // Split helpers
        fn split_on_colon(raw: &str) -> &str {
            raw.split(": ").nth(1).unwrap()
        }
        fn get_last_int<T>(raw: &str) -> T
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            raw.split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        }

        // Parse ID
        let id: usize = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .replace(':', "")
            .parse()
            .unwrap();

        // Parse items
        let item_values = split_on_colon(lines.next().unwrap()).split(", ");
        let mut items = Queue::new();
        for item_raw in item_values {
            let item: u64 = item_raw.parse().unwrap();
            items.add(item).expect("Failed to queue item.");
        }

        // Parse operation
        let operation_raw = split_on_colon(lines.next().unwrap());
        let operation = operation_raw.parse::<Operation>().unwrap();

        // Parse divisor and targets
        let divisor: u64 = get_last_int(lines.next().unwrap());
        let target_pass: usize = get_last_int(lines.next().unwrap());
        let target_fail: usize = get_last_int(lines.next().unwrap());

        Monkey {
            id,
            items,
            operation,
            divisor,
            target_pass,
            target_fail,
            inspections: 0,
        }
    }

    fn test(&self, number: u64) -> bool {
        number % self.divisor == 0
    }

    fn get_target(&self, number: u64) -> usize {
        if self.test(number) {
            self.target_pass
        } else {
            self.target_fail
        }
    }

    fn inspect_and_throw(&mut self, worry_fn: &dyn Fn(u64) -> u64) -> Option<(usize, u64)> {
        if let Ok(item) = self.items.remove() {
            // Record inspection
            self.inspections += 1;

            // Determine new worry level
            let new_item = worry_fn(self.operation.apply(item));

            // Determine where to throw
            let target = self.get_target(new_item);

            // "Throw" the item
            return Some((target, new_item));
        }
        None
    }

    pub fn throw_items(&mut self, worry_fn: &dyn Fn(u64) -> u64) -> HashMap<usize, Vec<u64>> {
        let mut thrown_items: HashMap<usize, Vec<u64>> = HashMap::new();
        while let Some((target, item)) = self.inspect_and_throw(worry_fn) {
            thrown_items
                .entry(target)
                .and_modify(|items| items.push(item))
                .or_insert_with(|| vec![item]);
        }
        thrown_items
    }

    pub fn catch_items(&mut self, items: &[u64]) {
        for item in items.iter().copied() {
            self.items.add(item).expect("Failed to queue item.");
        }
    }
}

fn create_monkeys(data: &[Vec<String>]) -> Vec<Monkey> {
    data.iter().map(|m| Monkey::new(m)).collect()
}

fn simulate_round(monkeys: &mut [Monkey], worry_fn: &dyn Fn(u64) -> u64) {
    for i in 0..monkeys.len() {
        // Throw from first monkey
        let thrown_items = match monkeys.get_mut(i) {
            Some(monkey) => monkey.throw_items(worry_fn),
            None => continue,
        };

        if !thrown_items.is_empty() {
            for (target, items) in &thrown_items {
                if let Some(monkey) = monkeys.get_mut(*target) {
                    monkey.catch_items(items);
                }
            }
        }
    }
}

fn simulate_rounds(monkeys: &mut [Monkey], rounds: usize, worry_fn: &dyn Fn(u64) -> u64) {
    for _ in 0..rounds {
        simulate_round(monkeys, worry_fn);
    }
}

fn highest_n(monkeys: &[Monkey], n: usize) -> HashSet<u128> {
    let mut set: HashSet<u128> = HashSet::new();
    for monkey in monkeys {
        set.insert(monkey.inspections);
        if set.len() > n {
            if let Some(min) = set.iter().copied().min() {
                set.remove(&min);
            }
        }
    }
    set
}

fn resolve_monkey_business(monkeys: &[Monkey]) -> u128 {
    highest_n(monkeys, 2).iter().product()
}

// PART 1

fn solve_part1(data: &[Vec<String>], rounds: usize) -> u128 {
    let mut monkeys = create_monkeys(data);
    let worry = |n| n / 3;

    simulate_rounds(&mut monkeys, rounds, &worry);

    let inspections: Vec<u128> = monkeys.iter().map(|m| m.inspections).collect();
    println!("{:?}", inspections);

    resolve_monkey_business(&monkeys)
}

pub fn part1() {
    print::intro(11, 1);

    let data = input::day_input_grouped::<String>(11);
    let activity_test = solve_part1(&data.test, 20);
    let activity_real = solve_part1(&data.real, 20);

    print::answer_with_test(activity_real, activity_test);
}

// PART 2

fn solve_part2(data: &[Vec<String>], rounds: usize) -> u128 {
    let mut monkeys = create_monkeys(data);
    let gcd: u64 = monkeys.iter().map(|m| m.divisor).product();
    let worry = |n| n % gcd;

    simulate_rounds(&mut monkeys, rounds, &worry);
    resolve_monkey_business(&monkeys)
}

pub fn part2() {
    print::intro(11, 2);

    let data = input::day_input_grouped::<String>(11);
    let activity_test = solve_part2(&data.test, 10000);
    let activity_real = solve_part2(&data.real, 10000);

    print::answer_with_test(activity_real, activity_test);
}
