use std::collections::VecDeque;

use crate::input;
use crate::print;

struct Header {
    size: usize,
    bytes: VecDeque<u8>,
}

impl Header {
    pub fn new(size: usize) -> Header {
        Header {
            size,
            bytes: VecDeque::new(),
        }
    }

    pub fn append(&mut self, byte: u8) {
        self.bytes.push_back(byte);
        while self.bytes.len() > self.size {
            self.bytes.pop_front();
        }
    }

    pub fn is_full(&self) -> bool {
        self.bytes.len() == self.size
    }

    pub fn unique(&self) -> bool {
        for i in 1..self.bytes.len() {
            let byte = *self.bytes.get(i).unwrap();
            for j in 0..i {
                let cmp = *self.bytes.get(j).unwrap();
                if cmp == byte {
                    return false;
                }
            }
        }
        true
    }
}

fn find_marker(datastream: &str, header_size: usize) -> Option<usize> {
    let mut header = Header::new(header_size);

    for (index, byte) in datastream.as_bytes().iter().enumerate() {
        header.append(*byte);
        if header.is_full() && header.unique() {
            return Some(index + 1);
        }
    }

    None
}

// PART 1

pub fn part1() {
    print::intro(6, 1);

    let data = input::day_input_raw(6);
    let pos_test = find_marker(&data.test, 4).unwrap();
    let pos_real = find_marker(&data.real, 4).unwrap();

    print::answer_with_test(pos_real, pos_test);
}

// PART 2

pub fn part2() {
    print::intro(6, 2);

    let data = input::day_input_raw(6);
    let pos_test = find_marker(&data.test, 14).unwrap();
    let pos_real = find_marker(&data.real, 14).unwrap();

    print::answer_with_test(pos_real, pos_test);
}
