use crate::input;
use crate::print;

type Coord = (isize, isize);
type Visibility = [bool; 4];

#[derive(Copy, Clone, Debug)]
enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

const MAX_HEIGHT: u8 = 9;
const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

fn str_to_vec_u8(s: &str) -> Vec<u8> {
    s.chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect()
}

fn resolve_delta(dir: Direction) -> (isize, isize) {
    match dir {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::East => (-1, 0),
        Direction::West => (1, 0),
    }
}

/**
 * Forest tracks tree heights and can scan for visibility from any tree in any
 * cardinal direction
 */
struct Forest {
    width: usize,
    height: usize,
    trees: Vec<u8>,
}

impl Forest {
    pub fn new(input: &[String]) -> Forest {
        // Resolve dimensions
        let width = match input.first() {
            Some(row) => row.len(),
            None => 0,
        };
        let height = input.len();

        // Create trees
        let mut trees = vec![];
        for row_raw in input {
            for height in str_to_vec_u8(row_raw) {
                trees.push(height);
            }
        }

        Forest {
            width,
            height,
            trees,
        }
    }

    fn get_bounds(&self) -> Coord {
        (
            self.width.try_into().unwrap_or(1),
            self.height.try_into().unwrap_or(1),
        )
    }

    // Converts a 2D coord to a D index, or None if out of bounds
    fn coord_to_index(&self, (x, y): Coord) -> Option<usize> {
        if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) {
            if x < self.width && y < self.height {
                return Some(self.width * y + x);
            }
        }
        None
    }

    // Converts a 1D index to a 2D coord, or None if out of bounds
    fn index_to_coord(&self, i: usize) -> Coord {
        let i = isize::try_from(i).unwrap_or(0);
        let (w, h) = self.get_bounds();
        (i % h, i / w)
    }

    // Gets the tree height at a given 2D coord, or None if out of bounds
    fn get_tree_at(&self, coord: Coord) -> Option<u8> {
        if let Some(index) = self.coord_to_index(coord) {
            return self.trees.get(index).copied();
        }
        None
    }

    // Starting at a coord and moving in a direction, counts how many trees are
    // visible, and returns that count and whether the edge was reached
    fn get_tree_visibility_dir(&self, coord: Coord, dir: Direction) -> (usize, bool) {
        let tree = self.get_tree_at(coord).expect("No tree here");

        let (dx, dy) = resolve_delta(dir);

        let mut x = coord.0 + dx;
        let mut y = coord.1 + dy;
        let mut count: usize = 0;

        while let Some(other) = self.get_tree_at((x, y)) {
            count += 1;
            if other >= tree {
                return (count, false);
            }
            x += dx;
            y += dy;
        }

        (count, true)
    }

    // Returns true if a tree at a given 2D coord is visible externally
    fn get_tree_visibility(&self, coord: Coord) -> bool {
        DIRS.iter()
            .any(|dir| self.get_tree_visibility_dir(coord, *dir).1)
    }

    // Gets the scenic score for the tree at a given 2D coord
    fn get_scenic_score(&self, coord: Coord) -> usize {
        DIRS.iter()
            .map(|dir| self.get_tree_visibility_dir(coord, *dir).0)
            .product()
    }

    // Returns an iterator through every 2D coord in the forest
    fn iter_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.trees.len()).map(|i| self.index_to_coord(i))
    }

    // Counts all trees in the forest that are visible externally
    pub fn count_visible_trees(&self) -> usize {
        self.iter_coords()
            .map(|coord| self.get_tree_visibility(coord) as usize)
            .sum()
    }

    // Finds the tree with the highest scenic score
    pub fn get_max_scenic_score(&self) -> usize {
        self.iter_coords()
            .map(|coord| self.get_scenic_score(coord))
            .max()
            .unwrap()
    }
}

// PART 1

fn count_visible_trees(input: &[String]) -> usize {
    let forest = Forest::new(input);
    forest.count_visible_trees()
}

pub fn part1() {
    print::intro(8, 1);

    let data = input::day_input(8);

    let count_test = Forest::new(&data.test).count_visible_trees();
    let count_real = Forest::new(&data.real).count_visible_trees();

    print::answer_with_test(count_real, count_test);
}

// PART 2

pub fn part2() {
    print::intro(8, 2);

    let data = input::day_input(8);

    let max_test = Forest::new(&data.test).get_max_scenic_score();
    let max_real = Forest::new(&data.real).get_max_scenic_score();

    print::answer_with_test(max_real, max_test);
}
