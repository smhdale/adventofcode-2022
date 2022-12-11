use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

use crate::input;
use crate::print;

/**
 * A single cleaning range, for a single elf.
 */
struct Range {
    areas: HashSet<isize>,
}

impl FromStr for Range {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-");
        let from: isize = parts.next().unwrap().parse().expect("Invalid range");
        let to: isize = parts.next().unwrap().parse().expect("Invalid range");

        let mut areas = HashSet::new();
        for i in from..=to {
            areas.insert(i);
        }
        Ok(Range { areas })
    }
    type Err = ParseError;
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        for area in &other.areas {
            if !self.areas.contains(&area) {
                return false;
            }
        }
        true
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        for area in &other.areas {
            if self.areas.contains(&area) {
                return true;
            }
        }
        false
    }
}

/**
 * A pair of ranges, for a pair of elves.
 */
struct RangePair {
    a: Range,
    b: Range,
}

impl FromStr for RangePair {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s.split(",");
        let a = ranges
            .next()
            .unwrap()
            .parse::<Range>()
            .expect("Invalid range pair");
        let b = ranges
            .next()
            .unwrap()
            .parse::<Range>()
            .expect("Invalid range pair");
        Ok(RangePair { a, b })
    }
    type Err = ParseError;
}

// PART 1

fn is_fully_contained_pair(pair: &RangePair) -> bool {
    pair.a.contains(&pair.b) || pair.b.contains(&pair.a)
}

fn count_fully_contained_pairs(pairs: &[RangePair]) -> isize {
    pairs
        .iter()
        .map(|pair| is_fully_contained_pair(pair) as isize)
        .sum()
}

pub fn part1() {
    print::intro(4, 1);

    let data = input::day_input::<RangePair>(4);

    let pairs_test = count_fully_contained_pairs(&data.test);
    let pairs_real = count_fully_contained_pairs(&data.real);

    print::answer_with_test(pairs_real, pairs_test);
}

// PART 2

fn is_overlapping_pair(pair: &RangePair) -> bool {
    pair.a.overlaps(&pair.b)
}

fn count_overlapping_pairs(pairs: &[RangePair]) -> isize {
    pairs
        .iter()
        .map(|pair| is_overlapping_pair(pair) as isize)
        .sum()
}

pub fn part2() {
    print::intro(4, 2);

    let data = input::day_input::<RangePair>(4);

    let pairs_test = count_overlapping_pairs(&data.test);
    let pairs_real = count_overlapping_pairs(&data.real);

    print::answer_with_test(pairs_real, pairs_test);
}
