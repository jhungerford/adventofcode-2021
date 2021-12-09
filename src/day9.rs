use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let map = HeightMap::load("input/day9.txt");

    println!("Part 1: {}", map.risk());
    println!("Part 2: {}", map.basins());
}

struct HeightMap {
    heights: Vec<Vec<i32>>
}

impl HeightMap {
    /// Loads a HeightMap from the given file.
    fn load(filename: &str) -> Self {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let heights = f.lines()
            .map(|line| line.unwrap().chars().map(|c| c as i32 - '0' as i32).collect())
            .collect();

        HeightMap { heights }
    }

    /// Returns the risk of this HeightMap.  The risk level of a low point
    /// is 1 plus its height, and the risk of the map is the sum of all of
    /// the low point risks.
    fn risk(&self) -> i32 {
        let mut risk = 0;

        for row in 0..self.heights.len() {
            for col in 0..self.heights[row].len() {
                let height = self.heights[row][col];
                if self.neighbors(&Location::new(row, col)).iter()
                    .map(|loc| self.height(loc))
                    .all(|n| height < n) {
                    risk += height + 1;
                };
            }
        }

        risk
    }

    /// Returns the product of the sizes of the three largest basins on the map.
    /// A basin is a connected set of locations, walled off by 9-height squares.
    fn basins(&self) -> usize {
        let mut basin_sizes: Vec<usize> = Vec::new();
        let mut visited = HashSet::new();

        // Brute-force flood fill to find the basins.  Scan until we find a location that
        // hasn't been scanned and isn't a wall, then expand outward to explore the basin.
        for row in 0..self.heights.len() {
            for col in 0..self.heights[row].len() {
                let start_loc = Location::new(row, col);

                if !visited.contains(&start_loc) && self.height(&start_loc) < 9 {
                    // Visiting a new basin.
                    let mut to_visit = VecDeque::new();
                    to_visit.push_back(start_loc);

                    let mut basin = HashSet::new();

                    while let Some(loc) = to_visit.pop_front() {
                        basin.insert(loc);
                        visited.insert(loc);

                        self.neighbors(&loc).iter()
                            .filter(|l| !basin.contains(l) && self.height(l) < 9)
                            .for_each(|l| to_visit.push_back(*l))
                    }

                    basin_sizes.push(basin.len());
                }
            }
        }

        // Multiply the sizes of the top-3 largest basins.
        basin_sizes.iter().sorted().rev().take(3).product()
    }

    /// Returns the valid locations around the given row and column.
    fn neighbors(&self, loc: &Location) -> Vec<Location> {
        let mut neighbors = Vec::new();

        for (add_row, add_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let valid_neighbor = (loc.row > 0 || add_row >= 0)
                && (loc.col > 0 || add_col >= 0)
                && (loc.row < self.heights.len() - 1 || add_row <= 0)
                && (loc.col < self.heights[loc.row].len() - 1 || add_col <= 0);

            if valid_neighbor {
                neighbors.push(Location::new(
                    (loc.row as i32 + add_row) as usize,
                    (loc.col as i32 + add_col) as usize
                ));
            }
        }

        neighbors
    }

    /// Returns the height at the given location.
    #[inline]
    fn height(&self, location: &Location) -> i32 {
        self.heights[location.row][location.col]
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn new(row: usize, col: usize) -> Self {
        Location { row, col }
    }
}

#[test]
fn test_sample() {
    let map = HeightMap::load("input/day9_sample.txt");

    assert_eq!(15, map.risk());
    assert_eq!(1134, map.basins());
}
