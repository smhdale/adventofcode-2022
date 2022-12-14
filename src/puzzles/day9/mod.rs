use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

use crate::input;
use crate::print;

type Coord = (isize, isize);

struct Movement {
    delta: Coord,
    repeat: usize,
}

impl FromStr for Movement {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let dir = parts.next().unwrap();
        let repeat: usize = parts.next().unwrap().parse().unwrap();
        let delta: Coord = match dir {
            // Up
            "U" => (0, -1),
            // Down
            "D" => (0, 1),
            // Left
            "L" => (-1, 0),
            // Right (catchall)
            _ => (1, 0),
        };
        Ok(Movement { delta, repeat })
    }
    type Err = ParseError;
}

struct Rope {
    head: Coord,
    knots: Vec<Coord>,
    track_tail: HashSet<Coord>,
}

impl Rope {
    pub fn new(length: usize) -> Rope {
        Rope {
            head: (0, 0),
            knots: vec![(0, 0); length],
            track_tail: HashSet::new(),
        }
    }

    fn move_head(&mut self, (dx, dy): &Coord) {
        self.head.0 += dx;
        self.head.1 += dy;
    }

    fn move_knot(&mut self, index: usize) {
        let count = self.knots.len();

        // Determine previous knot
        let prev_knot = if index == 0 {
            self.head
        } else {
            *self.knots.get(index - 1).unwrap()
        };

        if let Some(knot) = self.knots.get_mut(index) {
            // Move knot towards prev knot
            let dx = prev_knot.0 - knot.0;
            let dy = prev_knot.1 - knot.1;

            if dx.abs() > 1 || dy.abs() > 1 {
                knot.0 += dx.signum();
                knot.1 += dy.signum();
            }

            // If index matches last knot, track it
            if index == count - 1 {
                self.track_tail.insert(*knot);
            }
        }
    }

    fn move_knots(&mut self) {
        let knots = self.knots.len();
        for i in 0..knots {
            self.move_knot(i);
        }
    }

    fn apply_movement(&mut self, movement: &Movement) {
        for _ in 0..movement.repeat {
            self.move_head(&movement.delta);
            self.move_knots();
        }
    }

    pub fn apply_movements(&mut self, movements: &[Movement]) {
        for m in movements {
            self.apply_movement(m)
        }
    }
}

fn simulate_rope(length: usize, movements: &[Movement]) -> usize {
    let mut rope = Rope::new(length);
    rope.apply_movements(movements);
    rope.track_tail.len()
}

// PART 1
// Moving a rope with 1 knot around and reporting area covered by its tail

pub fn part1() {
    print::intro(9, 1);

    let data = input::day_input::<Movement>(9);
    let area_test = simulate_rope(1, &data.test);
    let area_real = simulate_rope(1, &data.real);

    print::answer_with_test(area_real, area_test);
}

// PART 2
// This time, the rope has 9 knots

pub fn part2() {
    print::intro(9, 2);

    let data = input::day_input::<Movement>(9);
    let area_test = simulate_rope(9, &data.test);
    let area_real = simulate_rope(9, &data.real);

    print::answer_with_test(area_real, area_test);
}
