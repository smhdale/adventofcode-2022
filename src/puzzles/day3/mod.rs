use std::collections::hash_set::Iter;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

use crate::input;
use crate::print;

const BASE_PRIORITY_LOWER: usize = 'a' as usize;
const BASE_PRIORITY_UPPER: usize = 'A' as usize;

#[derive(PartialEq, Eq, Hash)]
enum Compartment {
    Left,
    Right,
}

// Gets the priority of a packed item represented by a char
fn get_item_priority(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - BASE_PRIORITY_LOWER + 1,
        'A'..='Z' => item as usize - BASE_PRIORITY_UPPER + 27,
        _ => 0,
    }
}

struct RucksackCompartment {
    _set: HashSet<char>,
    items: Vec<char>,
}

impl RucksackCompartment {
    fn new() -> RucksackCompartment {
        RucksackCompartment {
            _set: HashSet::new(),
            items: vec![],
        }
    }

    fn add_item(&mut self, item: char) {
        self._set.insert(item);
        self.items.push(item);
    }
}

struct Rucksack {
    _set: HashSet<char>,
    compartments: HashMap<Compartment, RucksackCompartment>,
}

impl FromStr for Rucksack {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let items = s.chars();

        let mut sack = Rucksack::new();
        for (index, item) in items.enumerate() {
            let target = if index < len / 2 {
                Compartment::Left
            } else {
                Compartment::Right
            };
            sack.add_to_compartment(target, item);
        }

        Ok(sack)
    }
    type Err = ParseError;
}

impl Rucksack {
    // Creates a new, empty rucksack
    fn new() -> Rucksack {
        Rucksack {
            _set: HashSet::new(),
            compartments: HashMap::new(),
        }
    }

    // Adds an item to a compartment of the rucksack
    fn add_to_compartment(&mut self, compartment: Compartment, item: char) {
        self._set.insert(item);
        self.compartments
            .entry(compartment)
            .and_modify(|e| e.add_item(item))
            .or_insert({
                let mut c = RucksackCompartment::new();
                c.add_item(item);
                c
            });
    }

    // Checks if an item exists in a specific compartment of the rucksack
    fn exists_in_compartment(&self, compartment: Compartment, item: &char) -> bool {
        match self.compartments.get(&compartment) {
            Some(c) => c._set.contains(item),
            None => false,
        }
    }

    // Checks if an item exists in the rucksack
    fn exists(&self, item: &char) -> bool {
        return self._set.contains(item);
    }

    // Allows iterating all items in the rucksack
    fn items(&self) -> Iter<char> {
        self._set.iter()
    }
}

// PART 1
// Find and sum values of items within each sack that are in both compartments

fn find_duplicate_item(sack: &Rucksack) -> Option<char> {
    for item in sack.items() {
        if sack.exists_in_compartment(Compartment::Left, item)
            && sack.exists_in_compartment(Compartment::Right, item)
        {
            return Some(*item);
        }
    }
    None
}

fn sum_duplicate_item_priorities(sacks: &Vec<Rucksack>) -> usize {
    sacks
        .iter()
        .map(|s| find_duplicate_item(&s).map_or(0, get_item_priority))
        .sum()
}

pub fn part1() {
    print::intro(3, 1);

    let sacks = input::day_input::<Rucksack>(3);
    let sum_test = sum_duplicate_item_priorities(&sacks.test);
    let sum_real = sum_duplicate_item_priorities(&sacks.real);

    print::answer_with_test(sum_real, sum_test);
}

// PART 2
// Find the shared item in each group of three rucksacks

fn find_badge(group: &[Rucksack]) -> Option<char> {
    match group.split_first() {
        Some((first, rest)) => {
            'item: for item in first.items() {
                for other in rest {
                    if !other.exists(item) {
                        continue 'item;
                    }
                }
                return Some(*item);
            }
            None
        }
        None => None,
    }
}

fn sum_badge_priorities(sacks: &Vec<Rucksack>) -> usize {
    sacks
        .chunks(3)
        .map(|group| find_badge(group).map_or(0, get_item_priority))
        .sum()
}

pub fn part2() {
    print::intro(3, 2);

    let sacks = input::day_input::<Rucksack>(3);
    let sum_test = sum_badge_priorities(&sacks.test);
    let sum_real = sum_badge_priorities(&sacks.real);

    print::answer_with_test(sum_real, sum_test);
}
