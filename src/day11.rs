use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let map = Map::load("input/day11.txt");

    println!("Part 1: {}", map.clone().step_times(100));
    println!("Part 2: {}", map.clone().all_flash())
}

#[derive(Debug, Clone)]
struct Map {
    levels: Vec<Vec<i32>>
}

impl Map {
    fn load(filename: &str) -> Self {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let levels = f.lines()
            .map(|line| line.unwrap().chars().map(|c| c as i32 - '0' as i32).collect())
            .collect();

        Map { levels }
    }

    /// Simulates a step, returning the number of flashes this round.
    fn step(&mut self) -> usize {
        // Any octopus with an energy level > 9 flashes
        let mut to_spread = Vec::new();
        let mut flashed = HashSet::new();

        // Energy level of each octopus increases by 1
        for row in 0..self.levels.len() {
            for col in 0..self.levels[row].len() {
                self.levels[row][col] += 1;

                if self.levels[row][col] > 9 {
                    flashed.insert(Location::new(row, col));
                    to_spread.push(Location::new(row, col));
                }
            }
        }

        // Flashing octopi increase the level of neighbors by 1, which can trigger other flashes
        while let Some(octopus) = to_spread.pop() {
            for neighbor in self.neighbors(&octopus) {
                if !flashed.contains(&neighbor) {
                    self.levels[neighbor.row][neighbor.col] += 1;

                    if self.levels[neighbor.row][neighbor.col] > 9 {
                        flashed.insert(neighbor);
                        to_spread.push(neighbor);
                    }
                }
            }
        }

        // Any octopus that flashed has its energy level set to 0.
        for octopus in &flashed {
            self.levels[octopus.row][octopus.col] = 0;
        }

        flashed.len()
    }

    /// Simulates the given number of steps, returning the number of octopodes that flash.
    fn step_times(&mut self, steps: usize) -> usize {
        (0 .. steps).map(|_| self.step()).sum()
    }

    /// Returns the first step when all octopuses flash.
    fn all_flash(&mut self) -> usize {
        let all_count = self.levels.len() * self.levels[0].len();
        let mut step = 1;

        while self.step() < all_count {
            step += 1;
        }

        step
    }

    /// Returns a list of valid neighbors around the given location.
    fn neighbors(&self, loc: &Location) -> Vec<Location> {
        let mut neighbors = Vec::new();

        for add_row in -1 ..= 1 {
            for add_col in -1 ..= 1 {
                let valid_neighbor = (add_row != 0 || add_col != 0)
                    && (add_row > -1 || loc.row > 0)
                    && (add_col > -1 || loc.col > 0)
                    && (add_row < 1 || loc.row < self.levels.len() - 1)
                    && (add_col < 1 || loc.col < self.levels[loc.row].len() - 1);

                if valid_neighbor {
                    neighbors.push(Location::new(
                        (loc.row as i32 + add_row) as usize,
                        (loc.col as i32 + add_col) as usize,
                    ));
                }
            }
        }

        neighbors
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Location {
    row: usize,
    col: usize,
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Location {
    fn new(row: usize, col: usize) -> Self {
        Location { row, col }
    }
}

#[test]
fn test_sample() {
    let map = Map::load("input/day11_sample.txt");

    assert_eq!(1656, map.clone().step_times(100));
    assert_eq!(195, map.clone().all_flash());
}