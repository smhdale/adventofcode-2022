use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::Sub;
use std::str::FromStr;
use std::string::ParseError;

// use uint::construct_uint;
// use primitive_types::U1024;
use num_bigint::BigUint;

use queues::*;

use crate::input;
use crate::print;

// construct_uint! {
// 	pub struct BigUint(16);
// }

#[derive(Debug)]
enum OpValue {
    Old,
    New(BigUint),
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mult,
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
    fn interpolate(&self, op_value: &OpValue, number: &BigUint) -> BigUint {
        match op_value {
            OpValue::Old => number.clone(),
            OpValue::New(n) => n.clone(),
        }
    }

    pub fn apply(&self, number: &mut BigUint) {
        let a = self.interpolate(&self.a, number);
        let b = self.interpolate(&self.b, number);
        number.clone_from(
            &match self.op {
                Op::Add => a.add(b),
                Op::Sub => a.sub(b),
                Op::Mult => a.mul(b),
            }
        );
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
                n => {
                    let value = n.parse::<BigUint>().unwrap();
                    OpValue::New(value)
                }
            }
        }

        fn parse_op(op: &str) -> Op {
            match op {
                "*" => Op::Mult,
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
    items: Queue<BigUint>,
    operation: Operation,
    divisor: BigUint,
    target_pass: usize,
    target_fail: usize,
    inspections: usize,
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
            let item: BigUint = item_raw.parse().unwrap();
            items.add(item).expect("Failed to queue item.");
        }

        // Parse operation
        let operation_raw = split_on_colon(lines.next().unwrap());
        let operation = operation_raw.parse::<Operation>().unwrap();

        // Parse divisor and targets
        let divisor: BigUint = get_last_int(lines.next().unwrap());
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

    fn test(&self, number: &BigUint) -> bool {
        // (number % self.divisor).is_zero()
        number % &self.divisor == BigUint::from(0 as u8)
    }

    fn get_target(&self, number: &BigUint) -> usize {
        if self.test(number) {
            self.target_pass
        } else {
            self.target_fail
        }
    }

    fn inspect_and_throw(&mut self, worry_mod: &BigUint) -> Option<(usize, &BigUint)> {
        if let Ok(item) = self.items.remove() {
            // Record inspection
            self.inspections += 1;

            // Determine new worry level
            self.operation.apply(&mut item);
            item.div_assign(worry_mod);

            // Determine where to throw
            let target = self.get_target(&item);

            // "Throw" the item
            return Some((target, &item));
        }
        None
    }

    pub fn throw_items(&mut self, worry_mod: &BigUint) -> HashMap<usize, Vec<BigUint>> {
        let mut thrown_items: HashMap<usize, Vec<BigUint>> = HashMap::new();
        while let Some((target, item)) = self.inspect_and_throw(worry_mod) {
            thrown_items
                .entry(target)
                .and_modify(|items| items.push(item.clone()))
                .or_insert_with(|| vec![item.clone()]);
        }
        thrown_items
    }

    pub fn catch_items(&mut self, items: &[BigUint]) {
        for item in items {
            self.items.add(item).expect("Failed to queue item.");
        }
    }
}

fn create_monkeys(data: &[Vec<String>]) -> HashMap<usize, Monkey> {
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();
    for group in data {
        let monkey = Monkey::new(group);
        monkeys.insert(monkey.id, monkey);
    }
    monkeys
}

fn simulate_round(monkeys: &mut HashMap<usize, Monkey>, worry_mod: &BigUint) {
    let mut cursor: usize = 0;
    let size = monkeys.len();

    while cursor < size {
        let thrown_items: HashMap<usize, Vec<BigUint>>;

        // Throw from first monkey
        if let Some(monkey) = monkeys.get_mut(&cursor) {
            thrown_items = monkey.throw_items(worry_mod);
        } else {
            cursor += 1;
            continue;
        }

        if !thrown_items.is_empty() {
            for (target, items) in &thrown_items {
                if let Some(monkey) = monkeys.get_mut(target) {
                    monkey.catch_items(items);
                }
            }
        }

        cursor += 1;
    }
}

fn simulate_rounds(monkeys: &mut HashMap<usize, Monkey>, rounds: usize, worry_mod: &BigUint) {
    for _ in 0..rounds {
        simulate_round(monkeys, worry_mod);
    }
}

fn highest_n(monkeys: &HashMap<usize, Monkey>, n: usize) -> HashSet<usize> {
    let mut set: HashSet<usize> = HashSet::new();
    for monkey in monkeys.values() {
        set.insert(monkey.inspections);
        if set.len() > n {
            if let Some(min) = set.iter().copied().min() {
                set.remove(&min);
            }
        }
    }
    set
}

fn resolve_monkey_business(data: &[Vec<String>], rounds: usize, worry_mod: usize) -> BigUint {
    let mut monkeys = create_monkeys(data);
    simulate_rounds(&mut monkeys, rounds, &BigUint::from(worry_mod));

    let activities: Vec<usize> = monkeys.values().map(|m| m.inspections).collect();
    println!("{:?}", activities);

    let most_active = highest_n(&monkeys, 2);

    let mut res: BigUint = BigUint::from(1 as u8);
    for n in most_active.into_iter() {
        res *= BigUint::from(n);
    }
    res
}

// PART 1

pub fn part1() {
    print::intro(11, 1);

    let data = input::day_input_grouped::<String>(11);
    let activity_test = resolve_monkey_business(&data.test, 20, 3);
    let activity_real = resolve_monkey_business(&data.real, 20, 3);

    print::answer_with_test(activity_real, activity_test);
}

// PART 2

pub fn part2() {
    print::intro(11, 2);

    let data = input::day_input_grouped::<String>(11);
    let activity_test = resolve_monkey_business(&data.test, 10000, 1);
    let activity_real = resolve_monkey_business(&data.real, 10000, 1);

    print::answer_with_test(activity_real, activity_test);
}
