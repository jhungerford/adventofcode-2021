use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let map = HeightMap::load("input/day9.txt");

    println!("Part 1: {}", map.risk());
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
                if self.neighbors(row, col).iter().all(|&n| height < n) {
                    risk += height + 1;
                }
            }
        }

        risk
    }

    /// Returns the heights of the points around the given row and column.
    fn neighbors(&self, row: usize, col: usize) -> Vec<i32> {
        let mut neighbors = Vec::new();

        for (add_row, add_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let valid_neighbor = (row > 0 || add_row >= 0)
                && (col > 0 || add_col >= 0)
                && (row < self.heights.len() - 1 || add_row <= 0)
                && (col < self.heights[row].len() - 1 || add_col <= 0);

            if valid_neighbor {
                neighbors.push(self.heights[(row as i32 + add_row) as usize][(col as i32 + add_col) as usize]);
            }
        }

        neighbors
    }
}

#[test]
fn test_risk_sample() {
    let map = HeightMap::load("input/day9_sample.txt");

    assert_eq!(15, map.risk());
}